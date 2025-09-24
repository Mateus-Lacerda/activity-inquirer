use anyhow::Result;
use chrono::Local;
use std::time::Duration;
use tokio::time;

use crate::config::AppConfig;

// Função para executar inquérito usando processo separado
async fn run_inquiry_safe() -> Result<()> {
    use tokio::process::Command;
    use std::env;

    // Obter o caminho do executável atual
    let current_exe = env::current_exe()
        .map_err(|e| anyhow::anyhow!("Não foi possível obter caminho do executável: {e}"))?;

    println!("🚀 Iniciando inquérito em processo separado...");

    // Executar o inquérito em um processo separado com timeout
    let mut child = Command::new(&current_exe)
        .arg("--inquiry")
        .spawn()
        .map_err(|e| anyhow::anyhow!("Erro ao iniciar processo de inquérito: {e}"))?;

    // Aguardar com timeout de 5 minutos
    let timeout_duration = Duration::from_secs(300);

    match tokio::time::timeout(timeout_duration, child.wait()).await {
        Ok(Ok(status)) => {
            if status.success() {
                println!("✅ Inquérito concluído com sucesso!");
                Ok(())
            } else {
                eprintln!("⚠️  Inquérito terminou com código: {}", status.code().unwrap_or(-1));
                Ok(()) // Não falhar o daemon
            }
        }
        Ok(Err(e)) => {
            eprintln!("⚠️  Erro ao aguardar processo de inquérito: {e}");
            Ok(()) // Não falhar o daemon
        }
        Err(_) => {
            eprintln!("⚠️  Timeout no inquérito (5 minutos). Terminando processo...");
            let _ = child.kill().await;
            Ok(()) // Não falhar o daemon por timeout
        }
    }
}

pub async fn run_daemon() -> Result<()> {
    // Carregar configuração
    let config = AppConfig::load()?;
    println!("🤖 Iniciando modo daemon - Activity Inquirer");
    println!("⏰ Intervalo configurado: {}", config.format_interval());
    println!("💡 Pressione Ctrl+C para parar o daemon");
    println!();

    // Executar inquérito imediatamente ao iniciar
    println!(
        "📝 Executando primeiro inquérito... ({})",
        Local::now().format("%H:%M:%S")
    );
    if let Err(e) = run_inquiry_safe().await {
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

        println!(
            "⏰ Hora do inquérito #{} ({})",
            inquiry_count,
            now.format("%H:%M:%S")
        );

        // Executar inquérito
        match run_inquiry_safe().await {
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
        println!(
            "📅 Próximo inquérito em: {}",
            next_inquiry.format("%H:%M:%S")
        );
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
