use advent2025::*;
fn main() {
    let (ranges, things) = parse(input!());
    //part 1
    let res = part1(&ranges, things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&ranges, things);
    println!("Part 2: {}", res);
}
type ParsedItem = u64;
fn parse(input: &str) -> (Vec<[u64; 2]>, impl Iterator<Item = ParsedItem> + Clone + '_) {
    let mut parts = input.split("\n\n");
    (
        parts
            .next()
            .expect("ranges")
            .lines()
            .map(|l| {
                l.split('-')
                    .map(|i| i.parse::<u64>().expect("an int"))
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("two elements array")
            })
            .collect(),
        ints(parts.next().expect("ingredients")),
    )
}
fn part1<I>(ranges: &[[u64; 2]], things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things
        .filter(|i| ranges.iter().any(|[start, end]| (start..=end).contains(&i)))
        .count()
}

fn part2<I>(ranges: &[[u64; 2]], things: I) -> usize
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
