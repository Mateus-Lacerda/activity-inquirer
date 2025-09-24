use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub daemon_interval_minutes: u64,
    pub theme: String,
    pub auto_start_daemon: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            daemon_interval_minutes: 60, // 1 hora por padrão
            theme: "GruvboxDark".to_string(),
            auto_start_daemon: false,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path).with_context(|| {
                format!("Falha ao ler arquivo de configuração: {config_path:?}")
            })?;

            let config: AppConfig = toml::from_str(&content)
                .with_context(|| "Falha ao parsear arquivo de configuração")?;

            Ok(config)
        } else {
            // Criar configuração padrão se não existir
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        // Criar diretório se não existir
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Falha ao criar diretório de configuração: {parent:?}"))?;
        }

        let content =
            toml::to_string_pretty(self).with_context(|| "Falha ao serializar configuração")?;

        fs::write(&config_path, content)
            .with_context(|| format!("Falha ao salvar arquivo de configuração: {config_path:?}"))?;

        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let config_dir = if cfg!(target_os = "windows") {
            dirs::config_dir()
                .context("Não foi possível encontrar o diretório de configuração no Windows")?
        } else if cfg!(target_os = "macos") {
            dirs::config_dir()
                .context("Não foi possível encontrar o diretório de configuração no macOS")?
        } else {
            dirs::config_dir()
                .context("Não foi possível encontrar o diretório de configuração no Linux")?
        };

        Ok(config_dir.join("activity-inquirer").join("config.toml"))
    }

    pub fn get_daemon_interval_seconds(&self) -> u64 {
        self.daemon_interval_minutes * 60
    }

    pub fn get_available_intervals() -> Vec<(String, u64)> {
        vec![
            ("1 minuto".to_string(), 1),
            ("5 minutos".to_string(), 5),
            ("10 minutos".to_string(), 10),
            ("15 minutos".to_string(), 15),
            ("30 minutos".to_string(), 30),
            ("1 hora".to_string(), 60),
            ("2 horas".to_string(), 120),
            ("4 horas".to_string(), 240),
            ("8 horas".to_string(), 480),
        ]
    }

    pub fn format_interval(&self) -> String {
        Self::format_interval_static(self.daemon_interval_minutes)
    }

    pub fn format_interval_static(minutes: u64) -> String {
        if minutes < 60 {
            format!("{} minuto{}", minutes, if minutes == 1 { "" } else { "s" })
        } else {
            let hours = minutes / 60;
            let remaining_minutes = minutes % 60;
            if remaining_minutes == 0 {
                format!("{} hora{}", hours, if hours == 1 { "" } else { "s" })
            } else {
                format!("{hours}h{remaining_minutes}m")
            }
        }
    }
}
