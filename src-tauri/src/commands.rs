use crate::entities::course::Model;
use crate::entities::{course, CourseEntity};
use crate::installer::{self, CourseStatus};
use crate::manifest::Course;
use sea_orm::sea_query::prelude::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};

pub struct AppState {
    pub db: DatabaseConnection,
}

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|err| format!("failed to resolve app data dir: {err}"))
}

fn resource_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .resource_dir()
        .map_err(|err| format!("failed to resolve resource dir: {err}"))
}

#[tauri::command]
pub async fn update_course(
    app: AppHandle,
    state: State<'_, AppState>,
    course_id: String,
) -> Result<CourseStatus, String> {
    let resource_dir: PathBuf = resource_dir(&app)?;
    let data_dir: PathBuf = app_data_dir(&app)?;

    let previous_record: Option<Model> = CourseEntity::find_by_id(course_id.clone())
        .one(&state.db)
        .await
        .map_err(|err| err.to_string())?;

    let previous_version: Option<String> = previous_record.as_ref().map(|record| record.version.clone());

    let migrations =
        installer::read_course_migrations(&installer::resource_courses_dir(&resource_dir).join(&course_id))
            .map_err(|err| err.to_string())?;

    let course: Course =
        installer::install_course(&resource_dir, &data_dir, &course_id).map_err(|err| err.to_string())?;

    // TODO: Migrate old course progress tracking to match new course.
    //     // Migrating course progress
    //     if let Some(old_version) = previous_version {
    //         let new_lesson_ids: Vec<String> = course.lessons.iter().map(|lesson| lesson.id.clone()).collect();
    //         let migration = migrations.as_ref().and_then(|migration| migration.find_for_version(&old_version));
    //     }

    // Updating course status.
    let now = Utc::now();

    match previous_record {
        Some(record) => {
            let mut active: course::ActiveModel = record.into();
            active.title = Set(course.title.clone());
            active.version = Set(course.version.clone());
            active.updated_at = Set(now);
            active.update(&state.db).await.map_err(|err| err.to_string())?;
        }
        None => {
            let active = course::ActiveModel {
                id: Set(course.id.clone()),
                title: Set(course.title.clone()),
                version: Set(course.version.clone()),
                installed_at: Set(now),
                updated_at: Set(now),
            };

            active.insert(&state.db).await.map_err(|e| e.to_string())?;
        }
    }

    Ok(CourseStatus {
        id: course.id,
        title: course.title,
        resource_version: course.version.clone(),
        installed_version: course.version,
        needs_update: false,
    })
}
