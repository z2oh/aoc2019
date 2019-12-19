use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

// Newtype around f32 so we can put floats in a HashSet. This is dangerous, but
// fine for a simple problem like this.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct NF32(f32);
impl Eq for NF32 { }
impl Hash for NF32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0 as u32).hash(state);
    }
}
impl From<f32> for NF32 {
    fn from(f: f32) -> NF32 {
        Self(f)
    }
}

// Simple coordinate type alias.
type Pos = (i32, i32);

#[derive(PartialEq, Clone, Copy)]
enum Space {
    Empty,
    Asteroid,
}

fn main() {
    let inp = include_str!("input")
        .split('\n')
        .map(|line| line.chars());

    // Building the asteroid map.
    let mut map = HashMap::new();
    let mut y = 0;
    for line in inp {
        let mut x = 0;
        for ch in line {
            let space = match ch {
                '#' => Space::Asteroid,
                '.' => Space::Empty,
                _ => panic!("Invalid input char: {}", ch),
            };
            map.insert((x, y), space);
            x += 1;
        }
        y += 1;
    }

    // Part 1.
    let mut max_count = 0;
    let mut station_pos = (-1, -1);
    for (&pos1, &space1) in map.iter() {
        if space1 == Space::Empty { continue }
        let mut set = HashSet::new();
        for (&pos2, &space2) in map.iter() {
            if space2 == Space::Empty { continue }
            if pos1 == pos2 { continue }
            set.insert(angle(pos1, pos2));
        }
        if set.len() > max_count {
            max_count = set.len();
            station_pos = pos1;
        }
    }

    // Part 2.
    let mut angles = Vec::new();
    for (&pos, &space) in map.iter() {
        if space == Space::Empty { continue }
        if pos == station_pos { continue }
        angles.push(Angle {
            theta: transform(angle(station_pos, pos)),
            pos,
            zapped: false,
        });
    }

    // Sort by angle first, and then by distance from the space station.
    angles.sort_unstable_by(|a, b|
        if a.theta == b.theta {
            let a_dist = dist_squared(station_pos, a.pos);
            let b_dist = dist_squared(station_pos, b.pos);
            a_dist.partial_cmp(&b_dist).unwrap()
        } else {
            a.theta.partial_cmp(&b.theta).unwrap()
        }
    );

    let mut iteration = 0;
    let mut idx = 0;
    let mut zapped = 0;
    let ans = loop {
        let mut a = &mut angles[idx];
        if !a.zapped {
            a.zapped = true;
            zapped += 1;
            if zapped == 200 {
                break a.pos;
            }
            // Skip through the rest of this angle.
            let current_angle = a.theta;
            while angles[idx].theta == current_angle {
                idx += 1;
                if idx >= angles.len() {
                    idx %= angles.len();
                    iteration += 1;
                }
            }
        } else {
            idx += iteration;
        }

        if idx >= angles.len() {
            idx %= angles.len();
            iteration += 1;
        }
    };

    println!("{:?}", ans.0 * 100 + ans.1);
}

#[derive(Debug)]
struct Angle {
    theta: NF32,
    pos: Pos,
    zapped: bool,
}

// Transform the angle to be in the correct coordinate space.
fn transform(theta: NF32) -> NF32 {
    let angle = -1.0 * (theta.0 - std::f32::consts::FRAC_PI_2);
    if angle < 0.0 {
        NF32(angle + 2.0 * std::f32::consts::PI)
    } else {
        NF32(angle)
    }
}

// Calculate the angle of the vector between two points. Bounded to [0, 2*pi).
fn angle(from: Pos, to: Pos) -> NF32 {
    // We multiply by -1 on the y coordinate because our y coordinate grows as
    // we move down rather than up.
    let y = -1.0 * (to.1 - from.1) as f32;
    let x = (to.0 - from.0) as f32;
    let mut theta = y.atan2(x);

    // Normalize the angle between [0, 2*pi).
    if theta < 0.0 {
        theta += 2.0 * std::f32::consts::PI;
    }

    NF32(theta)
}

fn dist_squared(from: Pos, to: Pos) -> NF32 {
    let y = (to.1 - from.1) as f32;
    let x = (to.0 - from.0) as f32;
    (x.powi(2) + y.powi(2)).into()
}
