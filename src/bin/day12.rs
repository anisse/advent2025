use advent2025::*;
fn main() {
    let (presents, regions) = parse(input!());
    //part 1
    let res = part1(&presents, regions.clone());
    println!("Part 1: {}", res);
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
            //.inspect(|i| println!("region size {i}"))
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
    //let presents = prepare_shapes(presents);
    regions.filter(|r| can_fit(r, presents)).count()
}

fn can_fit(r: &Region, presents: &[Map]) -> bool {
    let space: usize = r
        .qty
        .iter()
        .enumerate()
        .map(|(i, q)| used_space(&presents[i]) * q)
        .sum();
    space < (r.size[0] * r.size[1])
}
fn used_space(p: MapRef) -> usize {
    iter_items(p).filter(|(_, c)| *c == b'#').count()
}
/*
fn prepare_shapes(presents: &[Map]) -> Vec<Vec<Map>> {
    presents
        .iter()
        .map(|p| {
            let mut rotations = (0..4)
                .scan(p.to_vec(), |acc, _| {
                    let r = rotate90(acc);
                    *acc = r.to_vec();
                    Some(r)
                })
                .collect::<Vec<_>>();
            rotations.sort();
            rotations.dedup();
            rotations
        })
        .collect()
}
fn rotate90(map: MapRef) -> Map {
    let rows = map.len();
    let cols = map[0].len();
    (0..cols)
        .map(|c| (0..rows).rev().map(|r| map[r][c]).collect())
        .collect()
}
*/

/*
#[test]
fn test() {
    let (presents, regions) = parse(sample!());
    //part 1
    let res = part1(&presents, regions.clone());
    assert_eq!(res, 2);
}
*/
