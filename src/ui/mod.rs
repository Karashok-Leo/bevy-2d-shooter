use bevy::app::plugin_group;

pub mod bar;
mod debug_panel;
pub mod game_over;
pub mod hud;
pub mod main_menu;
pub mod player_health_bar;
mod popup;
pub mod util;
mod damage_popup;

plugin_group! {
    pub struct UiPlugins{
        util:::UIUtilPlugin,
        popup:::PopupPlugin,
        bar:::BarPlugin,
        hud:::HudPlugin,
        main_menu:::MainMenuPlugin,
        game_over:::GameOverPlugin,
    }
}
