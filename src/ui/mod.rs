use bevy::app::plugin_group;

pub mod bar;
pub mod game_over;
pub mod gui;
pub mod hud;
pub mod main_menu;
pub mod player_health_bar;

plugin_group! {
    pub struct UiPlugins{
        bar:::BarPlugin,
        game_over:::GameOverPlugin,
        gui:::GuiPlugin,
        hud:::HudPlugin,
        main_menu:::MainMenuPlugin,
    }
}
