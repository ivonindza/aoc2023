//! https://adventofcode.com/2023/day/20

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::{cell::RefCell, rc::Rc};

pub mod parser;

#[derive(Clone)]
pub struct Configuration {
    modules: HashMap<String, Module>,
}

#[derive(Clone)]
enum Module {
    Broadcast(BroadcastModule),
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PulseType {
    High,
    Low,
}

#[derive(Debug, Clone, Copy)]
enum FlipFlopState {
    On,
    Off,
}

impl FlipFlopState {
    pub fn flip(self) -> Self {
        match self {
            FlipFlopState::On => FlipFlopState::Off,
            FlipFlopState::Off => FlipFlopState::On,
        }
    }
}

#[derive(Clone)]
struct FlipFlopModule {
    name: String,
    outputs: Vec<String>,
    state: FlipFlopState,
}

impl FlipFlopModule {
    pub fn new(name: String, outputs: Vec<String>) -> Self {
        FlipFlopModule {
            name,
            outputs,
            state: FlipFlopState::Off,
        }
    }
}

#[derive(Clone)]
struct ConjunctionModule {
    name: String,
    outputs: Vec<String>,
    inputs: HashMap<String, PulseType>,
}

impl ConjunctionModule {
    pub fn new(name: String, outputs: Vec<String>, inputs: Vec<String>) -> Self {
        ConjunctionModule {
            name,
            outputs,
            inputs: HashMap::from_iter(inputs.into_iter().map(|name| (name, PulseType::Low))),
        }
    }
}

#[derive(Clone)]
struct BroadcastModule {
    name: String,
    outputs: Vec<String>,
}

impl BroadcastModule {
    pub fn new(name: String, outputs: Vec<String>) -> Self {
        BroadcastModule { name, outputs }
    }
}

struct Message {
    pub pulse: PulseType,
    pub sender: String,
    pub receiver: String,
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pulse = match self.pulse {
            PulseType::Low => "low",
            PulseType::High => "high",
        };
        write!(f, "{} --{}--> {}", self.sender, pulse, self.receiver)
    }
}

trait Tracker {
    fn track(&mut self, msg: &Message);
}

struct CountTracker {
    low_count: u32,
    high_count: u32,
}

impl CountTracker {
    pub fn new() -> Self {
        CountTracker {
            low_count: 0,
            high_count: 0,
        }
    }

    pub fn counts(&self) -> (u32, u32) {
        (self.low_count, self.high_count)
    }
}

impl Tracker for CountTracker {
    fn track(&mut self, msg: &Message) {
        match msg.pulse {
            PulseType::Low => {
                self.low_count += 1;
            },
            PulseType::High => {
                self.high_count += 1;
            },
        }
    }
}

struct HighPulseTracker {
    module_name: String,
    detected: bool,
}

impl HighPulseTracker {
    pub fn new(module_name: &str) -> Self {
        HighPulseTracker {
            module_name: module_name.to_string(),
            detected: false,
        }
    }

    pub fn is_detected(&self) -> bool {
        self.detected
    }
}

impl Tracker for HighPulseTracker {
    fn track(&mut self, msg: &Message) {
        if msg.sender == self.module_name && msg.pulse == PulseType::High {
            self.detected = true;
        }
    }
}

struct Runner {
    fifo: VecDeque<Message>,
    tracker: Rc<RefCell<dyn Tracker>>,
}

impl Runner {
    pub fn new(tracker: Rc<RefCell<dyn Tracker>>) -> Self {
        Runner {
            fifo: VecDeque::new(),
            tracker,
        }
    }

    fn send_msg(&mut self, msg: Message) {
        self.tracker.borrow_mut().track(&msg);
        self.fifo.push_back(msg);
    }

    fn emit_pulse(&mut self, sender: &str, receivers: &[String], pulse: PulseType) {
        for receiver in receivers {
            let msg = Message {
                pulse,
                sender: sender.to_string(),
                receiver: receiver.to_string(),
            };
            self.send_msg(msg);
        }
    }

    fn push_button(&mut self) {
        let msg = Message {
            pulse: PulseType::Low,
            sender: "button".to_string(),
            receiver: "broadcaster".to_string(),
        };
        self.send_msg(msg);
    }

