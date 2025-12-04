use advent2025::*;
fn main() {
    let map = grid(input!());
    //part 1
    let res = part1(&map);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&map);
    println!("Part 2: {}", res);
}
fn part1(map: MapRef) -> usize {
    iter_items(map)
        .filter(|(_, c)| *c == b'@')
        .filter(|(pos, _)| {
            [-1, 0, 1]
                .into_iter()
                .flat_map(|y| [-1, 0, 1].into_iter().map(move |x| Coord::from((x, y))))
                .filter(|c| *c != Coord::from((0, 0)))
                .map(|c| Coord::from((pos.x() + c.x(), pos.y() + c.y())))
                .filter(|c| c.valid_for(map))
                .filter(|c| map[c.y()][c.x()] == b'@')
                .count()
                < 4
        })
        .count()
}

fn accessible(map: MapRef) -> impl Iterator<Item = Coord> {
    iter_items(map)
        .filter(|(_, c)| *c == b'@')
        .map(|(pos, _)| pos)
        .filter(|pos| {
            [-1, 0, 1]
                .into_iter()
                .flat_map(|y| [-1, 0, 1].into_iter().map(move |x| Coord::from((x, y))))
                .filter(|c| *c != Coord::from((0, 0)))
                .map(|c| Coord::from((pos.x() + c.x(), pos.y() + c.y())))
                .filter(|c| c.valid_for(map))
                .filter(|c| map[c.y()][c.x()] == b'@')
                .count()
                < 4
        })
}

fn part2(map: MapRef) -> usize {
    let mut map = map.to_vec();
    let mut removed = 0;
    loop {
        let removable = accessible(&map).collect::<Vec<_>>();
        if removable.is_empty() {
            break;
        }
        removable.into_iter().for_each(|pos| {
            map[pos.y()][pos.x()] = b'.';
            removed += 1;
        });
    }
    removed
}

#[test]
fn test() {
    let map = grid(sample!());
    //part 1
    let res = part1(&map);
    assert_eq!(res, 13);
    //part 2
    let res = part2(&map);
    assert_eq!(res, 43);
}
