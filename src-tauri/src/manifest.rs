use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub version: String,
    pub description: String,
    pub lessons: Vec<LessonRef>,
}

#[derive(Debug, Deserialize)]
pub struct LessonRef {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CourseMigrations {
    #[serde(rename = "migration", default)]
    pub migrations: Vec<VersionMigration>,
}

impl CourseMigrations {
    pub fn find_for_version(&self, from_version: &str) -> Option<&VersionMigration> {
        self.migrations.iter().find(|m| m.from_version == from_version)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VersionMigration {
    pub from_version: String,

    #[serde(default, rename = "lessons")]
    pub lesson_id_map: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Lesson {
    pub title: String,
    pub panels: Vec<Panel>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Left,
    Right,
    Full,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Panel {
    Markdown {
        side: Side,
        path: String,
    },
    CodeExercise {
        side: Side,
        prompt: String,
        language: String,
        starter_path: String,
        validation: Validation,
        controls: Vec<String>,

        #[serde(default = "default_true")]
        required: bool,
    },
    Quiz {
        side: Side,
        question: String,
        kind: QuizKind,
        options: Vec<String>,
        answer: Vec<String>,

        #[serde(default = "default_true")]
        required: bool,
    },
}

impl Panel {
    pub fn is_required(&self) -> bool {
        match self {
            Panel::Markdown { .. } => false,
            Panel::CodeExercise { required, .. } => *required,
            Panel::Quiz { required, .. } => *required,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuizKind {
    SingleChoice,
    MultiChoice,
    FreeText,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Validation {
    StdoutMatch {
        expected_path: String,

        #[serde(default)]
        trim: bool,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum CourseError {
    #[error("io error reading {path}: {source}")]
    IO {
        path: PathBuf,

        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse toml at {path}: {source}")]
    TOML {
        path: PathBuf,

        #[source]
        source: toml::de::Error,
    },
    #[error("lesson id '{0}' not found in course manifest")]
    LessonNotFound(String),
    #[error("duplicate lesson id '{0}' in course manifest")]
    DuplicateLessonId(String),
}

fn default_true() -> bool {
    true
}