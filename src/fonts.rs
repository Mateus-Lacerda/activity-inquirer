use egui::{Context, FontData, FontDefinitions, FontFamily};

// Embarcando as fontes Nerd Font no binário
const FIRA_CODE_NERD_REGULAR: &[u8] = include_bytes!("../assets/fonts/FiraCode-Regular.ttf");
const FIRA_CODE_NERD_BOLD: &[u8] = include_bytes!("../assets/fonts/FiraCode-Bold.ttf");

pub fn setup_fonts(ctx: &Context) {
    println!("Configurando fontes Nerd Font...");
    let mut fonts = FontDefinitions::default();

    // Adicionar FiraCode Nerd Font Regular
    fonts.font_data.insert(
        "FiraCodeNerdFont-Regular".to_owned(),
        FontData::from_static(FIRA_CODE_NERD_REGULAR),
    );

    // Adicionar FiraCode Nerd Font Bold
    fonts.font_data.insert(
        "FiraCodeNerdFont-Bold".to_owned(),
        FontData::from_static(FIRA_CODE_NERD_BOLD),
    );

    // Configurar FiraCode Nerd Font como fonte principal para texto proporcional
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "FiraCodeNerdFont-Regular".to_owned());

    // Configurar FiraCode Nerd Font como fonte principal para texto monoespaçado
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "FiraCodeNerdFont-Regular".to_owned());

    // Manter fontes de fallback do sistema para glifos não suportados
    // (egui já inclui algumas fontes de fallback por padrão)

    // Aplicar as configurações de fonte
    ctx.set_fonts(fonts);
}

