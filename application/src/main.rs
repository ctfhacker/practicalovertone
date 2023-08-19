use eframe::egui;

use std::f32::consts::PI;

mod hot_reload;
use hot_reload::dlopen;

mod audio;
use audio::get;

use app_state::AppState;

type EguiUpdate = extern "C" fn(&mut AppState, &egui::Context, &mut eframe::Frame);

/// A hot-reloadable library
struct Library {
    handle: hot_reload::Handle,
    update_gui: hot_reload::Symbol<EguiUpdate>,
}

impl Library {
    fn reload(&mut self) {
        self.handle.drop();
        self.handle = hot_reload::load_library("./target/release/libpracticalovertone.so");
        self.update_gui = self.handle.get_symbol("update_gui").unwrap();
    }
}

fn load_library() -> Library {
    // Reload the practical overtone library
    let handle = hot_reload::load_library("./target/release/libpracticalovertone.so");

    // Get the `update_gui` export
    let update_gui = handle.get_symbol("update_gui").unwrap();

    Library { handle, update_gui }
}

struct App {
    library: Library,
    state: AppState,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Reload the library
        self.library.reload();

        let update_gui = &self.library.update_gui;
        update_gui(&mut self.state, ctx, frame);
    }
}

fn main() {
    let library = load_library();

    let mut state = AppState::default();
    let (audio_device, audio_data) = audio::get();
    state.audio_device = Some(audio_device);
    state.audio_data = audio_data;

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "PracticalOvertone",
        options,
        Box::new(|_| Box::new(App { library, state })),
    )
    .expect("Failed to launch app");
}
