#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[allow(clippy::all)]
pub mod generated_code {
    slint::include_modules!();
}
pub use generated_code::*;
mod camera;
use nokhwa::utils::Resolution;

fn main() {
    let app = MainApp::new().unwrap();
    let app_weak = app.as_weak();
    app.on_open_camera(move |width, height| {
        let app = app_weak.clone();
        match camera::start(app.clone(), 0, Resolution::new(width as u32, height as u32)) {
            Ok(()) => (),
            Err(err) => {
                app.unwrap().set_error(format!("{}", err).into());
                app.unwrap().invoke_show_confirm_popup();
            }
        };
    });

    let _ = app.run();
}
