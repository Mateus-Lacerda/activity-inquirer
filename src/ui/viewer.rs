use anyhow::Result;
use chrono::{Local, NaiveDate};
use egui::{CentralPanel, Context, ComboBox, ScrollArea, RichText};
use crate::config::AppConfig;
use crate::database::Database;
use crate::fonts;
use crate::models::{Activity, Theme, AppPage};
use crate::ui::theme;

pub struct ViewerApp {
    db: Database,
    selected_date: NaiveDate,
    activities: Vec<Activity>,
    current_theme: Theme,
    message: Option<String>,
    current_page: AppPage,
    // Configurações
    config: AppConfig,
    selected_interval: u64,
    // Controle de fontes
    fonts_configured: bool,
}

impl ViewerApp {
    pub fn new() -> Result<Self> {
        let db = Database::new()?;
        let selected_date = Local::now().date_naive();
        
        let config = AppConfig::load().unwrap_or_default();
        let current_theme = match config.theme.as_str() {
            "GruvboxLight" => Theme::GruvboxLight,
            _ => Theme::GruvboxDark,
        };
        
        let mut app = ViewerApp {
            db,
            selected_date,
            activities: Vec::new(),
            current_theme,
            message: None,
            current_page: AppPage::Viewer,
            selected_interval: config.daemon_interval_minutes,
            config,
            fonts_configured: false,
        };
        
        app.load_activities()?;
        Ok(app)
    }

    fn load_activities(&mut self) -> Result<()> {
        match self.db.get_activities_for_date(self.selected_date) {
            Ok(activities) => {
                self.activities = activities;
                self.message = None;
            }
            Err(e) => {
                self.message = Some(format!("Erro ao carregar atividades: {e}"));
                self.activities.clear();
            }
        }
        Ok(())
    }

    fn format_time(&self, activity: &Activity) -> String {
        activity.timestamp.format("%H:%M").to_string()
    }

    fn change_theme(&mut self, ctx: &Context, new_theme: Theme) {
        self.current_theme = new_theme;
        theme::apply_theme(ctx, new_theme);
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
}

impl eframe::App for ViewerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Configurar fontes na primeira execução
        if !self.fonts_configured {
            fonts::setup_fonts(ctx);
            self.fonts_configured = true;
        }

        // Aplicar tema
        theme::apply_theme(ctx, self.current_theme);
        
        CentralPanel::default().show(ctx, |ui| {
            // Barra de navegação
            ui.horizontal(|ui| {
                if ui.selectable_label(self.current_page == AppPage::Viewer, "Visualizador").clicked() {
                    self.current_page = AppPage::Viewer;
                }
                
                if ui.selectable_label(self.current_page == AppPage::Settings, "Configurações").clicked() {
                    self.current_page = AppPage::Settings;
                }
            });
            
            ui.separator();
            
            // Renderizar conteúdo baseado na página atual
            match self.current_page {
                AppPage::Viewer => self.render_viewer_content(ctx, ui),
                AppPage::Settings => self.render_settings_content(ctx, ui),
            }
        });
    }
}

