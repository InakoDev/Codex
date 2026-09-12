use crate::entities::lesson_progress::{self, Entity as LessonProgressEntity};
use crate::manifest::VersionMigration;
use chrono::{DateTime, Utc};
use gtk::gdk::keys::constants::d;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct OverviewStats {
    pub courses_started: u64,
    pub lessons_completed: u64,
    pub days_learning: u64,
}

#[derive(Debug, Serialize)]
pub struct CourseProgress {
    pub course_id: String,
    pub total_lessons: u64,
    pub lessons_completed: u64,
    pub percent_complete: f64,
}

#[derive(Debug, Serialize)]
pub struct CalendarDay {
    pub date: String,
    pub lessons_completed: u64,
}

pub async fn migrate_course_progress(
    db: &DatabaseConnection,
    course_id: &str,
    migration: Option<&VersionMigration>,
    new_lesson_ids: &[String],
) -> Result<(), DbErr> {
    let old_rows = LessonProgressEntity::find()
        .filter(lesson_progress::Column::CourseId.eq(course_id))
        .all(db)
        .await?;

    if old_rows.is_empty() {
        return Ok(()); // Nothing to migrate.
    }

    let mut remapped: HashMap<String, DateTime<Utc>> = HashMap::new();

    for row in &old_rows {
        let new_id = match migration {
            Some(migration) => migration.lesson_id_map.get(&row.lesson_id).cloned(),
            None => {
                if new_lesson_ids.contains(&row.lesson_id) {
                    Some(row.lesson_id.clone())
                } else {
                    None
                }
            }
        };

        let Some(new_id) = new_id else { continue };

        // Guardrail to prevent cases where a migration file might point to a Lesson ID that doesn't exist.
        if !new_lesson_ids.contains(&new_id) {
            println!("[warning]: '{new_id}' doesn't exist in lesson ids.");
            continue;
        }

        remapped
            .entry(new_id)
            .and_modify(|existing| {
                if row.completed_at < *existing {
                    *existing = row.completed_at;
                }
            })
            .or_insert(row.completed_at);
    }

    LessonProgressEntity::delete_many()
        .filter(lesson_progress::Column::CourseId.eq(course_id))
        .exec(db)
        .await?;

    for (lesson_id, completed_at) in remapped {
        let active = lesson_progress::ActiveModel {
            course_id: Set(course_id.to_string()),
            lesson_id: Set(lesson_id),
            completed_at: Set(completed_at),
        };

        active.insert(db).await?;
    }

    Ok(())
}
