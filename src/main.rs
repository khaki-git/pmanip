mod config;
mod hero_screen;
mod membuild;
mod editor;
mod tint;

use fltk::{app, prelude::*, enums::*, *};

fn main() {
    let app = app::App::default();
    Font::set_font(Font::Helvetica, &Font::load_font("assets/UbuntuMedium.ttf").unwrap());
    let background_colour = config::get_colour(config::Colours::Background);

    app::set_frame_type2(FrameType::UpBox, FrameType::RFlatBox);
    app::set_frame_type2(FrameType::DownBox, FrameType::RFlatBox);
    app::set_background_color(background_colour.to_rgb().0, background_colour.to_rgb().1, background_colour.to_rgb().2);
    app::set_frame_border_radius_max(5);
    app::set_foreground_color(255, 255, 255);

    hero_screen::Hero::default();
    app.run().unwrap();
}