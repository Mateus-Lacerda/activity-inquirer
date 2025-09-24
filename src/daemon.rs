use anyhow::Result;
use std::time::Duration;
use tokio::time;
use chrono::Local;

use crate::config::AppConfig;
use crate::run_inquiry_mode;

pub async fn run_daemon() -> Result<()> {
    // Carregar configuração
    let config = AppConfig::load()?;
    println!("🤖 Iniciando modo daemon - Activity Inquirer");
    println!("⏰ Intervalo configurado: {}", config.format_interval());
    println!("💡 Pressione Ctrl+C para parar o daemon");
    println!();

    // Executar inquérito imediatamente ao iniciar
    println!("📝 Executando primeiro inquérito... ({})", Local::now().format("%H:%M:%S"));
    if let Err(e) = run_inquiry_mode() {
        eprintln!("❌ Erro no inquérito inicial: {e}");
    } else {
        println!("✅ Primeiro inquérito concluído!");
    }
    println!();

    // Configurar timer para executar no intervalo especificado
    let interval_seconds = config.get_daemon_interval_seconds();
    let mut interval = time::interval(Duration::from_secs(interval_seconds));
    
    // Pular o primeiro tick (já executamos o inquérito inicial)
    interval.tick().await;
    
    let mut inquiry_count = 1;
    
    loop {
        // Aguardar próximo tick
        interval.tick().await;
        
        inquiry_count += 1;
        let now = Local::now();
        
        println!("⏰ Hora do inquérito #{} ({})", inquiry_count, now.format("%H:%M:%S"));
        
        // Executar inquérito
        match run_inquiry_mode() {
            Ok(_) => {
                println!("✅ Inquérito #{inquiry_count} concluído com sucesso!");
            }
            Err(e) => {
                eprintln!("❌ Erro no inquérito #{inquiry_count}: {e}");
                eprintln!("🔄 Continuando execução...");
            }
        }
        
        // Calcular próximo inquérito
        let next_inquiry = now + chrono::Duration::minutes(config.daemon_interval_minutes as i64);
        println!("📅 Próximo inquérito em: {}", next_inquiry.format("%H:%M:%S"));
        println!();
    }
}

pub fn print_daemon_info() {
    println!("📊 Informações do Daemon:");
    println!("  • Intervalo padrão: 60 minutos (1 hora)");
    println!("  • Primeiro inquérito: Imediato");
    println!("  • Inquéritos subsequentes: A cada hora");
    println!("  • Persistência: Salva no banco SQLite");
    println!("  • Localização do banco: ~/.config/activity-inquirer/");
    println!();
}
