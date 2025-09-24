use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: Option<i64>,
    pub description: String,
    pub timestamp: DateTime<Local>,
    pub date: NaiveDate,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum Theme {
    #[default]
    GruvboxDark,
    GruvboxLight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppPage {
    Viewer,
    Settings,
}
