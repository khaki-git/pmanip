use std::cmp::PartialEq;
use crate::config;
use crate::membuild;
use std::cell::RefCell;
use fltk::{prelude::*, image::PngImage, enums::*, *};
use crate::editor::Editor;

pub struct NewProject {
    pub status: NewProjectEvent,
    pub inp_clone: RefCell<input::Input>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum NewProjectEvent {
    Create,
    Cancel
}

impl NewProject {
    pub fn default() -> Self {
        let mut win = window::Window::default()
            .with_size(500, 300).with_label(config::get_title("New Project".to_string()).as_str());
        win.make_resizable(true);

        let win_ref_1 = membuild::build_ref(win.clone());
        let win_ref_2 = win_ref_1.clone();

        let mut inp = input::Input::default();
        inp.set_color(config::get_colour(config::Colours::BackgroundAlt));
        let mut label = frame::Frame::default()
            .with_label("Name:");
        label.set_label_color(Color::White);
        let mut status = NewProjectEvent::Cancel;
        let status_ref = membuild::build_ref(status);
        let status_ref0 = status_ref.clone();

        let inp_borrow = membuild::build_ref_mut(inp);
        let inp_clone = inp_borrow.clone();

        let mut ok = button::Button::default()
            .with_label("OK");
        ok.set_color(config::get_colour(config::Colours::Primary));
        ok.set_callback(move |_| {
            *status_ref.borrow_mut() = NewProjectEvent::Create;
            win_ref_1.borrow_mut().hide();
        });
        let ok_ref = membuild::build_ref_mut(ok);

        let mut cancel = button::Button::default()
            .with_label("Cancel");
        cancel.set_color(config::get_colour(config::Colours::BackgroundAlt));
        cancel.set_callback(move |_| {
            win_ref_2.borrow_mut().hide();
        });
        let cancel_ref = membuild::build_ref_mut(cancel);

        win.handle(move |win, ev| {
            if ev == Event::Resize {
                let (w, h) = (win.width(), win.height());
                inp_borrow.borrow_mut().resize(0+10, h/2-(50/2), w-20, 50);
                label.resize(0+10, h/2-(50/2)-50, w-20, 50);
                ok_ref.borrow_mut().resize(w/2-160, h-75, 150, 50);
                cancel_ref.borrow_mut().resize(w/2+10, h-75, 150, 50);

                println!("Window size updated (X: {}, Y: {})", w, h);
                true
            } else {
                false
            }
        });

        win.end();
        win.make_modal(true);
        win.show();

        while win.shown() {
            app::wait();
        }

        let new_status = status_ref0.borrow().clone();

        NewProject {
            inp_clone,
            status: new_status
        }
    }
}

pub struct Hero {

}

impl Hero {
    pub fn default() -> Self {
        let mut win = window::Window::default()
            .with_size(800, 500)
            .with_label(config::get_title("Menu".to_string()).as_str());
        win.set_color(config::get_colour(config::Colours::Background));
        win.make_resizable(true);

        let win_ref_1 = membuild::build_ref_mut(win.clone());
        let win_ref_2 = win_ref_1.clone();

        let mut logo = PngImage::load("assets/icon.png").unwrap();
        logo.scale(200, 200, true, true);
        let mut logo_box = frame::Frame::default().center_of(&win);
        logo_box.set_image(Some(logo));

        let mut open_project_button = button::Button::default()
            .with_size(150, 50)
            .below_of(&logo_box, 20)
            .with_label("Open Project");
        open_project_button.set_color(config::get_colour(config::Colours::Primary));

        let mut create_project_button = button::Button::default()
            .with_size(150, 50)
            .below_of(&open_project_button, 20)
            .with_label("New Project");
        create_project_button.set_color(config::get_colour(config::Colours::Primary));

        create_project_button.set_callback(move |_| {
            let new_wind = NewProject::default();
            println!("New project status: {:?}", new_wind.status);
            if new_wind.status == NewProjectEvent::Create {
                let proj_name = new_wind.inp_clone.borrow().value();

                if proj_name.is_empty() {
                    dialog::message_default("Project name cannot be empty.");
                } else {
                    win.hide();
                    Editor::launch(proj_name);
                }
            }
        });

        open_project_button.set_callback(move |_| {
            let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
            dialog.set_filter("*.{pmanip,png,jpg,jpeg,webp}");
            dialog.show();
            let path = dialog.filename();

            if path.to_str().unwrap().to_string().is_empty() {
                dialog::message_default("No file selected.");
            } else {
                win_ref_2.borrow_mut().hide();
                Editor::launch(path.to_str().unwrap().to_string());
            }
        });

        win_ref_1.borrow_mut().handle(move |win, ev| {
            if ev == Event::Resize {
                let (w, h) = (win.width(), win.height());
                logo_box.resize(w/2-100, h/2-100-80, 200, 200);

                create_project_button.resize(
                    w/2-150, h/2-75+120,
                    300, 50
                );

                open_project_button.resize(
                    w/2-150, h/2-75+120+50+15,
                    300, 50
                );
                true
            } else {
                false
            }
        });

        win_ref_1.borrow_mut().end();
        win_ref_1.borrow_mut().show();

        Hero {}
    }
}