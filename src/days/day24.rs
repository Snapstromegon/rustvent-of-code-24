use std::collections::{HashMap, HashSet};

use crate::solution::{Solution, SolvedValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LogicGate<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

impl<'a> LogicGate<'a> {
    fn eval(
        &self,
        wire_states: &mut HashMap<&'a str, bool>,
        gates: &'a HashMap<&str, LogicGate>,
        visited: &mut HashSet<&'a str>,
    ) -> bool {
        match self {
            LogicGate::And(a, b) => {
                get_wire_state(wire_states, gates, a, visited)
                    && get_wire_state(wire_states, gates, b, visited)
            }
            LogicGate::Or(a, b) => {
                get_wire_state(wire_states, gates, a, visited)
                    || get_wire_state(wire_states, gates, b, visited)
            }
            LogicGate::Xor(a, b) => {
                get_wire_state(wire_states, gates, a, visited)
                    ^ get_wire_state(wire_states, gates, b, visited)
            }
        }
    }

    fn parse(s: &'a str) -> Self {
        let parts = s.split(' ').collect::<Vec<_>>();
        match *parts.get(1).unwrap() {
            "AND" => LogicGate::And(parts[0], parts[2]),
            "OR" => LogicGate::Or(parts[0], parts[2]),
            "XOR" => LogicGate::Xor(parts[0], parts[2]),
            _ => unreachable!("Invalid input"),
        }
    }
}

fn parse_input(input: &str) -> (HashMap<&str, bool>, HashMap<&str, LogicGate>) {
    let (initial, gate_setup) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();

    for line in initial.lines() {
        let (key, value) = line.split_once(": ").unwrap();
        map.insert(key, value == "1");
    }

    let mut gates = HashMap::new();

    for line in gate_setup.lines() {
        let (gate, output) = line.split_once(" -> ").unwrap();
        gates.insert(output, LogicGate::parse(gate));
    }

    (map, gates)
}

fn get_wire_state<'a>(
    wire_states: &mut HashMap<&'a str, bool>,
    gates: &'a HashMap<&str, LogicGate>,
    wire: &'a str,
    visited: &mut HashSet<&'a str>,
) -> bool {
    if visited.contains(wire) {
        return false;
    }
    if let Some(state) = wire_states.get(wire) {
        return *state;
    }

    // println!("Evaluating {wire}");
    // println!("{:?}", gates.get(wire));

    visited.insert(wire);
    let gate = gates.get(wire).unwrap();
    let res = gate.eval(wire_states, gates, visited);
    wire_states.insert(wire, res);
    res
}

fn get_system_var<'a>(
    wire_states: &mut HashMap<&'a str, bool>,
    gates: &'a HashMap<&str, LogicGate>,
    var: char,
) -> usize {
    let mut res = 0;
    let mut var_keys: Vec<&str> = gates
        .keys()
        .filter(|k| k.starts_with(var))
        .copied()
        .collect();
    var_keys.extend(
        wire_states
            .keys()
            .filter(|k| k.starts_with(var))
            .copied()
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
        return None;
        let (wire_states, gates) = parse_input(input);

        let gate_outputs = gates.keys().collect::<Vec<_>>();

        for (g1, &&gate1) in gate_outputs.iter().enumerate() {
            for (g2, &&gate2) in gate_outputs.iter().enumerate().skip(g1 + 1) {
                for (g3, &&gate3) in gate_outputs.iter().enumerate().skip(g2 + 1) {
                    for (g4, &&gate4) in gate_outputs.iter().enumerate().skip(g3 + 1) {
                        for (g5, &&gate5) in gate_outputs.iter().enumerate().skip(g4 + 1) {
                            for (g6, &&gate6) in gate_outputs.iter().enumerate().skip(g5 + 1) {
                                for (g7, &&gate7) in gate_outputs.iter().enumerate().skip(g6 + 1) {
                                    for &gate8 in gate_outputs.iter().skip(g7 + 1) {
                                        let mut wire_states = wire_states.clone();
                                        let mut gates = gates.clone();

                                        let tmp_g1 = gates.get(gate1).unwrap().clone();
                                        gates.insert(gate1, gates.get(gate2).unwrap().clone());
                                        gates.insert(gate2, tmp_g1);

                                        let tmp_g3 = gates.get(gate3).unwrap().clone();
                                        gates.insert(gate3, gates.get(gate4).unwrap().clone());
                                        gates.insert(gate4, tmp_g3);

                                        let tmp_g5 = gates.get(gate5).unwrap().clone();
                                        gates.insert(gate5, gates.get(gate6).unwrap().clone());
                                        gates.insert(gate6, tmp_g5);

                                        let tmp_g7 = gates.get(gate7).unwrap().clone();
                                        gates.insert(gate7, gates.get(gate8).unwrap().clone());
                                        gates.insert(gate8, tmp_g7);

                                        let x = get_system_var(&mut wire_states, &gates, 'x');
                                        let y = get_system_var(&mut wire_states, &gates, 'y');
                                        let z = get_system_var(&mut wire_states, &gates, 'z');
                                        // println!("x: {x}");
                                        // println!("y: {y}");
                                        // println!("z: {z}");

                                        if x + y == z {
                                            let mut res_touched = [
                                                gate1, gate2, gate3, gate4, gate5, gate6, gate7,
                                                gate8,
                                            ];
                                            res_touched.sort_unstable();
                                            return Some(res_touched.join(",").into());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        None
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
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}
