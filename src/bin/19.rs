use std::{
    cmp,
    collections::{HashMap, VecDeque},
    io::stdin,
    vec,
};

use scanf::sscanf;

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Debug, Clone)]
enum Op {
    Lt,
    Gt,
}

#[derive(Debug, Clone)]
struct Check {
    field: String,
    op: Op,
    val: i32,
}

#[derive(Debug)]
struct Rule {
    check: Option<Check>,
    destination: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

fn parse_rule(s: &str) -> Rule {
    let (mut check_str, mut dest) = (String::new(), String::new());
    if sscanf!(s, "{}:{}", check_str, dest).is_ok() {
        let check_bytes = check_str.as_bytes();
        let field = check_bytes[0] as char;
        let op = check_bytes[1] as char;
        let val = check_str[2..].parse::<i32>().unwrap();
        let op = match op {
            '<' => Op::Lt,
            '>' => Op::Gt,
            _ => todo!(),
        };
        let check = Check {
            field: field.to_string(),
            op: op,
            val: val,
        };
        return Rule {
            check: Some(check),
            destination: dest,
        };
    }
    assert!(!s.contains("<"));
    assert!(!s.contains(">"));
    assert!(!s.contains(":"));
    return Rule {
        check: None,
        destination: s.to_string(),
    };
}

fn read_input() -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut name, mut rules_str) = (String::new(), String::new());
        let (mut x, mut m, mut a, mut s) =
            (String::new(), String::new(), String::new(), String::new());
        if !line.is_empty()
            && sscanf!(&line[1..line.len() - 1], "x={},m={},a={},s={}", x, m, a, s).is_ok()
        {
            parts.push(Part {
                x: x.parse::<i32>().unwrap(),
                m: m.parse::<i32>().unwrap(),
                a: a.parse::<i32>().unwrap(),
                s: s.parse::<i32>().unwrap(),
            });
        } else if sscanf!(&line, "{}{{{}}}", name, rules_str).is_ok() {
            let rules = rules_str.split(",").map(|part| parse_rule(part)).collect();
            workflows.insert(
                name.clone(),
                Workflow {
                    name: name,
                    rules: rules,
                },
            );
        }
    }

    (workflows, parts)
}

fn test(part: &Part, check: &Check) -> bool {
    let f = match check.field.as_str() {
        "x" => part.x,
        "m" => part.m,
        "a" => part.a,
        "s" => part.s,
        _ => todo!(),
    };

    return match check.op {
        Op::Lt => f < check.val,
        Op::Gt => f > check.val,
    };
}

fn apply_wf(part: &Part, wf: &Workflow) -> String {
    for rule in &wf.rules {
        if rule.check.is_none() {
            return rule.destination.clone();
        }
        if let Some(check) = &rule.check {
            if test(part, check) {
                return rule.destination.clone();
            }
        }
    }
    todo!()
}

fn accept(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current = "in".to_string();

    while current != "A" && current != "R" {
        let wf = &workflows[current.as_str()];
        current = apply_wf(part, wf);
    }

    current == "A"
}

fn part_one() {
    let (workflows, parts) = read_input();
    let mut result = 0;
    for part in &parts {
        if accept(part, &workflows) {
            result += part.x + part.m + part.a + part.s;
        }
    }

    println!("{result}");
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i32,
    end: i32,
}

fn interval(s: i32, e: i32) -> Interval {
    Interval { start: s, end: e }
}

fn intersection(i1: Interval, i2: Interval) -> Interval {
    Interval {
        start: cmp::max(i1.start, i2.start),
        end: cmp::min(i1.end, i2.end),
    }
}

fn length(i: Interval) -> i32 {
    cmp::max(i.end - i.start, 0)
}

#[derive(Clone, Copy, Debug)]
struct Restriction {
    x: Interval,
    m: Interval,
    a: Interval,
    s: Interval,
}

const max_val: i32 = 4000;

fn no_restriction() -> Restriction {
    Restriction {
        x: interval(1, max_val + 1),
        m: interval(1, max_val + 1),
        a: interval(1, max_val + 1),
        s: interval(1, max_val + 1),
    }
}

fn rule_interval(r: &Rule) -> Interval {
    if let Some(check) = &r.check {
        return match check.op {
            Op::Lt => interval(1, check.val),
            Op::Gt => interval(check.val + 1, max_val + 1),
        };
    }

    interval(1, max_val + 1)
}

fn add_rule(r: Restriction, rule: &Rule) -> Restriction {
    let mut result = r;
    let interval = rule_interval(&rule);
    if let Some(check) = &rule.check {
        match check.field.as_str() {
            "x" => result.x = intersection(result.x, interval),
            "m" => result.m = intersection(result.m, interval),
            "a" => result.a = intersection(result.a, interval),
            "s" => result.s = intersection(result.s, interval),
            _ => todo!(),
        }
    }

    result
}

fn add_rule_opposite(r: Restriction, rule: &Rule) -> Restriction {
    let mut result = r;
    let interval = rule_interval(&rule);
    let opposite = if interval.start == 1 {
        Interval {
            start: interval.end,
            end: max_val + 1,
        }
    } else {
        Interval {
            start: 1,
            end: interval.start,
        }
    };

    if let Some(check) = &rule.check {
        match check.field.as_str() {
            "x" => result.x = intersection(result.x, opposite),
            "m" => result.m = intersection(result.m, opposite),
            "a" => result.a = intersection(result.a, opposite),
            "s" => result.s = intersection(result.s, opposite),
            _ => todo!(),
        }
    }

    result
}

fn matching_combinations(workflows: &HashMap<String, Workflow>) -> i64 {
    let mut q = VecDeque::new();
    q.push_back((
        "in".to_string(),
        Restriction {
            x: interval(1, max_val + 1),
            m: interval(1, max_val + 1),
            a: interval(1, max_val + 1),
            s: interval(1, max_val + 1),
        },
        vec!["in".to_string()],
    ));

    let mut accepted = Vec::new();

    while !q.is_empty() {
        let (current, restriction, path) = q.pop_back().unwrap();
        if current == "A" {
            accepted.push((restriction, path));
            continue;
        }

        let workflow = &workflows[current.as_str()];
        let mut current_restriction = restriction;
        for rule in &workflow.rules {
            if rule.destination != "R" {
                let mut new_path = path.clone();
                new_path.push(rule.destination.clone());
                q.push_back((
                    rule.destination.clone(),
                    add_rule(current_restriction, rule),
                    new_path,
                ));
            }
            current_restriction = add_rule_opposite(current_restriction, rule);
        }
    }

    let mut result = 0i64;
    for (restriction, path) in &accepted {
        result += (length(restriction.x) as i64)
            * (length(restriction.m) as i64)
            * (length(restriction.a) as i64)
            * (length(restriction.s) as i64);
    }

    result
}

fn part_two() {
    let (workflows, _) = read_input();
    let result = matching_combinations(&workflows);

    println!("{result}");
}

fn main() {
    part_two();
}
