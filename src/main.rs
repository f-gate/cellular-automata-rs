
mod rule;
mod block;
mod settings;
#[path = "graphics/wgpud/lib.rs"] mod display;

  fn main() {
    let edge_max: i16 = 40;
    let step_in: i16 = 5 ;
    let n_rule = rule::Rule {
        ruletype: rule::RuleType::Survival,
        rulegroup: rule::RuleGroup::Multiple(vec![12,13,14,15,16,17,18,19,20,21,22,23,24,25,26]),
    };
    let b_rule = rule::Rule {
      ruletype: rule::RuleType::Birth,
      rulegroup: rule::RuleGroup::Multiple(vec![13, 14]),
    };
    let s_rule = 2;

    let mut block = block::Block {
        method: block::Method::Moore,
        edge_max,
        step_in,
        n_rule: n_rule.get_binary_rule(),
        b_rule: b_rule.get_binary_rule(),
        s_rule,
        grid: block::Block::get_fresh_grid(
          settings::StartShape{
            shape: settings::Shape::Cube,
            is_hollow: false,
        }, &edge_max,
          step_in,
          s_rule
        ),
    };

    pollster::block_on(display::run(block));
    
}
