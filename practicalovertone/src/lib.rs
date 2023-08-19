use app_state::AppState;

use cpal::traits::StreamTrait;

use egui::plot::{Legend, Line, Plot};

#[no_mangle]
pub fn update_gui(state: &mut AppState, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    ctx.request_repaint();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Hello 1234");
        ui.horizontal(|ui| {
            ui.checkbox(&mut state.draw, "Draw?");
            ui.checkbox(&mut state.record, "Record");
            ui.add(
                egui::DragValue::new(&mut state.frequency)
                    .speed(1)
                    .clamp_range(100..=10000)
                    .prefix("Frequency: "),
            );
        });

        if state.record {
            state.audio_device.as_ref().unwrap().play().unwrap();
        } else {
            state.audio_device.as_ref().unwrap().pause().unwrap();
        }

        if state.draw {
            state.oscillator.set_index(0.0);
            state.oscillator.set_frequency(state.frequency);

            state.buffer.clear();
            for _ in 0..100_000 {
                state.buffer.push(state.oscillator.get_sample() * 10000.0);
            }
        }

        let plot = Plot::new("Test graph")
            .data_aspect(1.5)
            .legend(Legend::default());

        // .center_x_axis(true)
        // .center_y_axis(true);

        plot.show(ui, |plot_ui| {
            plot_ui.line(Line::new(state.points()));
        })
        .response
    });
}
