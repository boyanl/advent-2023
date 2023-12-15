use std::io::stdin;

fn hash(s: &str) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        res = (res + (c as i32)) * 17 % 256;
    }

    res
}

fn part_one() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let parts = line.split(",");
        let mut res = 0;
        for part in parts {
            res += hash(part);
        }

        println!("{res}");
    }
}

#[derive(Clone)]
struct Lens {
    label: String,
    length: i32,
}

#[derive(Clone)]
struct Box {
    lenses: Vec<Lens>,
}

fn simulate(instructions: &Vec<&str>) -> Vec<Box> {
    let mut boxes = vec![Box { lenses: Vec::new() }; 256];

    for &instr in instructions {
        if instr.contains("-") {
            let label = instr.split("-").next().unwrap().to_string();
            let box_num = hash(&label);

            let b = &mut boxes[box_num as usize];
            if let Some(idx) = b.lenses.iter().position(|lens| lens.label == label) {
                b.lenses.remove(idx);
            }
        } else if instr.contains("=") {
            let parts = instr.split("=").collect::<Vec<_>>();
            let label = parts[0].to_string();
            let box_num = hash(&label);
            let length = parts[1].parse::<i32>().unwrap();

            let b = &mut boxes[box_num as usize];
            if let Some(idx) = b.lenses.iter().position(|lens| lens.label == label) {
                b.lenses[idx].length = length;
            } else {
                b.lenses.push(Lens {
                    label: label,
                    length: length,
                });
            }
        }
    }

    boxes
}

fn total_power(boxes: &Vec<Box>) -> i32 {
    let mut result = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, lens) in b.lenses.iter().enumerate() {
            result += ((i + 1) as i32) * ((j + 1) as i32) * lens.length;
        }
    }

    result
}

fn part_two() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let instructions = line.split(",").collect::<Vec<_>>();

        let final_state = simulate(&instructions);
        let result = total_power(&final_state);
        println!("{result}");
    }
}

fn main() {
    part_two();
}
