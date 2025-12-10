use std::collections::VecDeque;

use advent2025::*;
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
    things.map(shortest_path).sum()
}

fn shortest_path(m: Machine) -> usize {
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
    for _ in things {
        todo!()
    }
    42
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 42);
    //part 2
    let res = part2(things);
    assert_eq!(res, 42);
}
