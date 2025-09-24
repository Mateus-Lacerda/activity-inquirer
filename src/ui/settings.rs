use anyhow::Result;
use egui::{CentralPanel, Context, ComboBox, RichText, Button};
use crate::config::AppConfig;
use crate::models::Theme;
use crate::ui::theme;

pub struct SettingsApp {
    config: AppConfig,
    current_theme: Theme,
    message: Option<String>,
    selected_interval: u64,
}

impl SettingsApp {
    pub fn new() -> Result<Self> {
        let config = AppConfig::load()?;
        let current_theme = match config.theme.as_str() {
            "GruvboxLight" => Theme::GruvboxLight,
            _ => Theme::GruvboxDark,
        };
        
        Ok(SettingsApp {
            selected_interval: config.daemon_interval_minutes,
            config,
            current_theme,
            message: None,
        })
    }

    fn save_config(&mut self) -> Result<()> {
        self.config.daemon_interval_minutes = self.selected_interval;
        self.config.theme = match self.current_theme {
            Theme::GruvboxDark => "GruvboxDark".to_string(),
            Theme::GruvboxLight => "GruvboxLight".to_string(),
        };
        
        self.config.save()?;
        self.message = Some(" Configurações salvas com sucesso!".to_string());
        Ok(())
    }

    fn change_theme(&mut self, ctx: &Context, new_theme: Theme) {
        self.current_theme = new_theme;
        theme::apply_theme(ctx, new_theme);
    }
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Aplicar tema
        theme::apply_theme(ctx, self.current_theme);
        
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // Cabeçalho
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Activity Inquirer - Configurações").size(20.0).strong());
                });
                
                ui.separator();
                ui.add_space(20.0);
                
                // Mensagem de status
                if let Some(ref message) = self.message {
                    ui.label(RichText::new(message).color(
                        if message.starts_with("") {
                            egui::Color32::from_rgb(0, 200, 0)
                        } else {
                            egui::Color32::RED
                        }
                    ));
                    ui.add_space(10.0);
                }
                
                // Configurações do Daemon
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(" Configurações do Daemon").size(16.0).strong());
                        ui.add_space(10.0);
                        
                        ui.horizontal(|ui| {
                            ui.label("Intervalo entre inquéritos:");
                            
                            ComboBox::from_id_source("interval_combo")
                                .selected_text(format!("{}", AppConfig::format_interval_static(self.selected_interval)))
                                .show_ui(ui, |ui| {
                                    for (label, minutes) in AppConfig::get_available_intervals() {
                                        ui.selectable_value(&mut self.selected_interval, minutes, label);
                                    }
                                });
                        });
                        
                        ui.add_space(5.0);
                        ui.label(format!("⏰ Atual: {}", AppConfig::format_interval_static(self.config.daemon_interval_minutes)));
                        
                        if self.selected_interval != self.config.daemon_interval_minutes {
                            ui.add_space(5.0);
                            ui.label(RichText::new(format!("🔄 Novo: {}", AppConfig::format_interval_static(self.selected_interval)))
                                .color(egui::Color32::from_rgb(255, 165, 0)));
                        }
                    });
                });
                
                ui.add_space(20.0);
                
                // Configurações de Tema
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(" Configurações de Tema").size(16.0).strong());
                        ui.add_space(10.0);
                        
                        ui.horizontal(|ui| {
                            ui.label("Tema:");
                            
                            ComboBox::from_label("")
                                .selected_text(match self.current_theme {
                                    Theme::GruvboxDark => "Gruvbox Dark",
                                    Theme::GruvboxLight => "Gruvbox Light",
                                })
                                .show_ui(ui, |ui| {
                                    if ui.selectable_value(&mut self.current_theme, Theme::GruvboxDark, "Gruvbox Dark").clicked() {
                                        self.change_theme(ctx, Theme::GruvboxDark);
                                    }
                                    if ui.selectable_value(&mut self.current_theme, Theme::GruvboxLight, "Gruvbox Light").clicked() {
                                        self.change_theme(ctx, Theme::GruvboxLight);
                                    }
                                });
                        });
                    });
                });
                
                ui.add_space(30.0);
                
                // Botões de ação
                ui.horizontal(|ui| {
                    if ui.add(Button::new("󰆓 Salvar Configurações").min_size([150.0, 30.0].into())).clicked() {
                        if let Err(e) = self.save_config() {
                            self.message = Some(format!("❌ Erro ao salvar: {}", e));
                        }
                    }
                    
                    ui.add_space(10.0);
                    
                    if ui.add(Button::new("🔄 Restaurar Padrões").min_size([150.0, 30.0].into())).clicked() {
                        let default_config = AppConfig::default();
                        self.selected_interval = default_config.daemon_interval_minutes;
                        self.current_theme = Theme::GruvboxDark;
                        self.change_theme(ctx, Theme::GruvboxDark);
                        self.message = Some(" Configurações restauradas para os padrões".to_string());
                    }
                });
                
                ui.add_space(20.0);
                
                // Informações adicionais
                ui.separator();
                ui.add_space(10.0);
                
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("ℹ️ Informações").size(14.0).strong());
                        ui.add_space(5.0);
                        
                        ui.label("• As configurações são salvas automaticamente");
                        ui.label("• O daemon precisa ser reiniciado para aplicar novos intervalos");
                        ui.label("• Intervalo mínimo: 1 minuto");
                        ui.label("• Configurações ficam em: ~/.config/activity-inquirer/config.toml");
                    });
                });
            });
        });
    }
}
