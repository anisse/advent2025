use std::collections::HashMap;

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
type ParsedItem = (String, Vec<String>);
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|l| {
        let mut items = l.split_ascii_whitespace();
        let first = items.next().expect("first");
        let first = first[..first.len() - 1].to_string();

        (first, items.map(str::to_string).collect())
    })
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let graph: Graph = things.collect();
    //print_graph(&graph);
    let mut cache = Cache::new();
    count_paths(&mut cache, "you", "out", &graph)
}
type Cache = HashMap<String, usize>;
type Graph = HashMap<String, Vec<String>>;

fn print_graph(graph: &Graph) {
    println!("digraph {{");
    graph.iter().for_each(|(source, dests)| {
        dests
            .iter()
            .for_each(|dest| println!("\"{source}\" -> \"{dest}\";"))
    });
    println!("}}");
}
fn count_paths(cache: &mut Cache, start: &str, end: &str, graph: &Graph) -> usize {
    if let Some(x) = cache.get(&start.to_string()) {
        return *x;
    }
    let mut count = 0;
    let next = graph.get(start);
    for n in next.iter().flat_map(|x| x.iter()) {
        //println!("{start}->{n}");
        count += if n == end {
            1
        } else {
            count_paths(cache, n, end, graph)
        };
    }
    cache.insert(start.to_string(), count);
    count
}

fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let graph: Graph = things.collect();
    let mut cache = Cache2::new();
    count_paths_convert(&mut cache, "svr", "out", "dac", false, "fft", false, &graph)
}
type Cache2 = HashMap<(String, bool, bool), usize>;
fn count_paths_convert(
    cache: &mut Cache2,
    start: &str,
    end: &str,
    m1: &str,
    m1_seen: bool,
    m2: &str,
    m2_seen: bool,
    graph: &Graph,
) -> usize {
    if let Some(x) = cache.get(&(start.to_string(), m1_seen, m2_seen)) {
        return *x;
    }
    let mut count: usize = 0;
    let next = graph.get(start);
    for n in next.iter().flat_map(|x| x.iter()) {
        let m1_new = m1_seen || n == m1;
        let m2_new = m2_seen || n == m2;
        //println!("{start}->{n} ({m1_new}, {m2_new})",);
        count += if n == end {
            if m1_seen && m2_seen { 1 } else { 0 }
        } else {
            count_paths_convert(cache, n, end, m1, m1_new, m2, m2_new, graph)
        };
        //println!("{start}: {count}");
    }
    cache.insert((start.to_string(), m1_seen, m2_seen), count);
    count
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 5);
    //part 2
    let res = part2(parse(
        "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
    ));
    assert_eq!(res, 2);
}
