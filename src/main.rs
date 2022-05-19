use block::Block;
use eframe::egui;

mod rule;
mod gui;
mod block;
mod settings;
fn main() {
    const BLOCK_SIZE: i16 = 256;
    let block_edge = (BLOCK_SIZE as f32).cbrt().round() as i16;
    let SIZE_FACTOR = 0.5;
    let SIZE_BOUNDS = (block_edge + 1).pow(3);
    let START_SHAPE = settings::StartShape {
        shape: settings::Shape::Cube,
        is_hollow: false,
    };

    let n_rule_p = rule::Rule {
        rulegroup: rule::RuleGroup::Single(5),
        ruletype: rule::RuleType::Survival,
    };
    let s_rule_p = rule::Rule {
        rulegroup: rule::RuleGroup::Single(5),
        ruletype: rule::RuleType::State,
    };
    let b_rule_p= rule::Rule {
        rulegroup: rule::RuleGroup::Single(5),
        ruletype: rule::RuleType::Birth,
    };
    let mut s_rule_v = 0;
    if let rule::RuleGroup::Single(v) = s_rule_p.rulegroup {
        s_rule_v = v;
    }

    let block = Block {
        method: block::Method::Moore,
        edge: block_edge,
        size_factor: SIZE_FACTOR,
        size_bounds: SIZE_BOUNDS,
        n_rule: n_rule_p.get_binary_rule(),
        s_rule: s_rule_v,
        b_rule: b_rule_p.get_binary_rule(),
        grid: block::Block::init(START_SHAPE, SIZE_BOUNDS, SIZE_FACTOR)
    };

    println!("{:?}", block.grid);





    let native_options = eframe::NativeOptions::default();
    eframe::run_native("3d Cellular Automata", native_options, Box::new(|cc| Box::new(gui::CellAutomata::new(cc))));
}
