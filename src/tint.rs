use fltk::{prelude::*, *, enums::*};
use crate::config;
use ::image;
use fltk::dialog::ColorMode;
use crate::membuild;
// Fully qualifying the `image` crate path.

pub struct Tint {
    pub img: ::image::RgbImage, // Use a fully qualified path to resolve ambiguity
}

pub fn invert_color(col: Color) -> Color {
    let (r, g, b) = col.to_rgb();
    let new_col = Color::from_rgb(255 - r, 255 - g, 255 - b);
    new_col
}

impl Tint {
    pub fn new(image: ::image::RgbImage) -> Self { // Again, use a fully qualified path here
        let mut win = window::Window::default()
            .with_label(config::get_title("Configure Tint".to_string()).as_str())
            .with_size(500, 300);
        win.make_resizable(true);

        let win_ref_1 = membuild::build_ref(win.clone());
        let win_ref_2 = win_ref_1.clone();
        let win_ref_3 = win_ref_1.clone();

        let mut tint_color = Color::White;

        let mut strength_slider = valuator::HorNiceSlider::default()
            .with_label("Tint Strength (0%)");
        strength_slider.set_minimum(0f64);
        strength_slider.set_maximum(100f64);
        strength_slider.set_selection_color(config::get_colour(config::Colours::Primary));
        strength_slider.set_slider_frame(FrameType::FlatBox);
        strength_slider.set_frame(FrameType::FlatBox);

        let mut change_color = button::Button::default();
        change_color.set_label("Change Colour");
        change_color.set_color(tint_color);
        change_color.set_label_color(invert_color(tint_color.clone()));

        let mut confirm = button::Button::default()
            .with_label("Confirm");
        confirm.set_color(config::get_colour(config::Colours::Primary));

        strength_slider.set_callback(move |slider| {
            println!("Strength Slider Dragged");
            slider.set_label(format!("Tint Strength ({}%)", slider.value().floor()).as_str());
            win_ref_3.borrow_mut().redraw();
        });

        change_color.set_callback(move |button| {
            let color_dialog = dialog::color_chooser("Adjust Tint Color...", ColorMode::Rgb);
            match color_dialog {
                Some(col) => {
                    tint_color = Color::from_rgb(col.0, col.1, col.2);
                    button.set_color(tint_color);
                    button.set_label_color(invert_color(tint_color));
                    win_ref_1.borrow_mut().redraw();
                }
                None => {
                    println!("No colour selected");
                }
            }
        });

        win.handle(move |win, ev| {
            let (w, h) = (win.width(), win.height());
            match ev {
                Event::Resize => {
                    strength_slider.resize(10, h/3-10, w-20, 20);
                    change_color.resize(w/2-(w-20)/(3*2), h/3+40, (w-20)/3, 50);
                    confirm.resize(w/2-(w-20)/(3*2), h/3+100, (w-20)/3, 50);
                    true
                }
                _ => false
            }
        });

        win_ref_2.borrow_mut().end();
        win_ref_2.borrow_mut().show();

        while win_ref_2.borrow_mut().shown() {
            app::wait();
        }

        Tint {
            img: image,
        }
    }
}
