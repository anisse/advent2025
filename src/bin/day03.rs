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
type ParsedItem = Vec<u8>;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|l| l.bytes().map(|x| x - b'0').collect())
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things
        .map(|l| {
            let max = *l.iter().take(l.len() - 1).max().unwrap();
            max as usize * 10
                + *l.iter()
                    .skip(l.iter().position(|x| *x == max).unwrap() + 1)
                    .max()
                    .unwrap() as usize
        })
        //.inspect(|x| println!("{x}"))
        .sum()
}

fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things
        .map(|l| {
            (0..12)
                .scan(0, |pos, i| {
                    let start = *pos;
                    let end = l.len() - (11 - i);
                    //println!("{start} -> {end}");
                    let max = l[start..end]
                        .iter()
                        .enumerate()
                        .reduce(|acc, x| if acc.1 >= x.1 { acc } else { x })
                        .unwrap();
                    //println!("{max:?}");
                    *pos = *pos + max.0 + 1;
                    Some(*max.1 as usize * 10_usize.pow(11 - i as u32))
                })
                .sum::<usize>()
        })
        //.inspect(|x| println!("{x}"))
        .sum()
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 357);
    //part 2
    let res = part2(things);
    assert_eq!(res, 3121910778619);
}
