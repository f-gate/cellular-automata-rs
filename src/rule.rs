pub enum RuleGroup {
    Single(i32),
    Multiple(Vec<i32>)
}
pub enum RuleType {
    Survival,
    Birth,
    State,
    Neighbor
}

pub struct Rule {
    ruletype: RuleType,
    rulegroup: RuleGroup
}

impl Rule {
    pub fn get_binary_rule(&self) -> [bool; 27] {
        match ruletype(v) {
            RuleGroup::Single => {[bool; 27][v] = true},

            RuleGroup::Multiple => {
                let mut out = [bool; 27];
                v.iter().map(|i| out[i] = true);
                out
            },

        }
    }
}