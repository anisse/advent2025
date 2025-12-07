use std::collections::HashMap;

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
    let start = find_first(map, b'S');
    let mut map = map.to_vec();
    let splitters = iter_items(&map)
        .filter(|(_, c)| *c == b'^')
        .map(|(pos, _)| pos)
        .collect::<Vec<_>>();
    map[start.y() + 1][start.x()] = b'|';

    let mut split = 0;
    splitters.into_iter().for_each(|pos| {
        let up = pos + (0, -1);
        if map[up.y()][up.x()] == b'|' {
            add_beam(&mut map, pos + (-1, 0));
            add_beam(&mut map, pos + (1, 0));
            split += 1;
        }
    });
    //print_map(&map);
    split
}

fn add_beam(map: MapRefMut, mut pos: Coord) {
    while pos.valid_for(map) {
        if map[pos.y()][pos.x()] == b'.' {
            map[pos.y()][pos.x()] = b'|';
        } else {
            break;
        }
        pos = pos + (0, 1);
    }
}
type Cache = HashMap<Coord, usize>;
fn part2(map: MapRef) -> usize {
    let start = find_first(map, b'S');
    let mut cache = Cache::new();
    beam(&mut cache, start + (0, 1), map)
    /*
    let splitters = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == b'^')
                .map(|(x, _)| Coord::from((x, y)))
                .collect::<Vec<_>>()
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    map[start.y() + 1][start.x()] = b'|';

    splitters
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|pos| {
                    let up = *pos + (0, -1);
                    if map[up.y()][up.x()] == b'|' {
                        let left = *pos + (-1, 0);
                        let right = *pos + (1, 0);
                        if left.valid_for(&map) && {
                        }
                        add_beam(&mut map, left);
                        add_beam(&mut map, right);
                        print_map(&map);
                        return true;
                    }
                    false
                })
                .count()
        })
        .inspect(|x| println!("{x:?} splitters"))
        .product()
        */
}

fn beam(cache: &mut Cache, mut pos: Coord, map: MapRef) -> usize {
    if let Some(x) = cache.get(&pos) {
        return *x;
    }
    while pos.valid_for(map) {
        if map[pos.y()][pos.x()] != b'.' {
            break;
        }
        pos = pos + (0, 1);
    }
    if let Some(x) = cache.get(&pos) {
        return *x;
    }
    let mut count = 1;
    if pos.valid_for(map) && map[pos.y()][pos.x()] == b'^' {
        println!("{pos:?}");
        let left = pos + (-1, 0);
        let right = pos + (1, 0);
        let count_left = if left.valid_for(map) {
            beam(cache, left, map)
        } else {
            0
        };
        let count_right = if right.valid_for(map) {
            beam(cache, right, map)
        } else {
            0
        };
        count = count_left + count_right;
    }
    cache.insert(pos, count);
    count
}

#[test]
fn test() {
    let map = grid(sample!());
    //part 1
    let res = part1(&map);
    assert_eq!(res, 21);
    //part 2
    let res = part2(&map);
    assert_eq!(res, 40);
}
