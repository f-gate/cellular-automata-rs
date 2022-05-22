
mod rule;
mod gui;
mod block;
mod settings;
fn main() {

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("3d Cellular Automata", native_options, Box::new(|cc| Box::new(gui::CellAutomata::new(cc))));
}
