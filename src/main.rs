
mod rule;
mod block;
mod settings;
#[path = "graphics/wgpud/lib.rs"] mod display;

  fn main() {
    let edge_max: i16 = 7;
    let step_in: i16 = 1;
    let size_bounds = (edge_max - (step_in*2)).pow(3);
    let n_rule = rule::Rule {
        ruletype: rule::RuleType::Survival,
        rulegroup: rule::RuleGroup::Multiple(vec![0,1,2,3,4,5,6]),
    };
    let b_rule = rule::Rule {
      ruletype: rule::RuleType::Birth,
      rulegroup: rule::RuleGroup::Multiple(vec![1,3]),
    };
    let s_rule = 2;

    let mut block = block::Block {
        method: block::Method::VonNeumann,
        edge_max,
        step_in,
        size_bounds,
        n_rule: n_rule.get_binary_rule(),
        b_rule: b_rule.get_binary_rule(),
        s_rule,
        grid: block::Block::get_fresh_grid(
          settings::StartShape{
            shape: settings::Shape::Cube,
            is_hollow: false,
        }, &edge_max,
          &size_bounds,
          step_in
        ),
    };
//    println!("{:?}", block.grid);
  

    pollster::block_on(display::run(block));
    
}
