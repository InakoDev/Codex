use sea_orm::DatabaseConnection;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};

use crate::installer::{self, CourseStatus};
use crate::manifest::Course;

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
    let resource_dir = resource_dir(&app)?;
    let data_dir = app_data_dir(&app)?;

    let course: Course = installer::install_course(&resource_dir, &data_dir, &course_id)
        .map_err(|err| err.to_string())?;

    // TODO: Update database using Sea-ORM.

    Ok(CourseStatus {
        id: course.id,
        title: course.title,
        resource_version: course.version.clone(),
        installed_version: course.version,
        needs_update: false,
    })
}
