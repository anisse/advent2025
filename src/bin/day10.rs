use advent2025::*;

use std::ops::AddAssign;

use z3::{
    Optimize, SatResult,
    ast::{Ast, Int, IntoAst},
};

fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(things);
    println!("Part 2: {}", res);
}

#[derive(Debug)]
struct Machine {
    target_state: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}
type ParsedItem = Machine;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| {
        let mut parts = x.split_ascii_whitespace();
        let target_state = parts
            .next()
            .expect("lights")
            .bytes()
            .skip(1)
            .filter(|l| *l != b']')
            .map(|l| l == b'#')
            .collect();
        let mut parts = parts.rev();
        let joltage = ints(parts.next().expect("joltage")).collect();
        let buttons = parts.rev().map(|b| ints(b).collect()).collect();

        Machine {
            target_state,
            buttons,
            joltage,
        }
    })
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things.map(light_min_presses).sum()
}

fn light_min_presses(m: Machine) -> usize {
    let mut states: Vec<Vec<bool>> = vec![];
    let mut generation = 0;
    states.push(vec![false; m.target_state.len()]);
    loop {
        generation += 1;
        let mut next = vec![];
        while let Some(state) = states.pop() {
            for b in m.buttons.iter() {
                let mut new_state = state.to_vec();
                button_press(b, &mut new_state);
                if new_state == m.target_state {
                    return generation;
                }
                next.push(new_state);
            }
        }
        states = next;
    }
}

fn button_press(button: &[usize], lights: &mut [bool]) {
    for light in button {
        lights[*light] = !lights[*light];
    }
}

fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things.map(joltage_min_presses_z3).sum()
}

fn joltage_min_presses_bruteforce(m: Machine) -> usize {
    let state = vec![0; m.joltage.len()];
    let mut max = m.joltage.iter().max().unwrap() * 2; /* heuristic */
    min_presses(&m, &state, 0, &mut max);
    max
}

fn min_presses(m: &Machine, state: &[usize], mut current: usize, max: &mut usize) -> Option<usize> {
    current += 1;
    let mut next_state = state.to_vec();
    for b in m.buttons.iter() {
        next_state.copy_from_slice(state);
        press_joltage(b, &mut next_state);
        match cmp(&next_state, &m.joltage) {
            CmpLimit::Under => {
                if current < *max - 1
                    && let Some(new_max) = min_presses(m, &next_state, current, max)
                {
                    println!("New max: {max}");
                    *max = new_max;
                }
            }
            CmpLimit::Equal => return Some(current),
            CmpLimit::Over => {}
        }
    }
    None
}

fn press_joltage(button: &[usize], joltage: &mut [usize]) {
    button.iter().for_each(|b| joltage[*b] += 1);
}

enum CmpLimit {
    Under,
    Equal,
    Over,
}

fn cmp(state: &[usize], limit: &[usize]) -> CmpLimit {
    let mut ret = CmpLimit::Equal;
    for (j, l) in state.iter().zip(limit.iter()) {
        if j < l {
            ret = CmpLimit::Under
        } else if j > l {
            return CmpLimit::Over;
        }
    }
    ret
}

fn joltage_min_presses_z3(m: Machine) -> usize {
    let solver = Optimize::new();
    let mut consts = vec![];
    let mut total = Int::from_i64(0);
    //solver.assert(&total.eq(Int::from_i64(0)));
    for button in 0..m.buttons.len() {
        consts.push(Int::new_const(format!("button {button} presses")));
        total += &consts[button];
        solver.assert(&consts[button].ge(Int::from_i64(0)));
    }
    for (i, joltage) in m.joltage.iter().enumerate() {
        let result = Int::from_i64(*joltage as i64);
        //let mut sum = Int::new_const(format!("equation for joltage {i}"));
        //solver.assert(&sum.eq(Int::from_i64(0)));
        let mut sum = Int::from_i64(0);
        for (j, button) in m.buttons.iter().enumerate() {
            if button.binary_search(&i).is_ok() {
                println!("Button press {j} will increase joltage {i} by 1, total {joltage}");
                sum += &consts[j]
            }
        }
        let constraint = &result.eq(&sum);
        println!("constraint: {constraint:?}");
        solver.assert(constraint);
    }
    solver.minimize(&total);
    assert_eq!(SatResult::Sat, solver.check(&[]));
    let model = solver.get_model().unwrap();
    println!("{model:?}");
    for b in consts.iter() {
        println!(
            "Presses for {b}: {}",
            model.eval(b, true).unwrap().as_i64().unwrap() as usize
        );
    }
    println!(
        "{}",
        model.eval(&total, true).unwrap().as_i64().unwrap() as usize
    );
    println!();
    model.eval(&total, true).unwrap().as_i64().unwrap() as usize
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 7);
    //part 2
    let res = part2(things);
    assert_eq!(res, 33);
}
