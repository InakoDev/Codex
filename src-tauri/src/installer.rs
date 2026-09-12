use crate::entities::{course, CourseEntity};
use crate::manifest::{Course, CourseError, CourseMigrations};
use crate::progress;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum InstallerError {
    #[error("course error: {0}")]
    Course(#[from] CourseError),

    #[error("IO error at {path}: {source}")]
    IO {
        path: PathBuf,

        #[source]
        source: std::io::Error,
    },

    #[error("bundled courses directory not found at {0}")]
    MissingResourceDirectory(PathBuf),

    #[error("database error: {0}")]
    DB(#[from] sea_orm::DbErr),
}

#[derive(Debug, Clone, Serialize)]
pub struct CourseStatus {
    pub id: String,
    pub title: String,
    pub resource_version: String,
    pub installed_version: String,
    pub needs_update: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReconcileResult {
    pub id: String,
    pub title: String,
    pub version: String,
    pub changed: bool,
}

fn io_err(path: &Path, source: std::io::Error) -> InstallerError {
    InstallerError::IO {
        path: path.to_path_buf(),
        source,
    }
}

fn remove_dir_if_exists(path: &Path) -> Result<(), InstallerError> {
    match std::fs::remove_dir_all(path) {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(io_err(path, err)),
    }
}

fn read_course_manifest(course_dir: &Path) -> Result<Course, InstallerError> {
    let manifest_path = course_dir.join("manifest.toml");
    let raw = std::fs::read_to_string(&manifest_path).map_err(|err| io_err(&manifest_path, err))?;

    let course: Course = toml::from_str(&raw).map_err(|source| {
        InstallerError::Course(CourseError::TOML {
            path: manifest_path.clone(),
            source,
        })
    })?;

    Ok(course)
}

pub fn read_course_migrations(course_dir: &Path) -> Result<Option<CourseMigrations>, InstallerError> {
    let migrations_path = course_dir.join("migrations.toml");

    if !migrations_path.exists() {
        return Ok(None);
    }

    let raw = std::fs::read_to_string(&migrations_path).map_err(|err| io_err(&migrations_path, err))?;

    let migrations: CourseMigrations = toml::from_str(&raw).map_err(|source| {
        InstallerError::Course(CourseError::TOML {
            path: migrations_path.clone(),
            source,
        })
    })?;

    Ok(Some(migrations))
}

fn list_course_dirs(courses_root: &Path) -> Result<Vec<PathBuf>, InstallerError> {
    if !courses_root.is_dir() {
        return Ok(vec![]);
    }

    let mut dirs = vec![];

    for entry in std::fs::read_dir(courses_root).map_err(|err| io_err(courses_root, err))? {
        let entry = entry.map_err(|err| io_err(courses_root, err))?;
        let path = entry.path();

        if path.is_dir() && path.join("manifest.toml").is_file() {
            dirs.push(path);
        }
    }

    Ok(dirs)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), InstallerError> {
    std::fs::create_dir_all(dst).map_err(|err| io_err(dst, err))?;

    for entry in std::fs::read_dir(src).map_err(|err| io_err(src, err))? {
        let entry = entry.map_err(|err| io_err(src, err))?;
        let file_type = entry.file_type().map_err(|err| io_err(&entry.path(), err))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path).map_err(|err| io_err(&src_path, err))?;
        }
    }

    Ok(())
}

pub async fn ensure_all_installed(
    db: &DatabaseConnection,
    resource_dir: &Path,
    app_data_dir: &Path,
) -> Result<Vec<ReconcileResult>, InstallerError> {
    let resource_root: PathBuf = resource_courses_dir(resource_dir);

    if !resource_root.is_dir() {
        return Err(InstallerError::MissingResourceDirectory(resource_root));
    }

    let mut results = Vec::new();

    for course_dir in list_course_dirs(&resource_root)? {
        let course: Course = read_course_manifest(&course_dir)?;
        let result = reconcile_course(db, resource_dir, app_data_dir, &course.id).await?;

        results.push(result);
    }

    Ok(results)
}

pub fn install_course(resource_dir: &Path, app_data_dir: &Path, course_id: &str) -> Result<Course, InstallerError> {
    let src: PathBuf = resource_courses_dir(resource_dir).join(course_id);

    if !src.is_dir() {
        return Err(InstallerError::MissingResourceDirectory(src));
    }

    let dst: PathBuf = data_courses_dir(app_data_dir).join(course_id);
    remove_dir_if_exists(&dst)?;
    copy_dir_recursive(&src, &dst)?;

    read_course_manifest(&dst)
}

pub async fn reconcile_course(
    db: &DatabaseConnection,
    resource_dir: &Path,
    app_data_dir: &Path,
    course_id: &str,
) -> Result<ReconcileResult, InstallerError> {
    let bundled = read_course_manifest(&resource_courses_dir(resource_dir).join(course_id))?;

    let installed_dir = data_courses_dir(app_data_dir).join(course_id);
    let folder_present = installed_dir.join("manifest.toml").is_file();

    let previous_record = CourseEntity::find_by_id(course_id.to_string()).one(db).await?;
    let needs_reconcile = !folder_present;

    if !needs_reconcile {
        return Ok(ReconcileResult {
            id: bundled.id,
            title: bundled.title,
            version: bundled.version,
            changed: false,
        });
    }

    // TODO: Make code re-usable, it's same as `commands::update_course`.
    let previous_version = previous_record.as_ref().map(|record| record.version.clone());

    let migrations = read_course_migrations(&resource_courses_dir(resource_dir).join(course_id))?;

    let course = install_course(resource_dir, app_data_dir, course_id)?;

    // Migrate course.
    if let Some(old_version) = previous_version {
        let new_lesson_ids: Vec<String> = course.lessons.iter().map(|lesson| lesson.id.clone()).collect();
        let migration = migrations
            .as_ref()
            .and_then(|migration| migration.find_for_version(&old_version));

        progress::migrate_course_progress(db, &course.id, migration, &new_lesson_ids).await?;
    }

    // Updating course status.
    let now = Utc::now();

    match previous_record {
        Some(record) => {
            let mut active: course::ActiveModel = record.into();
            active.title = Set(course.title.clone());
            active.version = Set(course.version.clone());
            active.updated_at = Set(now);
            active.update(db).await?;
        }
        None => {
            let active = course::ActiveModel {
                id: Set(course.id.clone()),
                title: Set(course.title.clone()),
                version: Set(course.version.clone()),
                installed_at: Set(now),
                updated_at: Set(now),
            };

            active.insert(db).await?;
        }
    }

    Ok(ReconcileResult {
        id: course.id,
        title: course.title,
        version: course.version,
        changed: true,
    })
}

pub fn list_bundled_courses(resource_dir: &Path) -> Result<Vec<Course>, InstallerError> {
    let resource_root: PathBuf = resource_courses_dir(resource_dir);

    list_course_dirs(&resource_root)?
        .iter()
        .map(|dir| read_course_manifest(dir))
        .collect()
}

pub fn list_installed_courses(app_data_dir: &Path) -> Result<Vec<Course>, InstallerError> {
    let data_root: PathBuf = data_courses_dir(app_data_dir);

    list_course_dirs(&data_root)?
        .iter()
        .map(|dir| read_course_manifest(dir))
        .collect()
}

pub fn resource_courses_dir(resource_dir: &Path) -> PathBuf {
    resource_dir.join("courses")
}

pub fn data_courses_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("courses")
}
