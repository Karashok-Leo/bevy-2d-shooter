use bevy::app::plugin_group;

pub mod bar;
pub mod damage_popup;
pub mod debug_panel;
pub mod game_over;
pub mod hud;
pub mod main_menu;
pub mod pause;
pub mod player_health_bar;
pub mod popup;
pub mod util;

plugin_group! {
    pub struct UIPlugins{
        util:::UIUtilPlugin,
        popup:::PopupPlugin,
        bar:::BarPlugin,
        hud:::HudPlugin,
        main_menu:::MainMenuPlugin,
        pause:::PausePlugin,
        game_over:::GameOverPlugin,
    }
}
