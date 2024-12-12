use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/12/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pricing {
    ByPerimeter,
    BySides,
}

fn solve(input: &str, pricing: Pricing) -> usize {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut region_ids = (0..grid.len())
        .map(|y| (0..grid[y].len()).map(|_| -1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut next_id = 1;

    (0..grid.len()).for_each(|y| {
        (0..grid[y].len()).for_each(|x| {
            if region_ids[y][x] == -1 {
                let current_id = next_id;
                let current_grid_letter = grid[y][x];
                next_id += 1;

                fn flood_fill(
                    grid: &[Vec<char>],
                    region_ids: &mut Vec<Vec<i32>>,
                    current_id: i32,
                    current_grid_letter: char,
                    x: usize,
                    y: usize,
                ) {
                    region_ids[y][x] = current_id;

                    fn suitable(
                        grid: &[Vec<char>],
                        region_ids: &[Vec<i32>],
                        current_grid_letter: char,
                        pos: &(usize, usize),
                    ) -> bool {
                        let x = pos.0;
                        let y = pos.1;
                        region_ids[y][x] == -1 && grid[y][x] == current_grid_letter
                    }

                    if x > 0 {
                        let left = (x - 1, y);
                        if suitable(grid, region_ids, current_grid_letter, &left) {
                            flood_fill(
                                grid,
                                region_ids,
                                current_id,
                                current_grid_letter,
                                left.0,
                                left.1,
                            );
                        }
                    }
                    if x + 1 < region_ids[y].len() {
                        let right = (x + 1, y);
                        if suitable(grid, region_ids, current_grid_letter, &right) {
                            flood_fill(
                                grid,
                                region_ids,
                                current_id,
                                current_grid_letter,
                                right.0,
                                right.1,
                            );
                        }
                    }
                    if y > 0 {
                        let up = (x, y - 1);
                        if suitable(grid, region_ids, current_grid_letter, &up) {
                            flood_fill(
                                grid,
                                region_ids,
                                current_id,
                                current_grid_letter,
                                up.0,
                                up.1,
                            )
                        }
                    }
                    if y + 1 < region_ids.len() {
                        let down = (x, y + 1);
                        if suitable(grid, region_ids, current_grid_letter, &down) {
                            flood_fill(
                                grid,
                                region_ids,
                                current_id,
                                current_grid_letter,
                                down.0,
                                down.1,
                            );
                        }
                    }
                }

                flood_fill(
                    &grid,
                    &mut region_ids,
                    current_id,
                    current_grid_letter,
                    x,
                    y,
                );
            }
        });
    });

    let mut areas: HashMap<i32, usize> = HashMap::new();
    let mut perimeters: HashMap<i32, usize> = HashMap::new();
    let mut sides: HashMap<i32, usize> = HashMap::new();

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Fence {
        Top,
        Bottom,
        Left,
        Right,
    }

    let mut fences: HashMap<(usize, usize), HashSet<(i32, Fence)>> = HashMap::new();
    let empty_hashset: HashSet<(i32, Fence)> = HashSet::new();

    region_ids.iter().enumerate().for_each(|(y, lines)| {
        lines.iter().enumerate().for_each(|(x, id)| {
            *areas.entry(*id).or_default() += 1;

            if x == 0 || region_ids[y][x - 1] != *id {
                fences.entry((x, y)).or_default().insert((*id, Fence::Left));
                *perimeters.entry(*id).or_default() += 1;

                if y == 0
                    || !fences
                        .get(&(x, y - 1))
                        .unwrap_or(&empty_hashset)
                        .contains(&(*id, Fence::Left))
                {
                    *sides.entry(*id).or_default() += 1;
                }
            }
            if y == 0 || region_ids[y - 1][x] != *id {
                fences.entry((x, y)).or_default().insert((*id, Fence::Top));
                *perimeters.entry(*id).or_default() += 1;

                if x == 0
                    || !fences
                        .get(&(x - 1, y))
                        .unwrap_or(&empty_hashset)
                        .contains(&(*id, Fence::Top))
                {
                    *sides.entry(*id).or_default() += 1;
                }
            }
            if x + 1 >= region_ids[y].len() || region_ids[y][x + 1] != *id {
                fences
                    .entry((x, y))
                    .or_default()
                    .insert((*id, Fence::Right));
                *perimeters.entry(*id).or_default() += 1;

                if y == 0
                    || !fences
                        .get(&(x, y - 1))
                        .unwrap_or(&empty_hashset)
                        .contains(&(*id, Fence::Right))
                {
                    *sides.entry(*id).or_default() += 1;
                }
            }
            if y + 1 >= region_ids.len() || region_ids[y + 1][x] != *id {
                fences
                    .entry((x, y))
                    .or_default()
                    .insert((*id, Fence::Bottom));
                *perimeters.entry(*id).or_default() += 1;

                if x == 0
                    || !fences
                        .get(&(x - 1, y))
                        .unwrap_or(&empty_hashset)
                        .contains(&(*id, Fence::Bottom))
                {
                    *sides.entry(*id).or_default() += 1;
                }
            }
        });
    });

    let measurements = match pricing {
        Pricing::ByPerimeter => perimeters,

        Pricing::BySides => sides,
    };

    measurements
        .into_iter()
        .map(|(id, measurement)| *areas.get(&id).unwrap_or(&0) * measurement)
        .sum::<usize>()
}

fn p1(input: &str) -> String {
    solve(input, Pricing::ByPerimeter).to_string()
}

fn p2(input: &str) -> String {
    solve(input, Pricing::BySides).to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(
            p1(r"
AAAA
BBCD
BBCC
EEEC
"),
            "140"
        );
        assert_eq!(
            p1(r"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"),
            "772"
        );
        assert_eq!(
            p1(r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"),
            "1930"
        );
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1450816");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(
            p2(r"
AAAA
BBCD
BBCC
EEEC
"),
            "80"
        );
        assert_eq!(
            p2(r"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"),
            "436"
        );
        assert_eq!(
            p2(r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"),
            "1206"
        );
        assert_eq!(
            p2(r"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"),
            "236"
        );
        assert_eq!(
            p2(r"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"),
            "368"
        );
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "865662");
    }
}
