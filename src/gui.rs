use eframe::egui;
use crate::block as block;
use crate::settings as settings;
use crate::rule ;


// CURRNTLY BOILER PLATE CODE FROM EFRAME EXAMPLE:: IGNORE FOR NOW
    pub struct CellAutomata {
        name: String,
        age: i32,
        block: block::Block,
    }

    impl CellAutomata {
        pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
            //can do cool stuff here
            Self::default()
        }
    }
    impl Default for CellAutomata {
        fn default() -> Self {
            let block_edge_max: i16 = 8;
            let step_in:i16 = 1;
            let size_bounds: i16 = (block_edge_max - step_in).pow(3);
            
            let start_shape = settings::StartShape {
                shape: settings::Shape::Cube,
                is_hollow: true,
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

            let block = block::Block {
                method: block::Method::Moore,
                edge: block_edge_max,
                step_in: step_in,
                size_bounds: size_bounds,
                n_rule: n_rule_p.get_binary_rule(),
                s_rule: s_rule_v,
                b_rule: b_rule_p.get_binary_rule(),
                grid: block::Block::init(start_shape, &(block_edge_max - step_in), &size_bounds, step_in),
            };

            CellAutomata {
                name :"name".to_owned(),
                age: 10,
                block: block
            }

        }
    }

    impl eframe::App for CellAutomata {
            fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
                egui::CentralPanel::default().show(ctx, |ui| {
                    
                });
            }
    }


    
    