//! https://adventofcode.com/2023/day/22

use std::collections::{HashMap, HashSet};

pub mod parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    x: u32,
    y: u32,
    z: u32,
}

impl From<(u32, u32, u32)> for Coord {
    fn from((x, y, z): (u32, u32, u32)) -> Coord {
        Coord { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Brick {
    start: Coord,
    end: Coord,
}

impl Brick {
    // Assumptions:
    // 1. The z-coordinate is >= 1 (i.e. the brick is above ground)
    // 2. Start and end coordinates differ only along a single axis.
    pub fn try_new(start: Coord, end: Coord) -> Result<Brick, &'static str> {
        if start.z == 0 {
            return Err("Brick must be above ground level");
        }

        if start.x > end.x || start.y > end.y || start.z > end.z {
            return Err("Brick's start coordinate cannot be larger than the end coordinate");
        }

        let diff_count = [end.x - start.x, end.y - start.y, end.z - start.z]
            .into_iter()
            .filter(|diff| *diff != 0)
            .count();

        if diff_count > 1 {
            return Err("Brick must lay along one axis");
        }

        Ok(Brick { start, end })
    }

    pub fn xy_projection(&self) -> HashSet<(u32, u32)> {
        let mut points: HashSet<(u32, u32)> = HashSet::new();
        if self.start.x == self.end.x {
            for i in self.start.y..=self.end.y {
                points.insert((self.start.x, i));
            }
        } else if self.start.y == self.end.y {
            for i in self.start.x..=self.end.x {
                points.insert((i, self.start.y));
            }
        }
        points
    }

    pub fn move_downwards_to(&mut self, z_coord: u32) {
        assert!(self.start.z >= z_coord);
        let offset = self.start.z - z_coord;
        self.start.z -= offset;
        self.end.z -= offset;
    }

    pub fn is_above(&self, other: &Brick) -> bool {
        let self_xy_proj = self.xy_projection();
        let other_xy_proj = other.xy_projection();

        (self.start.z == other.end.z + 1) && !self_xy_proj.is_disjoint(&other_xy_proj)
    }
}

/// Move all bricks down as far as they can go and return this new set of bricks.
fn land_bricks(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut bricks_ordered_by_z = bricks.clone();
    bricks_ordered_by_z.sort_unstable_by_key(|brick| brick.start.z);

    // Keep track of the highest currently occupied z-position for each xy-position. This
    // information lets us compute how far down we can push each succeeding brick.
    let mut z_coord_by_xy_position: HashMap<(u32, u32), u32> = HashMap::new();

    for brick in &mut bricks_ordered_by_z {
        let xy_projection = brick.xy_projection();

        let highest_z_coord = xy_projection
            .iter()
            .map(|xy_pos| z_coord_by_xy_position.get(xy_pos).copied().unwrap_or(0))
            .max()
            .unwrap();

        brick.move_downwards_to(highest_z_coord + 1);

        // Update highest z coords
        for xy_pos in xy_projection {
            z_coord_by_xy_position.insert(xy_pos, brick.end.z);
        }
    }

    bricks_ordered_by_z
}

/// For a given brick, return how many other bricks it supports, i.e. how many bricks would fall
/// down if it were disintegrated.
fn count_supported(
    brick: &Brick,
    bricks_above: &HashMap<Brick, HashSet<Brick>>,
    bricks_below: &HashMap<Brick, HashSet<Brick>>,
) -> u32 {
    let mut supported_bricks = HashSet::new();
    count_supported_recursive(*brick, &mut supported_bricks, bricks_above, bricks_below)
}

// Note: This can be sped up with memoization. For each supported brick, we should save the count,
// because the count of bricks higher up factors in the count of bricks under them.
fn count_supported_recursive(
    brick: Brick,
    supported_bricks: &mut HashSet<Brick>,
    bricks_above: &HashMap<Brick, HashSet<Brick>>,
    bricks_below: &HashMap<Brick, HashSet<Brick>>,
) -> u32 {
    let mut count = 0;
    supported_bricks.insert(brick);

    if let Some(above) = bricks_above.get(&brick) {
        for above_brick in above {
            if let Some(below) = bricks_below.get(above_brick) {
                if below.is_subset(&supported_bricks) {
                    // Add the above brick to the count of supported bricks, plus the count of the
                    // bricks that it supports.
                    count += 1 + count_supported_recursive(
                        *above_brick,
                        supported_bricks,
                        bricks_above,
                        bricks_below,
                    );
                }
            }
        }
    }

    count
}

/// A brick is a support brick if it fully supports at least one other brick.
fn is_support_brick(
    brick: &Brick,
    bricks_above: &HashMap<Brick, HashSet<Brick>>,
    bricks_below: &HashMap<Brick, HashSet<Brick>>,
) -> bool {
    count_supported(brick, bricks_above, bricks_below) != 0
}

/// Return the number of bricks that can be safely disintegrated. A brick can be safely
/// disintegrated if it's not a support brick, i.e. if it is not the only brick that supports
/// another brick.
pub fn solve_part1(bricks: &Vec<Brick>) -> u32 {
    let bricks = land_bricks(bricks);

    let mut bricks_above: HashMap<Brick, HashSet<Brick>> = HashMap::new();
    let mut bricks_below: HashMap<Brick, HashSet<Brick>> = HashMap::new();

    // For each brick compute the sets of bricks above it and below it
    for brick1 in &bricks {
        for brick2 in &bricks {
            if brick1.is_above(brick2) {
                bricks_above.entry(*brick2).or_default().insert(*brick1);
                bricks_below.entry(*brick1).or_default().insert(*brick2);
            }
        }
    }

    bricks
        .iter()
        .filter(|brick| !is_support_brick(brick, &bricks_above, &bricks_below))
        .count() as u32
}

/// For each brick determine how many other bricks would fall if it were disintegrated. Return the
/// sum of these values.
pub fn solve_part2(bricks: &Vec<Brick>) -> u32 {
    let bricks = land_bricks(bricks);

    let mut bricks_above: HashMap<Brick, HashSet<Brick>> = HashMap::new();
    let mut bricks_below: HashMap<Brick, HashSet<Brick>> = HashMap::new();

    // For each brick compute the sets of bricks above it and below it
    for brick1 in &bricks {
        for brick2 in &bricks {
            if brick1.is_above(brick2) {
                bricks_above.entry(*brick2).or_default().insert(*brick1);
                bricks_below.entry(*brick1).or_default().insert(*brick2);
            }
        }
    }

    bricks
        .iter()
        .map(|brick| count_supported(brick, &bricks_above, &bricks_below))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        "};

        let bricks = parser::parse_input(&input).unwrap();
        let result = solve_part1(&bricks);
        assert_eq!(result, 5);
        let result = solve_part2(&bricks);
        assert_eq!(result, 7);
    }
}
