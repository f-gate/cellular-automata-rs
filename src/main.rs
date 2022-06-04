
mod rule;
mod block;
mod settings;
#[path = "graphics/wgpud/lib.rs"] mod display;

  fn main() {
    let edge_max: i16 = 20;
    let step_in: i16 = 1;
    let size_bounds = (edge_max - step_in).pow(3);
    let n_rule = rule::Rule {
        ruletype: rule::RuleType::Survival,
        rulegroup: rule::RuleGroup::Multiple(vec![4,5,6,7]),
    };
    let b_rule = rule::Rule {
      ruletype: rule::RuleType::Birth,
      rulegroup: rule::RuleGroup::Multiple(vec![6,7,8]),
    };
    let s_rule = 10;

    let block = block::Block {
        method: block::Method::Moore,
        edge_max,
        step_in,
        size_bounds,
        n_rule: n_rule.get_binary_rule(),
        b_rule: b_rule.get_binary_rule(),
        s_rule,
        grid: block::Block::get_fresh_grid(
          settings::StartShape{
            shape: settings::Shape::Cube,
            is_hollow: true,
        }, &edge_max,
          &size_bounds,
          step_in
        ),
    };
  

    pollster::block_on(display::run(block));
    
}
