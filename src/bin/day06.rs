use advent2025::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(parse2(input!()));
    println!("Part 2: {}", res);
}
type ParsedItem = Vec<String>;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input
        .lines()
        .map(|x| x.split_ascii_whitespace().map(|n| n.to_string()).collect())
}
fn part1<I>(things: I) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    let mut things = things.peekable();
    let mut columns = vec![vec![]; things.peek().expect("first element").len()];
    let mut sum = 0;
    for line in things {
        if line[0].bytes().next().expect("first char").is_ascii_digit() {
            for (i, num) in line.iter().enumerate() {
                columns[i].push(num.parse::<u64>().expect("an int"));
            }
        } else {
            for (i, op) in line.iter().enumerate() {
                match op.as_ref() {
                    "+" => sum += columns[i].iter().sum::<u64>(),
                    "*" => sum += columns[i].iter().product::<u64>(),
                    _ => unreachable!(),
                };
            }
        }
    }
    sum
}

fn parse2(input: &str) -> Vec<String> {
    transpose(&grid(input))
        .into_iter()
        .rev()
        .map(|l| String::from_utf8(l).unwrap())
        .collect()
}
fn transpose(map: MapRef) -> Map {
    let rows = map.len();
    let cols = map[0].len();
    (0..cols)
        .map(|c| (0..rows).map(|r| map[r][c]).collect())
        .collect()
}

fn part2(things: Vec<String>) -> u64 {
    let mut sum = 0;
    let mut buf = vec![];
    for line in things {
        //println!("{}", line);
        if line.bytes().all(|c| c == b' ') {
            //println!("clear");
            buf.clear();
            continue;
        }
        let el = ints(&line).next().expect("an int");
        buf.push(el);
        //println!("'{el}'");
        if line.ends_with("+") {
            sum += buf.iter().sum::<u64>();
        }
        if line.ends_with("*") {
            sum += buf.iter().product::<u64>();
        }
        //println!("sum: {sum}, {}", buf.len());
    }
    sum
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 4277556);
    //part 2
    let res = part2(parse2(sample!()));
    assert_eq!(res, 3263827);
}
