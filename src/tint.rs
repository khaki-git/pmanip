use fltk::{prelude::*, *, enums::*};
use crate::config;
use ::image;
use fltk::dialog::ColorMode;
use image::{ImageBuffer, Pixel, Rgba};
use crate::membuild;
// Fully qualifying the `image` crate path.

pub struct Tint {
    pub img: ::image::ImageBuffer<Rgba<u8>, Vec<u8>>, // Use a fully qualified path to resolve ambiguity
}

pub fn invert_color(col: Color) -> Color {
    let (r, g, b) = col.to_rgb();
    let new_col = Color::from_rgb(255 - r, 255 - g, 255 - b);
    new_col
}

fn clamp(x: i32, y: i32, z: i32) -> i32 {
    if x < y {
        y
    } else if x > z {
        z
    } else {
        x
    }
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}

impl Tint {
    pub fn new(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Self { // Again, use a fully qualified path here
        let mut win = window::Window::default()
            .with_label(config::get_title("Configure Tint".to_string()).as_str())
            .with_size(500, 300);
        win.make_resizable(true);

        let win_ref_1 = membuild::build_ref(win.clone());
        let win_ref_2 = win_ref_1.clone();
        let win_ref_3 = win_ref_1.clone();
        let win_ref_4 = win_ref_1.clone();

        let img_ref_1 = membuild::build_ref(image);
        let img_ref_2 = img_ref_1.clone();

        let mut tint_color = Color::White;
        let tint_color_ref = membuild::build_ref(tint_color);
        let tint_color_ref_2 = tint_color_ref.clone();

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
        change_color.set_label_color(Color::Black);

        let strength_ref = membuild::build_ref(strength_slider);
        let strength_ref_2 = strength_ref.clone();
        let change_color_ref = membuild::build_ref(change_color);
        let change_color_ref_2 = change_color_ref.clone();


        let mut confirm = button::Button::default()
            .with_label("Confirm");
        confirm.set_color(config::get_colour(config::Colours::Primary));

        let confirm_reference = membuild::build_ref(confirm);
        let confirm_ref_2 = confirm_reference.clone();

        strength_ref.borrow_mut().set_callback(move |slider| {
            println!("Strength Slider Dragged");
            slider.set_label(format!("Tint Strength ({}%)", slider.value().floor()).as_str());
            win_ref_3.borrow_mut().redraw();
        });

        change_color_ref.borrow_mut().set_callback(move |button| {
            let color_dialog = dialog::color_chooser("Adjust Tint Color...", ColorMode::Rgb);
            match color_dialog {
                Some(col) => {
                    *tint_color_ref.borrow_mut() = Color::from_rgb(col.0, col.1, col.2);
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
                    strength_ref.borrow_mut().resize(10, h/3-10, w-20, 20);
                    change_color_ref.borrow_mut().resize(w/2-(w-20)/(3*2), h/3+40, (w-20)/3, 50);
                    confirm_ref_2.borrow_mut().resize(w/2-(w-20)/(3*2), h/3+100, (w-20)/3, 50);
                    true
                }
                _ => false
            }
        });

        confirm_reference.borrow_mut().set_callback(move |_| {
            let mut img_reference = img_ref_2.borrow_mut();
            let tint_strength = strength_ref_2.borrow().value()/100f64;
            let tint_colour = tint_color_ref_2.borrow().to_rgb();

            for x in 0..img_reference.width() {
                for y in 0..img_reference.height() {
                    let pixel = img_reference.get_pixel(x, y).to_rgba();
                    let colours = pixel.0;
                    let new_colour = [
                        clamp(lerp(colours[0] as f32, tint_colour.0 as f32, tint_strength as f32) as i32, 0, 255) as u8,
                        clamp(lerp(colours[1] as f32, tint_colour.1 as f32, tint_strength as f32) as i32, 0, 255) as u8,
                        clamp(lerp(colours[2] as f32, tint_colour.2 as f32, tint_strength as f32) as i32, 0, 255) as u8,
                        colours[3]
                    ];
                    img_reference.put_pixel(x, y, image::Rgba(new_colour));
                }
            }

            println!("Done!");

            win_ref_4.borrow_mut().hide();
        });

        win_ref_2.borrow_mut().end();
        win_ref_2.borrow_mut().show();

        while win_ref_2.borrow_mut().shown() {
            app::wait();
        }

        let new_image = img_ref_1.borrow().clone();
        Tint {
            img: new_image
        }
    }
}
