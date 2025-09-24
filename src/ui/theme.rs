use crate::models::Theme;
use egui::{Color32, Visuals};

pub fn apply_theme(ctx: &egui::Context, theme: Theme) {
    match theme {
        Theme::GruvboxDark => apply_gruvbox_dark(ctx),
        Theme::GruvboxLight => apply_gruvbox_light(ctx),
    }
}

fn apply_gruvbox_dark(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();

    // Cores do Gruvbox Dark
    let bg0 = Color32::from_rgb(40, 40, 40); // #282828
    let bg1 = Color32::from_rgb(60, 56, 54); // #3c3836
    let bg2 = Color32::from_rgb(80, 73, 69); // #504945
    let fg0 = Color32::from_rgb(251, 241, 199); // #fbf1c7
    let fg1 = Color32::from_rgb(235, 219, 178); // #ebdbb2
    // let red = Color32::from_rgb(251, 73, 52);     // #fb4934
    // let green = Color32::from_rgb(184, 187, 38);  // #b8bb26
    // let yellow = Color32::from_rgb(250, 189, 47); // #fabd2f
    let blue = Color32::from_rgb(131, 165, 152); // #83a598
    // let purple = Color32::from_rgb(211, 134, 155); // #5c1b2dff
    // let aqua = Color32::from_rgb(142, 192, 124);  // #8ec07c
    let orange = Color32::from_rgb(254, 128, 25); // #fe8019

    visuals.widgets.noninteractive.bg_fill = bg0;
    visuals.widgets.noninteractive.fg_stroke.color = fg1;

    visuals.widgets.inactive.bg_fill = bg1;
    visuals.widgets.inactive.fg_stroke.color = fg1;

    visuals.widgets.hovered.bg_fill = bg2;
    visuals.widgets.hovered.fg_stroke.color = fg0;

    visuals.widgets.active.bg_fill = orange;
    visuals.widgets.active.fg_stroke.color = bg0;

    visuals.selection.bg_fill = blue;
    visuals.selection.stroke.color = fg0;

    visuals.panel_fill = bg0;
    visuals.window_fill = bg0;
    visuals.extreme_bg_color = bg1;

    ctx.set_visuals(visuals);
}

fn apply_gruvbox_light(ctx: &egui::Context) {
    let mut visuals = Visuals::light();

    // Cores do Gruvbox Light
    let bg0 = Color32::from_rgb(251, 241, 199); // #fbf1c7
    let bg1 = Color32::from_rgb(235, 219, 178); // #ebdbb2
    let bg2 = Color32::from_rgb(213, 196, 161); // #d5c4a1
    let fg0 = Color32::from_rgb(40, 40, 40); // #282828
    let fg1 = Color32::from_rgb(60, 56, 54); // #3c3836
    // let red = Color32::from_rgb(157, 0, 6);       // #9d0006
    // let green = Color32::from_rgb(121, 116, 14);  // #79740e
    // let yellow = Color32::from_rgb(181, 118, 20); // #b57614
    let blue = Color32::from_rgb(7, 102, 120); // #076678
    // let purple = Color32::from_rgb(143, 63, 113); // #8f3f71
    // let aqua = Color32::from_rgb(66, 123, 88);    // #427b58
    let orange = Color32::from_rgb(175, 58, 3); // #af3a03

    visuals.widgets.noninteractive.bg_fill = bg0;
    visuals.widgets.noninteractive.fg_stroke.color = fg1;

    visuals.widgets.inactive.bg_fill = bg1;
    visuals.widgets.inactive.fg_stroke.color = fg1;

    visuals.widgets.hovered.bg_fill = bg2;
    visuals.widgets.hovered.fg_stroke.color = fg0;

    visuals.widgets.active.bg_fill = orange;
    visuals.widgets.active.fg_stroke.color = bg0;

    visuals.selection.bg_fill = blue;
    visuals.selection.stroke.color = bg0;

    visuals.panel_fill = bg0;
    visuals.window_fill = bg0;
    visuals.extreme_bg_color = bg1;

    ctx.set_visuals(visuals);
}
