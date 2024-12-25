#![allow(clippy::too_many_lines)]

use std::collections::{HashMap, HashSet};

use crate::solution::{Solution, SolvedValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LogicGate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl LogicGate {
    fn eval(
        &self,
        wire_states: &mut HashMap<String, bool>,
        gates: &HashMap<String, LogicGate>,
        visited: &mut HashSet<String>,
    ) -> bool {
        match self {
            LogicGate::And(a, b) => {
                get_wire_state(wire_states, gates, a.clone(), visited)
                    && get_wire_state(wire_states, gates, b.clone(), visited)
            }
            LogicGate::Or(a, b) => {
                get_wire_state(wire_states, gates, a.clone(), visited)
                    || get_wire_state(wire_states, gates, b.clone(), visited)
            }
            LogicGate::Xor(a, b) => {
                get_wire_state(wire_states, gates, a.clone(), visited)
                    ^ get_wire_state(wire_states, gates, b.clone(), visited)
            }
        }
    }

    fn parse(s: &str) -> Self {
        let parts = s.split(' ').collect::<Vec<_>>();
        match *parts.get(1).unwrap() {
            "AND" => LogicGate::And(parts[0].to_string(), parts[2].to_string()),
            "OR" => LogicGate::Or(parts[0].to_string(), parts[2].to_string()),
            "XOR" => LogicGate::Xor(parts[0].to_string(), parts[2].to_string()),
            _ => unreachable!("Invalid input"),
        }
    }

    fn values(&self) -> (String, String) {
        match self {
            Self::And(a, b) | Self::Or(a, b) | Self::Xor(a, b) => (a.to_owned(), b.to_owned()),
        }
    }

    fn is_direct(&self) -> bool {
        self.values().0.starts_with('x') || self.values().1.starts_with('x')
    }
}

fn parse_input(input: &str) -> (HashMap<String, bool>, HashMap<String, LogicGate>) {
    let (initial, gate_setup) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();

    for line in initial.lines() {
        let (key, value) = line.split_once(": ").unwrap();
        map.insert(key.to_string(), value == "1");
    }

    let mut gates = HashMap::new();

    for line in gate_setup.lines() {
        let (gate, output) = line.split_once(" -> ").unwrap();
        gates.insert(output.to_string(), LogicGate::parse(gate));
    }

    (map, gates)
}

fn get_wire_state(
    wire_states: &mut HashMap<String, bool>,
    gates: &HashMap<String, LogicGate>,
    wire: String,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(&wire) {
        return false;
    }
    if let Some(state) = wire_states.get(&wire) {
        return *state;
    }

    // println!("Evaluating {wire}");
    // println!("{:?}", gates.get(wire));

    visited.insert(wire.clone());
    let gate = gates.get(&wire).unwrap();
    let res = gate.eval(wire_states, gates, visited);
    wire_states.insert(wire, res);
    res
}

fn get_system_var(
    wire_states: &mut HashMap<String, bool>,
    gates: &HashMap<String, LogicGate>,
    var: char,
) -> usize {
    let mut res = 0;
    let mut var_keys: Vec<String> = gates
        .keys()
        .filter(|k| k.starts_with(var))
        .cloned()
        .collect();
    var_keys.extend(
        wire_states
            .keys()
            .filter(|k| k.starts_with(var))
            .cloned()
            .collect::<Vec<_>>(),
    );
    var_keys.sort_unstable();
    for (i, var_key) in var_keys.into_iter().enumerate() {
        if get_wire_state(wire_states, gates, var_key, &mut HashSet::new()) {
            res |= 1 << i;
        }
    }
    res
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let (mut wire_states, gates) = parse_input(input);
        Some(get_system_var(&mut wire_states, &gates, 'z').into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let (wires, gates) = parse_input(input);

        let mut flagged_gates = HashSet::new();

        let gates0: Vec<_> = gates
            .iter()
            .filter(|(_, gate)| gate.is_direct())
            .filter(|(_, gate)| matches!(gate, LogicGate::Xor(_, _)))
            .collect();

        flagged_gates.extend(
            gates0
                .iter()
                .filter(|(output, gate)| {
                    if gate.values().0 == "x00" || gate.values().1 == "x00" {
                        *output != "z00"
                    } else if *output == "z00" {
                        true
                    } else {
                        output.starts_with('z')
                    }
                })
                .map(|(output, _)| (*output).to_string()),
        );

        let gates3: Vec<_> = gates
            .iter()
            .filter(|(_, gate)| matches!(gate, LogicGate::Xor(_, _)))
            .filter(|(_, gate)| !gate.is_direct())
            .collect();

        flagged_gates.extend(
            gates3
                .iter()
                .filter(|(output, _)| !output.starts_with('z'))
                .map(|(output, _)| (*output).to_string()),
        );

        flagged_gates.extend(
            gates
                .iter()
                .filter(|(output, gate)| {
                    if !output.starts_with('z') {
                        return false;
                    }
                    let is_last = *output == &format!("z{}", wires.len() / 2);
                    if is_last {
                        !matches!(gate, LogicGate::Or(_, _))
                    } else {
                        !matches!(gate, LogicGate::Xor(_, _))
                    }
                })
                .map(|(output, _)| output.to_string()),
        );

        let mut check_next = vec![];
        for (output, gate) in gates0 {
            if output == "z00" || flagged_gates.contains(output) {
                continue;
            }
            if !gates3
                .iter()
                .any(|(_, og)| &og.values().0 == output || &og.values().1 == output)
            {
                check_next.push((output, gate));
                flagged_gates.insert(output.clone());
            }
        }

        flagged_gates.extend(check_next.iter().map(|(_, gate)| {
            let intended_result = format!("z{}", &gate.values().0[1..]);
            let matched = gates3
                .iter()
                .find(|(output, _)| *output == &intended_result)
                .unwrap();

            let or_gate = gates
                .iter()
                .filter(|(_, gate)| matches!(gate, LogicGate::Or(_, _)))
                .find(|(output, _)| {
                    *output == &matched.1.values().0 || *output == &matched.1.values().1
                })
                .unwrap();

            if &matched.1.values().0 != or_gate.0 {
                matched.1.values().0
            } else if &matched.1.values().1 != or_gate.0 {
                matched.1.values().1
            } else {
                unreachable!("Should never be reached")
            }
        }));

        let mut wrong_gates = Vec::from_iter(flagged_gates);
        wrong_gates.sort();
        Some(wrong_gates.join(",").into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 24;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(2024.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(57_270_694_330_992.into()));
    }

    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some("gwh,jct,rcb,wbw,wgb,z09,z21,z39".into()));
    }
}
