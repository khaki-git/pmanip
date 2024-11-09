use fltk::{window, prelude::*, *, enums::*};
use fltk::app::event_key;
use fltk::enums::Color;
use fltk::window::DoubleWindow;
use crate::config;
use crate::membuild;
use crate::tint;
use ::image;

pub struct Editor;

impl Editor {
    pub fn launch(file_path: String) -> Editor {

        let mut win = window::Window::default()
            .with_size(800, 500)
            .with_label(config::get_title(format!("Editor ({})", file_path)).as_str());
        win.set_color(config::get_colour(config::Colours::Background));
        win.make_resizable(true);

        let mut img_loaded = image::ImageBuffer::from_pixel(512, 512, image::Rgb([255, 255, 255]));

        let mut win_ref_1 = membuild::build_ref(win);
        let mut win_ref_2 = win_ref_1.clone();

        let mut zoom_point = (0, 0);

        let resize_value: f32 = 1f32;

        let resize_ref_1 = membuild::build_ref(resize_value);
        let resize_ref_2 = resize_ref_1.clone();

        let mut canvas_width = 512;
        let mut canvas_height = 512;

        let mut canvas = frame::Frame::default();
        canvas.set_frame(FrameType::FlatBox);
        canvas.set_color(Color::White);

        let mut tool_bar = frame::Frame::default();
        tool_bar.set_frame(FrameType::FlatBox);
        tool_bar.set_color(config::get_colour(config::Colours::BackgroundAlt));

        let mut layer_menu = frame::Frame::default();
        layer_menu.set_frame(FrameType::FlatBox);
        layer_menu.set_color(config::get_colour(config::Colours::BackgroundAlt));

        let mut dropdown_menu = frame::Frame::default();
        dropdown_menu.set_frame(FrameType::FlatBox);
        dropdown_menu.set_color(config::get_colour(config::Colours::BackgroundAlt));

        let mut file_dropdown = menu::MenuButton::default();
        file_dropdown.set_frame(FrameType::FlatBox);
        file_dropdown.add_choice("Save|Export|Open|New");
        file_dropdown.set_color(config::get_colour(config::Colours::BackgroundAlt2));
        file_dropdown.set_label("File");

        let mut canvas_dropdown = menu::MenuButton::default();
        canvas_dropdown.set_frame(FrameType::FlatBox);
        canvas_dropdown.add_choice("Resize|Expand|Merge all Layers");
        canvas_dropdown.set_color(config::get_colour(config::Colours::BackgroundAlt2));
        canvas_dropdown.set_label("Canvas");

        let mut filters_dropdown = menu::MenuButton::default();
        filters_dropdown.set_frame(FrameType::FlatBox);
        filters_dropdown.add_choice("Tint|Blur|Pixelate|Sharpen");
        filters_dropdown.set_color(config::get_colour(config::Colours::BackgroundAlt2));
        filters_dropdown.set_label("Filter");

        filters_dropdown.set_callback(move |dropdown| {
            println!("Activating dropdown");
            match dropdown.choice().unwrap().as_str() {
                "Tint" => {
                    let tint_val = tint::Tint::new(img_loaded.clone());
                    img_loaded = tint_val.img;
                },
                _ => {
                    println!("No option");
                }
            }
        });

        let mut menu_bar_elements = [file_dropdown, filters_dropdown, canvas_dropdown];

        win_ref_1.borrow_mut().handle(move |win, ev| {
            let (w, h) = (win.width(), win.height());
            match ev {
                Event::Resize => {
                    let (w, h) = (win.width(), win.height());
                    println!("scale: {w}, {h}");

                    let canvas_width_gui = canvas_width as f32 * *resize_ref_2.borrow();
                    let canvas_height_gui = canvas_height as f32 * *resize_ref_2.borrow();

                    println!("{}", *resize_ref_2.borrow());

                    let canvas_x = w/2 - canvas_width_gui as i32/2
                        + (-zoom_point.0 as f32 * 0.3 * resize_ref_2.borrow().clone()) as i32;
                    let canvas_y = h/2 - canvas_height_gui as i32/2
                        + (-zoom_point.1 as f32 * 0.3 * resize_ref_2.borrow().clone()) as i32;

                    canvas.resize(canvas_x, canvas_y, canvas_width_gui as i32, canvas_height_gui as i32);

                    tool_bar.resize(0, 50, w/8, h-50);
                    layer_menu.resize(0, 50, w/8, (h/2)-50);
                    dropdown_menu.resize(0, 0, w, 50);

                    let mut i = 0;
                    let items = menu_bar_elements.len();
                    for mut element in menu_bar_elements.iter_mut() {
                        let max = dropdown_menu.width()/2;
                        element.resize(i*(max/items as i32), 0, max/(items as i32), 50);
                        i += 1;
                    }
                    true
                },
                Event::KeyDown => {
                    let coords_alt = app::event_coords();
                    let coords = (
                        coords_alt.0 - w/2,
                        coords_alt.1 - h/2
                        );
                    match event_key().to_char() {
                        Some('q') => {
                            println!("Q pressed");
                            *resize_ref_1.borrow_mut() *= 1.2;
                            zoom_point = coords;
                            win.redraw();
                        },
                        Some('e') => {
                            println!("E pressed");
                            *resize_ref_1.borrow_mut() *= 0.8;
                            zoom_point = coords;
                        },
                        Some('a') => {
                            zoom_point.0 -= 50;
                        },
                        Some('d') => {
                            zoom_point.0 += 50;
                        },
                        Some('w') => {
                            zoom_point.1 -= 50;
                        },
                        Some('s') => {
                            zoom_point.1 += 50;
                        }
                        _ => {}
                    }
                    let canvas_width_gui = canvas_width as f32 * *resize_ref_2.borrow();
                    let canvas_height_gui = canvas_height as f32 * *resize_ref_2.borrow();

                    println!("{}", *resize_ref_2.borrow());

                    let canvas_x = w/2 - canvas_width_gui as i32/2
                        + (-zoom_point.0 as f32 * 0.3 * resize_ref_2.borrow().clone()) as i32;
                    let canvas_y = h/2 - canvas_height_gui as i32/2
                        + (-zoom_point.1 as f32 * 0.3 * resize_ref_2.borrow().clone()) as i32;

                    canvas.resize(canvas_x, canvas_y, canvas_width_gui as i32, canvas_height_gui as i32);
                    win.redraw();
                    true
                },
                _ => false
            }
        });

        win_ref_2.borrow_mut().end();
        win_ref_2.borrow_mut().show();

        Editor {}
    }
}