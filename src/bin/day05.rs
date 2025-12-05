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

fn part2<I>(ranges: &[[u64; 2]], _: I) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    let mut ranges_merged = ranges.to_vec();
    let mut to_merge = true;
    while to_merge {
        to_merge = false;
        'outer: for i in 0..(ranges_merged.len() - 1) {
            for j in (i + 1)..ranges_merged.len() {
                //println!("{:?} vs {:?}", ranges_merged[i], ranges_merged[j]);
                if let Some(new) = merge_on_overlap(&ranges_merged[i], &ranges_merged[j]) {
                    //println!("merging");
                    ranges_merged.remove(j);
                    ranges_merged[i] = new;
                    to_merge = true;
                    break 'outer;
                }
            }
        }
    }
    ranges_merged
        .iter()
        .map(|[start, end]| end - start + 1)
        .sum()
}

fn merge_on_overlap(r1: &[u64; 2], r2: &[u64; 2]) -> Option<[u64; 2]> {
    let ra1 = r1[0]..=r1[1];
    let ra2 = r2[0]..=r2[1];
    if ra1.contains(&r2[0]) || ra1.contains(&r2[1]) || ra2.contains(&r1[0]) || ra2.contains(&r1[1])
    {
        return Some([r1[0].min(r2[0]), r1[1].max(r2[1])]);
    }
    None
}

#[test]
fn test() {
    let (ranges, things) = parse(sample!());
    //part 1
    let res = part1(&ranges, things.clone());
    assert_eq!(res, 3);
    //part 2
    let res = part2(&ranges, things);
    assert_eq!(res, 14);
}