    fn process_broadcast(&mut self, module: &mut BroadcastModule, msg: Message) {
        self.emit_pulse(&module.name, &module.outputs, msg.pulse);
    }

    fn process_flip_flop(&mut self, module: &mut FlipFlopModule, msg: Message) {
        match msg.pulse {
            PulseType::High => { /* do nothing */ },
            PulseType::Low => {
                match module.state {
                    FlipFlopState::Off => {
                        self.emit_pulse(&module.name, &module.outputs, PulseType::High);
                    },
                    FlipFlopState::On => {
                        self.emit_pulse(&module.name, &module.outputs, PulseType::Low);
                    },
                }
                module.state = module.state.flip();
            },
        }
    }

    fn process_conjunction(&mut self, module: &mut ConjunctionModule, msg: Message) {
        module.inputs.insert(msg.sender, msg.pulse);

        if module
            .inputs
            .values()
            .all(|input| *input == PulseType::High)
        {
            self.emit_pulse(&module.name, &module.outputs, PulseType::Low);
        } else {
            self.emit_pulse(&module.name, &module.outputs, PulseType::High);
        }
    }

    pub fn run_logic(&mut self, cfg: &mut Configuration) {
        self.push_button();

        while let Some(msg) = self.fifo.pop_front() {
            if let Some(recv_module) = cfg.modules.get_mut(&msg.receiver) {
                match recv_module {
                    Module::Broadcast(module) => self.process_broadcast(module, msg),
                    Module::FlipFlop(module) => self.process_flip_flop(module, msg),
                    Module::Conjunction(module) => self.process_conjunction(module, msg),
                }
            }
        }
    }
}

/// Execute the module logic 1000 times. Count the high and low pulses and return their product.
pub fn solve_part1(cfg: &Configuration) -> u32 {
    let mut cfg = cfg.clone();
    let tracker = Rc::new(RefCell::new(CountTracker::new()));
    let mut runner = Runner::new(tracker.clone() as Rc<RefCell<dyn Tracker>>);

    for _ in 0..1000 {
        runner.run_logic(&mut cfg);
    }

    let (low_count, high_count) = tracker.borrow().counts();
    low_count * high_count
}

/// Iterate until a high pulse is detected at the output of the named module. Return the iteration
/// number.
fn iters_until_high_pulse(cfg: &Configuration, module_name: &str) -> u32 {
    let mut cfg = cfg.clone();
    let tracker = Rc::new(RefCell::new(HighPulseTracker::new(module_name)));
    let mut runner = Runner::new(tracker.clone() as Rc<RefCell<dyn Tracker>>);

    let mut n_iters: u32 = 0;
    while !tracker.borrow().is_detected() {
        runner.run_logic(&mut cfg);
        n_iters += 1;
    }
    n_iters
}

/// Count the number of times the button needs to be pressed before a Low pulse is sent to the "rx"
/// module. From the input file we can see that the "rx" module is connected as such:
///
/// &rk    &cd    &zf    &qx
///  |      |      |      |
///  |      ---- ---      |
///  |         | |        |
///  |         v v        |
///  --------> &gh <-------
///             |
///             |
///             v
///             rx
///
/// This means that "rx" will get a Low pulse when all four modules "rk", "cd", "zf", and "qx" emit
/// a High pulse. Therefore, first we compute for each of these modules the iteration when they
/// emit a High pulse. Then we compute the LCM of these values to determine the iteration when "gh"
/// emits a Low pulse and "rx" receives it.
pub fn solve_part2(cfg: &Configuration) -> u64 {
    use num::integer::lcm;

    let res1 = iters_until_high_pulse(cfg, "rk") as u64;
    let res2 = iters_until_high_pulse(cfg, "cd") as u64;
    let res3 = iters_until_high_pulse(cfg, "zf") as u64;
    let res4 = iters_until_high_pulse(cfg, "qx") as u64;

    lcm(lcm(lcm(res1, res2), res3), res4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        "};

        let cfg = parser::parse_input(&input).unwrap();
        let result = solve_part1(&cfg);
        assert_eq!(result, 32000000);
    }

    #[test]
    fn test2() {
        let input = indoc! {"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        "};

        let cfg = parser::parse_input(&input).unwrap();
        let result = solve_part1(&cfg);
        assert_eq!(result, 11687500);
    }
}
