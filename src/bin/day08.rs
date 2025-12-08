use std::collections::{HashMap, HashSet};

use advent2025::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(&things);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&things);
    println!("Part 2: {}", res);
}
type Junction = (i64, i64, i64);
fn parse(input: &str) -> Vec<Junction> {
    input
        .lines()
        .map(|x| {
            let v = ints(x).collect::<Vec<_>>();
            let a: [i64; 3] = v.try_into().expect("junction");
            a.into()
        })
        .collect()
}

fn distance(a: &Junction, b: &Junction) -> i64 {
    ((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)).isqrt()
}
fn part1(things: &[Junction]) -> usize {
    let mut distances: Vec<_> = things
        .iter()
        .take(things.len() - 1)
        .enumerate()
        .flat_map(|(i, a)| {
            things
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, b)| (distance(a, b), (i, j)))
        })
        .collect();
    distances.sort();
    //dbg!(&distances[..1000]);
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for k in 0..1000 {
        let (_, (i, j)) = distances[k];
        graph.entry(i).or_default().push(j);
        graph.entry(j).or_default().push(i);
    }
    //dbg!(&graph);
    let mut in_circuit: HashSet<usize> = HashSet::new();
    let mut circuits: Vec<Vec<usize>> = vec![];
    let mut c = 0;
    for (i, conns) in graph.iter() {
        if in_circuit.contains(i) {
            continue;
        }
        circuits.push(vec![*i]);
        in_circuit.insert(*i);
        let mut queue = conns.to_vec();
        while let Some(next) = queue.pop() {
            if in_circuit.contains(&next) {
                continue;
            }
            circuits[c].push(next);
            in_circuit.insert(next);
            queue.extend(graph[&next].iter());
        }
        c += 1;
    }
    //dbg!(&circuits);
    let mut lens = circuits.iter().map(|c| c.len()).collect::<Vec<_>>();
    lens.sort();
    lens[lens.len() - 1] * lens[lens.len() - 2] * lens[lens.len() - 3]
}

fn part2(things: &[Junction]) -> usize {
    let mut distances: Vec<_> = things
        .iter()
        .take(things.len() - 1)
        .enumerate()
        .flat_map(|(i, a)| {
            things
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, b)| (distance(a, b), (i, j)))
        })
        .collect();
    distances.sort();
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for (_, (i, j)) in distances.into_iter() {
        graph.entry(i).or_default().push(j);
        graph.entry(j).or_default().push(i);
        if is_connected(&graph, things.len()) {
            return (things[i].0 * things[j].0) as usize;
        }
    }
    unreachable!();
}
fn is_connected(graph: &HashMap<usize, Vec<usize>>, len: usize) -> bool {
    let mut in_circuit: HashSet<usize> = HashSet::new();
    let (start, conns) = graph.iter().next().expect("one element");
    in_circuit.insert(*start);
    let mut queue = conns.to_vec();
    while let Some(next) = queue.pop() {
        if in_circuit.contains(&next) {
            continue;
        }
        in_circuit.insert(next);
        queue.extend(graph[&next].iter());
    }

    in_circuit.len() == len
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(&things);
    assert_eq!(res, 40);
    //part 2
    let res = part2(&things);
    assert_eq!(res, 25272);
}
