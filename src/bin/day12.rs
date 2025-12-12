use advent2025::*;
fn main() {
    let (presents, regions) = parse(input!());
    //part 1
    let res = part1(&presents, regions.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&presents, regions);
    println!("Part 2: {}", res);
}
type ParsedItem = Region;
#[derive(Debug)]
struct Region {
    size: [usize; 2],
    qty: Vec<usize>,
}
fn parse(input: &str) -> (Vec<Map>, impl Iterator<Item = ParsedItem> + Clone + '_) {
    let mut parts = input.split("\n\n");
    let maps = (0..=5)
        .map(|_| grid(&parts.next().expect("present")[3..]))
        //.inspect(|m| print_map(m))
        .collect();

    let regions = parts.next().expect("regions").lines().map(|l| {
        let mut reg = l.split(':');
        let size = ints(reg.next().expect("region size"))
            .inspect(|i| println!("region size {i}"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("two size elements");
        let qty = ints(reg.next().expect("present qty")).collect();
        Region { size, qty }
    });

    (maps, regions)
}
fn part1<I>(presents: &[Map], regions: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    regions.filter(|r| can_fit(r, presents)).count()
}

fn can_fit(r: &Region, presents: &[Map]) -> bool {
    println!("{r:?}");
    return true;
}

fn part2<I>(presents: &[Map], regions: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    for _ in regions {
        todo!()
    }
    42
}

#[test]
fn test() {
    let (presents, regions) = parse(sample!());
    //part 1
    let res = part1(&presents, regions.clone());
    assert_eq!(res, 2);
    //part 2
    let res = part2(&presents, regions);
    assert_eq!(res, 42);
}
