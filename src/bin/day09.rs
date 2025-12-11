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

type Point = (i64, i64);
type Segment = ((i64, i64), (i64, i64));

fn area(p1: Point, p2: Point) -> i64 {
    ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1)
}
fn part2<I>(things: I) -> i64
where
    I: Iterator<Item = ParsedItem>,
{
    let coords: Vec<_> = things.collect();
    let mut segments: Vec<Segment> = coords
        .windows(2)
        .map(|s| {
            sorted_seg(
                std::convert::TryInto::<[_; 2]>::try_into(s.to_vec())
                    .unwrap()
                    .into(),
            )
        })
        //.inspect(|s| println!("Seg: {s:?}"))
        .collect();
    segments.push(sorted_seg((coords[coords.len() - 1], coords[0])));
    let mut rects: Vec<_> = coords
        .iter()
        .take(coords.len() - 1)
        .enumerate()
        .flat_map(|(i, c1)| {
            coords
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(_, c2)| (area(*c1, *c2), sorted_seg((*c1, *c2))))
        })
        .collect();
    segments.sort_by_key(|s| -area(s.0, s.1)); //reverse order
    rects.sort_by_key(|(s, _)| -s); // also from biggest to smallest
    rects
        .iter()
        //.inspect(|x| println!("{x:?}"))
        .find(|(_, r)| {
            !segments
                .iter()
                //.inspect(|s| println!("\t{s:?} : {}", segment_in_rect(s, r)))
                .any(|s| segment_in_rect(s, r))
        })
        .expect("an element")
        .0
}

fn sorted_seg(s: Segment) -> Segment {
    (
        (s.0.0.min(s.1.0), s.0.1.min(s.1.1)),
        (s.0.0.max(s.1.0), s.0.1.max(s.1.1)),
    )
}

fn segment_in_rect(s: &Segment, r: &Segment) -> bool {
    s.1.0 > r.0.0 && s.0.0 < r.1.0 && s.1.1 > r.0.1 && s.0.1 < r.1.1
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 50);
    //part 2
    let res = part2(things);
    assert_eq!(res, 24);
}
