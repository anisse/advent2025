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
type ParsedItem = i32;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| {
        let mut s = x.chars();
        let c = s.next().expect("char");
        let mul = if c == 'L' { -1 } else { 1 };
        mul * s.collect::<String>().parse::<i32>().expect("not int")
    })
}
fn part1<I>(things: I) -> i32
where
    I: Iterator<Item = ParsedItem>,
{
    things
        .fold((50, 0), |(point, mut pass), x| {
            let c = (point + x) % 100;
            if c == 0 {
                pass += 1
            };
            (c, pass)
        })
        .1
}

fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things
        .fold((50, 0), |(point, mut pass), x| {
            let mut sum = point + x;
            //println!("{sum}, {pass}");
            if sum == 0 || (point != 0 && (sum.signum() != point.signum())) {
                pass += 1
            }
            while sum.abs() >= 100 {
                sum += sum.signum() * -100;
                //println!("adjusted to {sum}");
                pass += 1
            }

            //println!("got {sum}, {pass}");
            (sum, pass)
        })
        .1
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 3);
    //part 2
    let res = part2(things);
    assert_eq!(res, 6);
}
