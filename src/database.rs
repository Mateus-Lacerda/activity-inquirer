use anyhow::{Context, Result};
use chrono::{DateTime, Local, NaiveDate};
use rusqlite::{params, Connection, Row};
use std::path::PathBuf;

use crate::models::Activity;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        
        // Criar diretório se não existir
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Falha ao criar diretório do banco de dados")?;
        }

        let conn = Connection::open(&db_path)
            .context("Falha ao abrir conexão com o banco de dados")?;

        let db = Database { conn };
        db.initialize_tables()?;
        Ok(db)
    }

    fn get_db_path() -> Result<PathBuf> {
        let config_dir = if cfg!(target_os = "windows") {
            // Windows: %APPDATA%\activity-inquirer
            dirs::config_dir()
                .context("Não foi possível encontrar o diretório de configuração no Windows")?
        } else if cfg!(target_os = "macos") {
            // macOS: ~/Library/Application Support/activity-inquirer
            dirs::config_dir()
                .context("Não foi possível encontrar o diretório de configuração no macOS")?
        } else {
            // Linux/Unix: ~/.config/activity-inquirer
            dirs::config_dir()
                .context("Não foi possível encontrar o diretório de configuração no Linux")?
        };

        Ok(config_dir.join("activity-inquirer").join("activities.db"))
    }

    fn initialize_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS activities (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                date TEXT NOT NULL
            )",
            [],
        ).context("Falha ao criar tabela de atividades")?;

        Ok(())
    }

    pub fn add_activity(&self, description: String) -> Result<()> {
        let now = Local::now();
        let date = now.date_naive();

        self.conn.execute(
            "INSERT INTO activities (description, timestamp, date) VALUES (?1, ?2, ?3)",
            params![description, now.to_rfc3339(), date.to_string()],
        ).context("Falha ao inserir atividade")?;

        Ok(())
    }

    pub fn get_activities_for_date(&self, date: NaiveDate) -> Result<Vec<Activity>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, description, timestamp, date FROM activities WHERE date = ?1 ORDER BY timestamp"
        ).context("Falha ao preparar consulta")?;

        let activity_iter = stmt.query_map([date.to_string()], |row| {
            self.row_to_activity(row)
        }).context("Falha ao executar consulta")?;

        let mut activities = Vec::new();
        for activity in activity_iter {
            println!("Atividade: {:?}", activity);
            activities.push(activity?);
        }

        Ok(activities)
    }

    pub fn get_last_activity_today(&self) -> Result<Option<Activity>> {
        let today = Local::now().date_naive();
        
        let mut stmt = self.conn.prepare(
            "SELECT id, description, timestamp, date FROM activities WHERE date = ?1 ORDER BY timestamp DESC LIMIT 1"
        ).context("Falha ao preparar consulta")?;

        let mut activity_iter = stmt.query_map([today.to_string()], |row| {
            self.row_to_activity(row)
        }).context("Falha ao executar consulta")?;

        if let Some(activity) = activity_iter.next() {
            Ok(Some(activity?))
        } else {
            Ok(None)
        }
    }

    pub fn count_activities_today(&self) -> Result<i64> {
        let today = Local::now().date_naive();
        
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM activities WHERE date = ?1",
            [today.to_string()],
            |row| row.get(0)
        ).context("Falha ao contar atividades de hoje")?;

        Ok(count)
    }

    fn row_to_activity(&self, row: &Row) -> rusqlite::Result<Activity> {
        let timestamp_str: String = row.get(2)?;
        let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|_e| rusqlite::Error::InvalidColumnType(2, "timestamp".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Local);

        let date_str: String = row.get(3)?;
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_e| rusqlite::Error::InvalidColumnType(3, "date".to_string(), rusqlite::types::Type::Text))?;

        Ok(Activity {
            id: Some(row.get(0)?),
            description: row.get(1)?,
            timestamp,
            date,
        })
    }
}
