extern crate ndarray;
extern crate rand;

use ndarray::Array2;
use rand::{thread_rng, Rng};
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    pub fn random(size: usize) -> Point {
        let mut rng = thread_rng();
        Point {
            x: rng.gen_range(0, size) as isize,
            y: rng.gen_range(0, size) as isize,
        }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        (((self.x - other.x) as f64).powi(2) + ((self.y - other.y) as f64).powi(2)).sqrt()
    }

    pub fn edge_distance(&self, size: usize) -> usize {
        let dists = vec![
            self.x,
            self.y,
            (size as isize) - self.x,
            (size as isize) - self.y,
        ];
        (*dists.iter().min().unwrap()) as usize
    }
}

pub fn get_directions() -> Vec<Point> {
    let mut directions = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            if x != 0 || y != 0 {
                directions.push(Point { x, y })
            }
        }
    }
    directions
}

pub fn get_neighborhood(pos: &Point, size: usize) -> Vec<Point> {
    get_directions()
        .iter()
        .filter_map(|d| get_neighbor(pos, size, d))
        .collect()
}

pub fn get_neighbor(pos: &Point, size: usize, direction: &Point) -> Option<Point> {
    return check_point(size, pos.add(direction));
}

pub fn check_point(size: usize, position: Point) -> Option<Point> {
    if position.x < size as isize && position.x >= 0 && position.y < size as isize && position.y >= 0
    {
        Some(position)
    } else {
        None
    }
}
