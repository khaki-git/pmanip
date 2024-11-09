use fltk::enums::*;

pub const PRIMARY_COLOUR: (u8, u8, u8) = (68, 179, 235);
pub const BACKGROUND_COLOUR: (u8, u8, u8) = (32, 41, 46);
pub const BACKGROUND_ALTERNATIVE: (u8, u8, u8) = (54, 68, 77);
pub const BACKGROUND_ALTERNATIVE_2: (u8, u8, u8) = (44, 58, 67);

pub enum Colours {
    Primary,
    Background,
    BackgroundAlt,
    BackgroundAlt2
}

pub const VERSION: usize = 2024;
pub const DROPDOWN_MENU_PADDING: i32 = 10;

pub fn get_colour(colour_id: Colours) -> Color {
    match colour_id {
        Colours::Primary => Color::from_rgb(PRIMARY_COLOUR.0, PRIMARY_COLOUR.1, PRIMARY_COLOUR.2),
        Colours::Background => Color::from_rgb(BACKGROUND_COLOUR.0, BACKGROUND_COLOUR.1, BACKGROUND_COLOUR.2),
        Colours::BackgroundAlt => Color::from_rgb(BACKGROUND_ALTERNATIVE.0, BACKGROUND_ALTERNATIVE.1, BACKGROUND_ALTERNATIVE.2),
        Colours::BackgroundAlt2 => Color::from_rgb(BACKGROUND_ALTERNATIVE_2.0, BACKGROUND_ALTERNATIVE_2.1, BACKGROUND_ALTERNATIVE_2.2),
    }
}

pub fn get_title(postfix: String) -> String {
    format!("PManip {} - {}", VERSION, postfix)
}