use fltk::{window, prelude::*, *, enums::*};
use fltk::app::event_key;
use fltk::enums::Color;
use fltk::window::DoubleWindow;
use crate::config;
use crate::membuild;
use crate::tint;
use ::image::{self, ImageBuffer, Rgb};
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use image::{GenericImageView, Rgba};

pub struct Editor;

impl Editor {
    pub fn launch(file_path: String) -> Editor {
        let mut win = window::Window::default()
            .with_size(800, 500)
            .with_label(config::get_title(format!("Editor ({})", file_path)).as_str());
        win.set_color(config::get_colour(config::Colours::Background));
        win.make_resizable(true);

        // Initialize with a default white image
        let mut img_loaded = if Path::new(&file_path).exists() {
            match image::open(&file_path) {
                Ok(opened) => {
                    let mut blank_buffer = ImageBuffer::from_pixel(
                        opened.width(),
                        opened.height(),
                        Rgba([255, 255, 255, 255])
                    );

                    for x in 0..opened.width() {
                        for y in 0..opened.height() {
                            let pixel = opened.get_pixel(x, y);
                            let pixel_contents = pixel.0;
                            blank_buffer.put_pixel(
                                x,
                                y,
                                Rgba([
                                    pixel_contents[0],
                                    pixel_contents[1],
                                    pixel_contents[2],
                                    pixel_contents[3]
                                ])
                            );
                        }
                    }
                    blank_buffer
                },
                Err(e) => {
                    eprintln!("Error opening image file: {}. Creating blank canvas instead.", e);
                    dialog::message_default("Unable to open image file, using blank canvas instead.");
                    ImageBuffer::from_pixel(512, 512, Rgba([255, 255, 255, 255]))
                }
            }
        } else {
            println!("Image file not found, creating blank canvas");
            ImageBuffer::from_pixel(512, 512, Rgba([255, 255, 255, 255]))
        };

        // Rest of the implementation remains the same...
        let img_ref = membuild::build_ref(img_loaded.clone());

        // Store canvas dimensions
        let canvas_width = img_loaded.width() as i32;
        let canvas_height = img_loaded.height() as i32;

        // Create UI elements
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

        let canvas_ref = Rc::new(RefCell::new(canvas));
        let zoom_ref = Rc::new(RefCell::new((0, 0))); // zoom point
        let resize_ref = Rc::new(RefCell::new(1.0f32)); // resize value

        let mut filters_dropdown = menu::MenuButton::default();
        filters_dropdown.set_frame(FrameType::FlatBox);
        filters_dropdown.add_choice("Tint|Blur|Pixelate|Sharpen");
        filters_dropdown.set_color(config::get_colour(config::Colours::BackgroundAlt2));
        filters_dropdown.set_label("Filter");

        // Create menu bar elements reference
        let menu_bar_ref = Rc::new(RefCell::new([file_dropdown, filters_dropdown, canvas_dropdown]));

        // Wrap the window in Rc<RefCell>
        let win_ref = Rc::new(RefCell::new(win));

        // Create draw_canvas closure
        let draw_canvas = {
            let img_ref = img_ref.clone();
            let canvas_ref = canvas_ref.clone();
            let win_ref = win_ref.clone();
            let win_ref_2 = win_ref.clone();

            move || {
                let rescaled = img_ref.borrow().clone();
                let canvas = canvas_ref.borrow();
                let (c_x, c_y) = (canvas.x(), canvas.y());
                let (target_width, target_height) = (canvas.width() as u32, canvas.height() as u32);

                println!("Updating Canvas");

                // Resize image
                let rescaled = image::imageops::resize(
                    &rescaled,
                    target_width,
                    target_height,
                    image::imageops::FilterType::CatmullRom
                );

                let raw_pixels: Vec<u8> = rescaled.into_raw();

                drop(canvas); // Drop the canvas borrow before drawing

                canvas_ref.borrow_mut().draw(move |c| {
                    let mut fltk_image = fltk::image::RgbImage::new(
                        &raw_pixels,
                        target_width as i32,
                        target_height as i32,
                        fltk::enums::ColorDepth::Rgb8,
                    ).unwrap();

                    fltk_image.scale(c.width(), c.height(), true, true);
                    fltk_image.draw(c_x, c_y, c.width(), c.height());
                });

                win_ref_2.borrow_mut().redraw();
            }
        };

        let draw_canvas = Rc::new(RefCell::new(draw_canvas));

        // Set up filters dropdown callback
        {
            let draw_canvas = draw_canvas.clone();
            let img_ref = img_ref.clone();

            if let Some(filters) = menu_bar_ref.borrow_mut().get_mut(1) {
                filters.set_callback(move |dropdown| {
                    if let Some(choice) = dropdown.choice() {
                        match choice.as_str() {
                            "Tint" => {
                                let img = img_ref.borrow().clone();
                                let tint_val = tint::Tint::new(img);
                                *img_ref.borrow_mut() = tint_val.img;
                                draw_canvas.borrow_mut()();
                            },
                            _ => println!("No option"),
                        }
                    }
                });
            }
        }

        // Set up window handling
        {
            let canvas_ref = canvas_ref.clone();
            let resize_ref = resize_ref.clone();
            let zoom_ref = zoom_ref.clone();
            let draw_canvas = draw_canvas.clone();
            let menu_bar_ref = menu_bar_ref.clone();
            let win_ref = win_ref.clone();

            win_ref.borrow_mut().handle(move |win, ev| {
                match ev {
                    Event::Resize => {
                        let (w, h) = (win.width(), win.height());

                        let resize_val = *resize_ref.borrow();
                        let zoom_point = *zoom_ref.borrow();

                        let canvas_width_gui = canvas_width as f32 * resize_val;
                        let canvas_height_gui = canvas_height as f32 * resize_val;

                        let canvas_x = w/2 - canvas_width_gui as i32/2
                            + (-zoom_point.0 as f32 * 0.3 * resize_val) as i32;
                        let canvas_y = h/2 - canvas_height_gui as i32/2
                            + (-zoom_point.1 as f32 * 0.3 * resize_val) as i32;

                        canvas_ref.borrow_mut().resize(
                            canvas_x,
                            canvas_y,
                            canvas_width_gui as i32,
                            canvas_height_gui as i32
                        );

                        tool_bar.resize(0, 50, w/8, h-50);
                        layer_menu.resize(0, 50, w/8, (h/2)-50);
                        dropdown_menu.resize(0, 0, w, 50);

                        // Get the length once before the loop
                        let menu_length = menu_bar_ref.borrow().len();
                        let max = dropdown_menu.width()/2;

                        // Update menu bar elements
                        for (i, element) in menu_bar_ref.borrow_mut().iter_mut().enumerate() {
                            element.resize(
                                (i as i32)*(max/menu_length as i32),
                                0,
                                max/(menu_length as i32),
                                50
                            );
                        }

                        draw_canvas.borrow_mut()();
                        true
                    },
                    Event::KeyDown => {
                        let (w, h) = (win.width(), win.height());
                        let coords_alt = app::event_coords();
                        let coords = (coords_alt.0 - w/2, coords_alt.1 - h/2);

                        let mut zoom_point = zoom_ref.borrow_mut();
                        let mut resize_val = resize_ref.borrow_mut();

                        match event_key().to_char() {
                            Some('q') => {
                                *resize_val *= 1.2;
                                *zoom_point = coords;
                            },
                            Some('e') => {
                                *resize_val *= 0.8;
                                *zoom_point = coords;
                            },
                            Some('a') => zoom_point.0 -= 50,
                            Some('d') => zoom_point.0 += 50,
                            Some('w') => zoom_point.1 -= 50,
                            Some('s') => zoom_point.1 += 50,
                            _ => {}
                        }

                        let canvas_width_gui = canvas_width as f32 * *resize_val;
                        let canvas_height_gui = canvas_height as f32 * *resize_val;

                        let canvas_x = w/2 - canvas_width_gui as i32/2
                            + (-zoom_point.0 as f32 * 0.3 * *resize_val) as i32;
                        let canvas_y = h/2 - canvas_height_gui as i32/2
                            + (-zoom_point.1 as f32 * 0.3 * *resize_val) as i32;

                        drop(zoom_point);
                        drop(resize_val);

                        canvas_ref.borrow_mut().resize(
                            canvas_x,
                            canvas_y,
                            canvas_width_gui as i32,
                            canvas_height_gui as i32
                        );

                        draw_canvas.borrow_mut()();
                        win.redraw();
                        true
                    },
                    _ => false
                }
            });
        }

        win_ref.borrow_mut().end();
        win_ref.borrow_mut().show();

        Editor {}
    }
}