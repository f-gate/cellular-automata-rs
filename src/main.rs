use eframe::egui;

mod gui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(gui::CellAutomata::new(cc))));
}
