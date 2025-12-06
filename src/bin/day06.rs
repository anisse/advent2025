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
    assert_eq!(res, 42);
    //part 2
    let res = part2(things);
    assert_eq!(res, 42);
}
