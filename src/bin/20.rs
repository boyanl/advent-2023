use std::{
    cmp,
    collections::{HashMap, VecDeque},
    io::stdin,
    vec,
};

use scanf::sscanf;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum Signal {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(bool),
    Conjunction(Vec<Signal>),
    Standard,
}

type NameMapping = HashMap<String, usize>;
type Connections = Vec<Vec<usize>>;

fn read_input() -> (NameMapping, Vec<Module>, Connections, Connections) {
    let mut modules = Vec::new();

    let mut outgoing = Vec::new();
    let mut incoming = Vec::new();

    let mut name_mapping = HashMap::new();
    let mut next_num = 0;

    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut from, mut to) = (String::new(), String::new());

        if sscanf!(&line, "{} -> {}", from, to).is_ok() {
            let destinations = to.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();

            let type_char = from.as_bytes()[0] as char;
            let module = match type_char {
                '%' => Module::FlipFlop(false),
                '&' => Module::Conjunction(Vec::new()),
                _ => Module::Standard,
            };
            let name = match type_char {
                '%' | '&' => &from[1..],
                _ => from.as_str(),
            };

            let module_n = *name_mapping.entry(name.to_string()).or_insert_with(|| {
                let old_val = next_num;
                next_num += 1;
                return old_val;
            });
            if outgoing.len() < module_n + 1 {
                outgoing.resize(module_n + 1, Vec::new());
            }

            for d in &destinations {
                let dest_n = *name_mapping.entry(d.clone()).or_insert_with(|| {
                    let old_val = next_num;
                    next_num += 1;
                    return old_val;
                });
                outgoing[module_n].push(dest_n);

                if incoming.len() < dest_n + 1 {
                    incoming.resize(dest_n + 1, Vec::new());
                }
                incoming[dest_n].push(module_n);
            }
            if modules.len() < module_n + 1 {
                modules.resize(module_n + 1, Module::Standard);
            }
            modules[module_n] = module;
        }
    }

    modules.resize(next_num, Module::Standard);

    for (i, module) in modules.iter_mut().enumerate() {
        if let Module::Conjunction(inc_map) = module {
            for _ in 0..incoming[i].len() {
                inc_map.push(Signal::Low);
            }
        }
    }

    (name_mapping, modules, incoming, outgoing)
}

type Input = (NameMapping, Vec<Module>, Connections, Connections);

fn simulate(start: &Input) -> i64 {
    let (mapping, modules, incoming, outgoing) = start;
    let mut current = modules.clone();

    let mut signals = VecDeque::new();

    let button_presses = 1000;
    let mut sent = Vec::new();

    let button_idx = 1_000_000usize;
    let broadcaster_idx = *mapping
        .get("broadcaster")
        .expect("should have broadcaster man");

    for _ in 0..button_presses {
        signals.push_back((Signal::Low, button_idx, broadcaster_idx));

        while !signals.is_empty() {
            let (signal, from, to) = signals.pop_front().unwrap();
            sent.push((signal, from, to));

            let module = &mut current[to];

            match module {
                Module::FlipFlop(on) => {
                    if signal == Signal::Low {
                        *on = !(*on);
                        for dest in &outgoing[to] {
                            let signal = if *on { Signal::High } else { Signal::Low };
                            signals.push_back((signal, to, *dest));
                        }
                    }
                }
                Module::Conjunction(states) => {
                    let from_local_idx = incoming[to].iter().position(|&x| x == from).unwrap();
                    states[from_local_idx] = signal;

                    let out_signal = if states.iter().all(|s| *s == Signal::High) {
                        Signal::Low
                    } else {
                        Signal::High
                    };

                    for dest in &outgoing[to] {
                        signals.push_back((out_signal, to, *dest));
                    }
                }
                Module::Standard => {
                    if to == broadcaster_idx {
                        for dest in &outgoing[to] {
                            signals.push_back((signal, to, *dest));
                        }
                    }
                }
            }
        }
    }

    let mut name_rev_mapping = HashMap::new();
    for (name, idx) in mapping {
        name_rev_mapping.insert(*idx, name.clone());
    }

    let (mut high_cnt, mut low_cnt) = (0, 0);
    for (signal, _, _) in &sent {
        match signal {
            Signal::High => high_cnt += 1,
            Signal::Low => low_cnt += 1,
        }
    }
    (high_cnt as i64) * (low_cnt as i64)
}

