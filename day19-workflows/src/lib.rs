//! https://adventofcode.com/2023/day/19

use std::collections::HashMap;
use std::ops::Range;

pub mod parser;

#[derive(Debug)]
pub enum PartCategory {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl Part {
    pub fn value_of(&self, category: &PartCategory) -> u32 {
        match category {
            PartCategory::X => self.x,
            PartCategory::M => self.m,
            PartCategory::A => self.a,
            PartCategory::S => self.s,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericPart {
    pub x: Range<u32>,
    pub m: Range<u32>,
    pub a: Range<u32>,
    pub s: Range<u32>,
}

impl GenericPart {
    pub fn default() -> Self {
        GenericPart {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }

    pub fn split_greater(mut self, category: &PartCategory, threshold: u32) -> (Self, Self) {
        let mut split = self.clone();

        match category {
            PartCategory::X => {
                split.x = (threshold + 1)..self.x.end;
                self.x = self.x.start..(threshold + 1);
            },
            PartCategory::M => {
                split.m = (threshold + 1)..self.m.end;
                self.m = self.m.start..(threshold + 1);
            },
            PartCategory::A => {
                split.a = (threshold + 1)..self.a.end;
                self.a = self.a.start..(threshold + 1);
            },
            PartCategory::S => {
                split.s = (threshold + 1)..self.s.end;
                self.s = self.s.start..(threshold + 1);
            },
        }

        (self, split)
    }

    pub fn split_less(mut self, category: &PartCategory, threshold: u32) -> (Self, Self) {
        let mut split = self.clone();

        match category {
            PartCategory::X => {
                split.x = self.x.start..threshold;
                self.x = threshold..self.x.end;
            },
            PartCategory::M => {
                split.m = self.m.start..threshold;
                self.m = threshold..self.m.end;
            },
            PartCategory::A => {
                split.a = self.a.start..threshold;
                self.a = threshold..self.a.end;
            },
            PartCategory::S => {
                split.s = self.s.start..threshold;
                self.s = threshold..self.s.end;
            },
        }

        (self, split)
    }
}

#[derive(Debug)]
pub enum Rule {
    ForwardUnconditionally {
        name: String,
    },
    ForwardIfGreater {
        category: PartCategory,
        threshold: u32,
        name: String,
    },
    ForwardIfLess {
        category: PartCategory,
        threshold: u32,
        name: String,
    },
}

#[derive(Debug)]
pub struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct State {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl State {
    fn process(&self, part: &Part, workflow_name: &str) -> bool {
        match workflow_name {
            "A" => {
                return true;
            },
            "R" => {
                return false;
            },
            _ => { /* continue processing */ },
        }

        let workflow = self.workflows.get(workflow_name).unwrap();
        for rule in &workflow.rules {
            match rule {
                Rule::ForwardUnconditionally { name } => {
                    return self.process(part, &name);
                },
                Rule::ForwardIfGreater {
                    category,
                    threshold,
                    name,
                } => {
                    if part.value_of(category) > *threshold {
                        return self.process(part, &name);
                    }
                },
                Rule::ForwardIfLess {
                    category,
                    threshold,
                    name,
                } => {
                    if part.value_of(category) < *threshold {
                        return self.process(part, &name);
                    }
                },
            }
        }

        unreachable!()
    }

    fn process_parts(&self, initial_workflow_name: &str) -> Vec<&Part> {
        self.parts
            .iter()
            .filter(|part| self.process(part, initial_workflow_name))
            .collect()
    }

    fn process_generic_part(
        &self,
        mut part: GenericPart,
        workflow_name: &str,
        accepted: &mut Vec<GenericPart>,
    ) {
        match workflow_name {
            "A" => {
                accepted.push(part);
                return;
            },
            "R" => {
                return;
            },
            _ => { /* continue processing */ },
        }

        let workflow = self.workflows.get(workflow_name).unwrap();
        for rule in &workflow.rules {
            match rule {
                Rule::ForwardUnconditionally { name } => {
                    self.process_generic_part(part, &name, accepted);
                    return;
                },
                Rule::ForwardIfGreater {
                    category,
                    threshold,
                    name,
                } => {
                    let (remainder, split) = part.split_greater(category, *threshold);
                    self.process_generic_part(split, &name, accepted);
                    part = remainder;
                },
                Rule::ForwardIfLess {
                    category,
                    threshold,
                    name,
                } => {
                    let (remainder, split) = part.split_less(category, *threshold);
                    self.process_generic_part(split, &name, accepted);
                    part = remainder;
                },
            }
        }

        unreachable!()
    }
}

/// Filter the parts through the workflows, then compute the sum of the ratings of accepted parts.
pub fn solve_part1(state: &State) -> u32 {
    state
        .process_parts("in")
        .iter()
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

/// Filter the generic part with ranges 1..=4000 through the workflows to compute how many distinct
/// combinations of ratings will be accepted.
pub fn solve_part2(state: &State) -> u64 {
    let mut accepted: Vec<GenericPart> = Vec::new();
    state.process_generic_part(GenericPart::default(), "in", &mut accepted);

    let mut combinations: u64 = 0;
    for part in accepted {
        combinations +=
            part.x.len() as u64 * part.m.len() as u64 * part.a.len() as u64 * part.s.len() as u64;
    }

    combinations
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        "};

        let state = parser::parse_input(&input).unwrap();
        let result = solve_part1(&state);
        assert_eq!(result, 19114);
        let result = solve_part2(&state);
        assert_eq!(result, 167409079868000);
    }
}
