use crate::database::Database;
use crate::fonts;
use crate::models::Activity;
use anyhow::Result;
use egui::{Button, CentralPanel, Context, RichText, TextEdit};

pub struct InquiryApp {
    db: Database,
    current_input: String,
    question_text: String,
    is_first_question: bool,
    last_activity: Option<Activity>,
    message: Option<String>,
    should_close: bool,
    fonts_configured: bool,
    close_requested: bool,
    close_timer: Option<std::time::Instant>,
}

impl InquiryApp {
    pub fn new() -> Result<Self> {
        let db = Database::new()?;
        let mut app = InquiryApp {
            db,
            current_input: String::new(),
            question_text: String::new(),
            is_first_question: true,
            last_activity: None,
            message: None,
            should_close: false,
            fonts_configured: false,
            close_requested: false,
            close_timer: None,
        };

        app.setup_question()?;
        Ok(app)
    }

    fn setup_question(&mut self) -> Result<()> {
        let count = self.db.count_activities_today()?;

        if count == 0 {
            self.is_first_question = true;
            self.question_text = "O que você está fazendo agora?".to_string();
        } else {
            self.is_first_question = false;
            if let Some(last_activity) = self.db.get_last_activity_today()? {
                self.question_text =
                    format!("Você ainda está fazendo \"{}\"?", last_activity.description);
                self.last_activity = Some(last_activity);
            } else {
                self.question_text = "O que você está fazendo agora?".to_string();
                self.is_first_question = true;
            }
        }

        Ok(())
    }

    fn save_activity(&mut self) -> Result<()> {
        if self.current_input.trim().is_empty() {
            self.message = Some("Por favor, digite uma atividade.".to_string());
            return Ok(());
        }

        self.db
            .add_activity(self.current_input.trim().to_string())?;
        self.message = Some("Atividade salva com sucesso!".to_string());
        self.current_input.clear();

        // Fechar a aplicação após salvar
        self.should_close = true;

        Ok(())
    }

    fn handle_yes_no_response(&mut self, is_yes: bool) -> Result<()> {
        if is_yes {
            // Se ainda está fazendo a mesma atividade, salva novamente
            if let Some(ref last_activity) = self.last_activity {
                self.db.add_activity(last_activity.description.clone())?;
                self.message = Some("Atividade continuada registrada!".to_string());
            }
        } else {
            // Se não está mais fazendo, pergunta o que está fazendo agora
            self.question_text = "O que você está fazendo agora?".to_string();
            self.is_first_question = true;
            return Ok(());
        }

        self.should_close = true;
        Ok(())
    }
}

impl eframe::App for InquiryApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Configurar fontes na primeira execução
        if !self.fonts_configured {
            fonts::setup_fonts(ctx);
            self.fonts_configured = true;
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);

                // Título
                ui.label(RichText::new("Activity Inquirer").size(24.0).strong());
                ui.add_space(30.0);

                // Pergunta
                ui.label(RichText::new(&self.question_text).size(16.0));
                ui.add_space(20.0);

                if self.is_first_question {
                    // Campo de texto para nova atividade
                    ui.horizontal(|ui| {
                        ui.label("Atividade:");
                        let response = ui.add(
                            TextEdit::singleline(&mut self.current_input).desired_width(300.0),
                        );

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            if let Err(e) = self.save_activity() {
                                self.message = Some(format!("Erro ao salvar: {e}"));
                            }
                        }
                    });

                    ui.add_space(20.0);

                    if ui.add(Button::new("Salvar")).clicked() {
                        if let Err(e) = self.save_activity() {
                            self.message = Some(format!("Erro ao salvar: {e}"));
                        }
                    }
                } else {
                    // Botões Sim/Não para atividade anterior
                    ui.horizontal(|ui| {
                        if ui.add(Button::new("Sim")).clicked() {
                            if let Err(e) = self.handle_yes_no_response(true) {
                                self.message = Some(format!("Erro: {e}"));
                            }
                        }

                        if ui.add(Button::new("Não")).clicked() {
                            if let Err(e) = self.handle_yes_no_response(false) {
                                self.message = Some(format!("Erro: {e}"));
                            }
                        }
                    });
                }

                ui.add_space(20.0);

                // Mensagem de status
                if let Some(ref message) = self.message {
                    ui.label(RichText::new(message).color(egui::Color32::GREEN));
                }

                ui.add_space(20.0);

                if ui.add(Button::new("Cancelar")).clicked() {
                    self.should_close = true;
                    self.close_requested = true;
                    self.close_timer = Some(std::time::Instant::now());
                }
            });
        });

        // Fechar a aplicação se necessário
        if self.should_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);

            // Se o fechamento foi solicitado, forçar repaint para garantir que o comando seja processado
            if self.close_requested {
                ctx.request_repaint();

                // Se passou mais de 2 segundos tentando fechar, forçar saída
                if let Some(timer) = self.close_timer {
                    if timer.elapsed().as_secs() > 2 {
                        eprintln!("⚠️  Forçando fechamento da aplicação após timeout");
                        std::process::exit(0);
                    }
                }
            }
        }
    }
}