fn part_one() {
    let modules = read_input();
    let result = simulate(&modules);
    println!("{result}");
}

fn gcd(i1: i64, i2: i64) -> i64 {
    let (mut d1, mut d2) = (cmp::max(i1, i2), cmp::min(i1, i2));

    while d2 != 0 {
        let rem = d1 % d2;
        d1 = d2;
        d2 = rem;
    }

    d1
}

fn lcm(i1: i64, i2: i64) -> i64 {
    (i1 / gcd(i1, i2)) * i2
}

fn simulate_2(start: &Input) -> i64 {
    let (mapping, modules, incoming, outgoing) = start;
    let mut current = modules.clone();

    let mut signals = VecDeque::new();

    let button_idx = 1_000_000usize;
    let broadcaster_idx = *mapping
        .get("broadcaster")
        .expect("should have broadcaster man");
    let destination_idx = *mapping.get("rx").expect("should have rx man");

    // Exploit the input's structure - rx is connected to a single conjuction
    // Which is connected only to several other conjunctions
    // We need to figure out when these conjunctions emit "High" signals
    // It seems that they're independent and do that on a certain period
    // So the answer is the LCM of all those periods

    let mut emitted_high = vec![Vec::new(); modules.len()];
    let conjunctions_of_interest = &incoming[destination_idx];
    assert!(conjunctions_of_interest.len() == 1);
    let conjunctions_of_interest = &incoming[conjunctions_of_interest[0]];

    for times in 1i64.. {
        if times % 1000000 == 0 {
            println!("Pressed it {times}");
        }
        signals.push_back((Signal::Low, button_idx, broadcaster_idx));

        while !signals.is_empty() {
            let (signal, from, to) = signals.pop_front().unwrap();
            if signal == Signal::Low && to == destination_idx {
                return times;
            }

            let module = &mut current[to];

            match module {
                Module::FlipFlop(on) => {
                    if signal == Signal::Low {
                        *on = !(*on);
                        for dest in &outgoing[to] {
                            let signal = if *on { Signal::High } else { Signal::Low };
                            signals.push_back((signal, to, *dest));
                        }
                    }
                }
                Module::Conjunction(states) => {
                    let from_local_idx = incoming[to].iter().position(|&x| x == from).unwrap();
                    states[from_local_idx] = signal;

                    let out_signal = if states.iter().all(|s| *s == Signal::High) {
                        Signal::Low
                    } else {
                        Signal::High
                    };

                    if out_signal == Signal::High {
                        emitted_high[to].push(times);

                        if conjunctions_of_interest
                            .iter()
                            .all(|idx| emitted_high[*idx].len() > 1)
                        {
                            let mut result = 1i64;
                            for idx in conjunctions_of_interest {
                                let sent_times = &emitted_high[*idx];
                                assert!(sent_times[1] - sent_times[0] == sent_times[0]);

                                result = lcm(result, sent_times[0]);
                            }

                            return result;
                        }
                    }

                    for dest in &outgoing[to] {
                        signals.push_back((out_signal, to, *dest));
                    }
                }
                Module::Standard => {
                    if to == broadcaster_idx {
                        for dest in &outgoing[to] {
                            signals.push_back((signal, to, *dest));
                        }
                    }
                }
            }
        }
    }

    -1
}

fn generate_dot_file(input: &Input) -> String {
    let (names, modules, _, outgoing) = input;
    let mut result = String::new();

    let mut name_rev_mapping = HashMap::new();
    for (name, idx) in names {
        name_rev_mapping.insert(*idx, name.clone());
    }

    result.push_str("digraph {\n");

    for (name, &i) in names {
        let m = &modules[i];

        let shape = match m {
            Module::FlipFlop(_) => "[shape=circle]",
            Module::Conjunction(_) => "[shape=diamond]",
            Module::Standard => "[shape=box]",
        };
        result.push_str(format!("\t{} {}\n", name, shape).as_str());

        for next in &outgoing[i] {
            result.push_str(format!("\t{} -> {}\n", name, name_rev_mapping[next]).as_str());
        }
    }

    result.push_str("}\n");

    result
}

fn part_two() {
    let input = read_input();
    let result = simulate_2(&input);
    println!("{result}");
    //
    // let dot_contents = generate_dot_file(&modules);
    // println!("{dot_contents}");
}

fn main() {
    part_two();
}
