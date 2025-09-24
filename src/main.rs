mod config;
mod daemon;
mod database;
mod fonts;
mod models;
mod ui;

use anyhow::Result;
use clap::{Arg, Command};
use eframe::egui;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("acv-inq")
        .about("Activity Inquirer - Rastreador de atividades pessoais")
        .version("1.0.0")
        .arg(
            Arg::new("inquiry")
                .long("inquiry")
                .help("Modo de inquérito - pergunta sobre atividade atual")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("daemon")
                .long("daemon")
                .help("Modo daemon - executa inquéritos automaticamente a cada hora")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("settings")
                .long("settings")
                .help("Abrir tela de configurações")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let is_inquiry_mode = matches.get_flag("inquiry");
    let is_daemon_mode = matches.get_flag("daemon");
    let is_settings_mode = matches.get_flag("settings");

    if is_daemon_mode {
        run_daemon_mode().await
    } else if is_inquiry_mode {
        run_inquiry_mode()
    } else if is_settings_mode {
        run_settings_mode()
    } else {
        run_viewer_mode()
    }
}

fn run_inquiry_mode() -> Result<()> {
    let app = ui::InquiryApp::new()?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_title("Activity Inquirer")
            .with_resizable(false)
            .with_close_button(true),
        // .with_always_on_top(),
        ..Default::default()
    };

    let result = eframe::run_native(
        "Activity Inquirer",
        options,
        Box::new(|cc| {
            ui::theme::apply_theme(&cc.egui_ctx, models::Theme::default());
            Box::new(app)
        }),
    );

    // Garantir que a aplicação sempre retorne, mesmo em caso de erro
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Aviso: Erro ao fechar aplicação: {e}");
            Ok(()) // Não falhar o daemon por problemas de fechamento
        }
    }
}

fn run_viewer_mode() -> Result<()> {
    let app = ui::ViewerApp::new()?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Activity Inquirer - Visualizador")
            .with_min_inner_size([600.0, 400.0])
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "Activity Inquirer - Visualizador",
        options,
        Box::new(|cc| {
            ui::theme::apply_theme(&cc.egui_ctx, models::Theme::default());
            Box::new(app)
        }),
    )
    .map_err(|e| anyhow::anyhow!("Erro ao executar aplicação: {e}"))?;

    Ok(())
}

fn run_settings_mode() -> Result<()> {
    let app = ui::SettingsApp::new()?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_title("Activity Inquirer - Configurações")
            .with_min_inner_size([500.0, 400.0])
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "Activity Inquirer - Configurações",
        options,
        Box::new(|cc| {
            ui::theme::apply_theme(&cc.egui_ctx, models::Theme::default());
            Box::new(app)
        }),
    )
    .map_err(|e| anyhow::anyhow!("Erro ao executar aplicação: {e}"))?;

    Ok(())
}

async fn run_daemon_mode() -> Result<()> {
    daemon::print_daemon_info();

    daemon::run_daemon().await
}