impl ViewerApp {
    fn render_viewer_content(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Cabeçalho
            ui.horizontal(|ui| {
                ui.label(RichText::new("Visualizador de Atividades").size(18.0).strong());
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Seletor de tema
                    ComboBox::from_label("Tema")
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
            
            ui.separator();
            ui.add_space(10.0);
            
            // Seletor de data
            ui.horizontal(|ui| {
                ui.label("Data:");
                
                let mut date_string = self.selected_date.format("%Y-%m-%d").to_string();
                if ui.text_edit_singleline(&mut date_string).changed() {
                    if let Ok(new_date) = NaiveDate::parse_from_str(&date_string, "%Y-%m-%d") {
                        self.selected_date = new_date;
                        if let Err(e) = self.load_activities() {
                            self.message = Some(format!("Erro ao carregar atividades: {e}"));
                        }
                    }
                }
                
                if ui.button("Hoje").clicked() {
                    self.selected_date = Local::now().date_naive();
                    if let Err(e) = self.load_activities() {
                        self.message = Some(format!("Erro ao carregar atividades: {e}"));
                    }
                }
                
                if ui.button("Ontem").clicked() {
                    self.selected_date = Local::now().date_naive() - chrono::Duration::days(1);
                    if let Err(e) = self.load_activities() {
                        self.message = Some(format!("Erro ao carregar atividades: {e}"));
                    }
                }
            });
            
            ui.add_space(10.0);
            
            // Mensagem de erro/status
            if let Some(ref message) = self.message {
                let color = if message.starts_with("") {
                    egui::Color32::from_rgb(0, 200, 0) // Verde para informações
                } else if message.starts_with("") {
                    egui::Color32::from_rgb(255, 165, 0) // Laranja para avisos
                } else {
                    egui::Color32::RED // Vermelho para erros
                };
                ui.label(RichText::new(message).color(color));
                ui.add_space(10.0);
            }
            
            // Lista de atividades
            ui.label(RichText::new(format!("Atividades para {}", self.selected_date.format("%d/%m/%Y"))).size(16.0).strong());
            ui.add_space(10.0);
            
            if self.activities.is_empty() {
                ui.label("Nenhuma atividade registrada para este dia.");
            } else {
                ScrollArea::vertical().show(ui, |ui| {
                    for (index, activity) in self.activities.iter().enumerate() {
                        // Criar um grupo visual para cada atividade
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                // Horário em destaque
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(self.format_time(activity))
                                        .monospace()
                                        .strong()
                                        .size(14.0)
                                        .color(egui::Color32::from_rgb(251, 241, 199))); // Gruvbox fg
                                    
                                    // Mostrar data se for diferente de hoje
                                    if activity.timestamp.date_naive() != Local::now().date_naive() {
                                        ui.label(RichText::new(activity.timestamp.format("%d/%m").to_string())
                                            .size(10.0)
                                            .color(egui::Color32::GRAY));
                                    }
                                });
                                
                                ui.add_space(15.0);
                                
                                // Descrição da atividade
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(&activity.description).size(13.0));
                                    
                                    // Mostrar tempo desde a atividade anterior (se houver)
                                    if index > 0 {
                                        let prev_activity = &self.activities[index - 1];
                                        let duration = activity.timestamp.signed_duration_since(prev_activity.timestamp);
                                        
                                        if let Ok(duration_std) = duration.to_std() {
                                            let minutes = duration_std.as_secs() / 60;
                                            if minutes > 0 {
                                                let time_diff = if minutes < 60 {
                                                    format!("{minutes}min depois")
                                                } else {
                                                    let hours = minutes / 60;
                                                    let remaining_minutes = minutes % 60;
                                                    if remaining_minutes == 0 {
                                                        format!("{hours}h depois")
                                                    } else {
                                                        format!("{hours}h{remaining_minutes}m depois")
                                                    }
                                                };
                                                println!("Tempo desde a última atividade: {time_diff}");
                                                let time_label = format!("󱑆 {time_diff}");
                                                println!("time_label: {time_label}");
                                                ui.label(RichText::new(time_label)
                                                    .size(10.0)
                                                    .color(egui::Color32::GRAY));
                                            }
                                        }
                                    }
                                });
                            });
                        });
                        
                        ui.add_space(8.0);
                    }
                });
            }
            
            ui.add_space(20.0);
            
            // Estatísticas básicas
            if !self.activities.is_empty() {
                ui.separator();
                ui.add_space(10.0);
                ui.label(RichText::new("Resumo").size(14.0).strong());
                ui.label(format!("Total de registros: {}", self.activities.len()));
                
                if let (Some(first), Some(last)) = (self.activities.first(), self.activities.last()) {
                    ui.label(format!("Primeiro registro: {}", self.format_time(first)));
                    ui.label(format!("Último registro: {}", self.format_time(last)));
                }
            }
        });
    }
    
    fn render_settings_content(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Cabeçalho das configurações
            ui.horizontal(|ui| {
                ui.label(RichText::new("Configurações").size(18.0).strong());
            });
            
            ui.add_space(20.0);
            
            // Mensagem de status
            if let Some(ref message) = self.message {
                let color = if message.starts_with("") {
                    egui::Color32::from_rgb(0, 200, 0)
                } else if message.starts_with("") {
                    egui::Color32::from_rgb(255, 165, 0)
                } else {
                    egui::Color32::RED
                };
                ui.label(RichText::new(message).color(color));
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
                            .selected_text(AppConfig::format_interval_static(self.selected_interval).to_string())
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
                        ui.label(RichText::new(format!("󰎔 Novo: {}", AppConfig::format_interval_static(self.selected_interval)))
                            .color(egui::Color32::from_rgb(255, 165, 0)));
                    }
                });
            });
            
            ui.add_space(20.0);
            
            // Botões de ação
            ui.horizontal(|ui| {
                if ui.button("💾 Salvar Configurações").clicked() {
                    if let Err(e) = self.save_config() {
                        self.message = Some(format!("❌ Erro ao salvar: {e}"));
                    }
                }
                
                ui.add_space(10.0);
                
                if ui.button("🔄 Restaurar Padrões").clicked() {
                    let default_config = AppConfig::default();
                    self.selected_interval = default_config.daemon_interval_minutes;
                    self.current_theme = Theme::GruvboxDark;
                    self.change_theme(ctx, Theme::GruvboxDark);
                    self.message = Some(" Configurações restauradas para os padrões".to_string());
                }
            });
        });
    }
}
