use crate::{BroadcastModule, Configuration, ConjunctionModule, FlipFlopModule, Module};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, one_of},
    combinator::opt,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
enum ModuleKind {
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct ModuleDesc<'a> {
    kind: ModuleKind,
    name: &'a str,
    outputs: Vec<&'a str>,
}

fn module_inputs<'a>(descs: &Vec<ModuleDesc<'a>>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut inputs_by_module: HashMap<&'a str, Vec<&'a str>> = HashMap::new();

    for desc in descs {
        for output in &desc.outputs {
            inputs_by_module
                .entry(output)
                .or_insert_with(|| Vec::new())
                .push(desc.name);
        }
    }
    inputs_by_module
}

fn create_modules<'a>(descs: Vec<ModuleDesc<'a>>) -> HashMap<String, Module> {
    let inputs_by_module = module_inputs(&descs);
    let mut modules = HashMap::new();

    for desc in descs {
        let name = desc.name.to_string();
        let outputs = desc.outputs.into_iter().map(String::from).collect();

        #[rustfmt::skip]
        let module = match desc.kind {
            ModuleKind::Broadcast => {
                Module::Broadcast(BroadcastModule::new(name, outputs))
            },
            ModuleKind::FlipFlop => {
                Module::FlipFlop(FlipFlopModule::new(name, outputs))
            },
            ModuleKind::Conjunction => {
                let inputs = inputs_by_module[desc.name].iter().cloned().map(String::from).collect();
                Module::Conjunction(ConjunctionModule::new(name, outputs, inputs))
            },
        };
        modules.insert(desc.name.to_string(), module);
    }
    modules
}

fn parse_module<'a>(input: &'a str) -> IResult<&'a str, ModuleDesc<'a>> {
    let (remainder, ((kind, name), outputs)) = separated_pair(
        pair(opt(one_of("%&")), alpha1),
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    )(input)?;

    let module_kind = match kind {
        None => ModuleKind::Broadcast,
        Some('%') => ModuleKind::FlipFlop,
        Some('&') => ModuleKind::Conjunction,
        _ => panic!("Unrecognized module type"),
    };
    let module_desc = ModuleDesc {
        kind: module_kind,
        name,
        outputs,
    };

    Ok((remainder, module_desc))
}

pub fn parse_input(input: &str) -> Result<Configuration, Box<dyn std::error::Error + '_>> {
    let (_, module_descs) = separated_list1(multispace1, parse_module)(input)?;

    Ok(Configuration {
        modules: create_modules(module_descs),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn module_descs() {
        let (_, desc) = parse_module("broadcaster -> a, b, c").unwrap();
        assert_matches!(
            desc,
            ModuleDesc {
                kind: ModuleKind::Broadcast,
                ..
            }
        );

        let (_, desc) = parse_module("%a -> b").unwrap();
        assert_matches!(
            desc,
            ModuleDesc {
                kind: ModuleKind::FlipFlop,
                ..
            }
        );
    }
}
