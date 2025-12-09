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
type ParsedItem = (i64, i64);
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| {
        let v = ints(x).collect::<Vec<_>>();
        let a: [i64; 2] = v.try_into().expect("tile");
        a.into()
    })
}
fn part1<I>(things: I) -> i64
where
    I: Iterator<Item = ParsedItem>,
{
    let coords: Vec<_> = things.collect();
    coords
        .iter()
        .take(coords.len() - 1)
        .enumerate()
        .flat_map(|(i, c1)| {
            coords
                .iter()
                .enumerate()
                .skip(i + 1)
                //.inspect(move |(_, c2)| println!("{c1:?} vs {c2:?}"))
                .map(move |(_, c2)| (c1.0 - c2.0 + 1).abs() * (c1.1 - c2.1 + 1).abs())
            //.inspect(move |x| println!("{x}"))
        })
        .max()
        .expect("max")
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
    assert_eq!(res, 50);
    //part 2
    //let res = part2(things);
    //assert_eq!(res, 42);
}
