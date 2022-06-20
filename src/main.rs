
mod rule;
mod block;
mod settings;
#[path = "graphics/wgpud/lib.rs"] mod display;

  fn main() {
    let edge_max: i16 = 10;
    let step_in: i16 = 4 ;
    let n_rule = rule::Rule {
        ruletype: rule::RuleType::Survival,
        rulegroup: rule::RuleGroup::Multiple(vec![4]),
    };
    let b_rule = rule::Rule {
      ruletype: rule::RuleType::Birth,
      rulegroup: rule::RuleGroup::Multiple(vec![4]),
    };
    let s_rule = 5;
    let fresh_set = block::Block::get_fresh_grid(
      settings::StartShape{
        shape: settings::Shape::Cube,
        is_hollow: false,
      },
      edge_max,
      step_in,
      s_rule,
      2.0);

    let mut block = block::Block {
        method: block::Method::Moore,
        edge_max,
        step_in,
        n_rule: n_rule.get_binary_rule(),
        b_rule: b_rule.get_binary_rule(),
        s_rule,
        grid: fresh_set.0,
        instances : fresh_set.1,
        space_between_blocks: 2.0,

    };

    pollster::block_on(display::run(block));
    
}
