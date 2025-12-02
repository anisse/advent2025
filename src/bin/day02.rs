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
type ParsedItem = (u64, u64);
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().next().expect("one line").split(',').map(|x| {
        let mut i = x.split('-').map(|x| x.parse().expect("an int"));
        (i.next().expect("item"), i.next().expect("item 2"))
    })
}
fn is_invalid(x: &str) -> bool {
    let mid = x.len() / 2;
    x.len() % 2 == 0 && (0..mid).all(|i| x[i..(i + 1)] == x[(mid + i)..(mid + i + 1)])
}
fn part1<I>(things: I) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    things.fold(0, |mut sum, el| {
        for i in el.0..=el.1 {
            if is_invalid(&format!("{i}")) {
                //println!("{i} is invalid");
                sum += i;
            }
        }
        sum
    })
}
fn is_invalid2(x: &str) -> bool {
    for n in 2..=x.len() {
        if x.len().is_multiple_of(n) {
            let len = x.len() / n;
            if (1..n).all(|i| x[0..len] == x[i * len..(i + 1) * len]) {
                return true;
            }
        }
    }
    false
}

fn part2<I>(things: I) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    things.fold(0, |mut sum, el| {
        for i in el.0..=el.1 {
            if is_invalid2(&format!("{i}")) {
                //println!("{i} is invalid");
                sum += i;
            }
        }
        sum
    })
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 1227775554);
    //part 2
    let res = part2(things);
    assert_eq!(res, 4174379265);
}
