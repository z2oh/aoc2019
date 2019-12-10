use std::io::{self, prelude::*};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn move_by(&self, instruction: WireInstruction) -> Self {
        use WireInstruction::*;
        match instruction {
            U(v) => Point(self.0, self.1 + v),
            D(v) => Point(self.0, self.1 - v),
            L(v) => Point(self.0 - v, self.1),
            R(v) => Point(self.0 + v, self.1),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum WireInstruction {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

impl WireInstruction {
    fn get_len(&self) -> i32 {
        use WireInstruction::*;
        match &self {
            U(v) => *v,
            D(v) => *v,
            L(v) => *v,
            R(v) => *v,
        }
    }

    fn dir(&self) -> Direction {
        use WireInstruction::*;
        match &self {
            U(_) => Direction::V,
            D(_) => Direction::V,
            L(_) => Direction::H,
            R(_) => Direction::H,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Direction { H, V, }

#[derive(Copy, Clone, Debug)]
struct LineSegment {
    from: Point,
    instruction: WireInstruction,
    // Part 2
    acc: i32,
}

impl LineSegment {
    fn intersect(&self, other: &Self) -> Option<(Point, /* Part 2*/i32)> {
        if self.instruction.dir() == other.instruction.dir() {
            None
        } else {
            // Extract the horizontal and vertical segments.
            let (h, v) = if self.instruction.dir() == Direction::H {
                (self, other)
            } else {
                (other, self)
            };

            use WireInstruction::*;
            let (lx, hx) = match h.instruction {
                L(z) => (h.from.0 - z, h.from.0),
                R(z) => (h.from.0, h.from.0 + z),
                _ => unreachable!(),
            };

            let (ly, hy) = match v.instruction {
                D(z) => (v.from.1 - z, v.from.1),
                U(z) => (v.from.1, v.from.1 + z),
                _ => unreachable!(),
            };

            if hx < v.from.0 || lx > v.from.0 || hy < h.from.1 || ly > h.from.1 {
                None
            } else {
                let (x, y) = (v.from.0, h.from.1);
                let sum = h.acc + (x - h.from.0).abs() + v.acc + (y - v.from.1).abs();
                Some((Point(x, y), sum))
            }
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Error {
    ParseIntError(std::num::ParseIntError),
    IoError(std::io::Error),
    ParseWireInstructionError(String),
    Unhandled,
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl std::str::FromStr for WireInstruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() < 2 {
            Err(Error::ParseWireInstructionError("Too short.".to_string()))
        } else {
            use WireInstruction::*;
            match s.chars().nth(0).unwrap() {
                'U' => Ok(U(s[1..].parse()?)),
                'D' => Ok(D(s[1..].parse()?)),
                'L' => Ok(L(s[1..].parse()?)),
                'R' => Ok(R(s[1..].parse()?)),
                other => Err(Error::ParseWireInstructionError(format!("Unexpected instruction: {}.", other))),
            }
        }
    }
}

fn parse_line(line: &str) -> Result<Vec<LineSegment>> {
    let mut from = Point(0, 0);
    let mut acc = 0;
    line.trim().split(',').map(str::parse).map(|instruction| {
        let instruction = instruction?;
        let segment = LineSegment { from, instruction, acc, };
        from = from.move_by(instruction);
        acc += instruction.get_len();
        Ok(segment)
    }).collect()
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let segments1 = parse_line(&lines.next().ok_or(Error::Unhandled)??)?;
    let segments2 = parse_line(&lines.next().ok_or(Error::Unhandled)??)?;

    /* Part 1
    let origin = Point(0, 0);
    let mut min_dist = std::i32::MAX;
    */
    let origin = Point(0, 0);
    let mut min_acc = std::i32::MAX;

    for s1 in &segments1 {
        for s2 in &segments2 {
            if let Some((pt, acc)) = s1.intersect(s2) {
                /* Part 1
                if pt != origin {
                    min_dist = min_dist.min(dist(origin, pt));
                }
                */
                if pt != origin {
                    min_acc = min_acc.min(acc);
                }
            }
        }
    }

    println!("{}", min_acc);
    Ok(())
}

#[inline(always)]
fn dist(a: Point, b: Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
