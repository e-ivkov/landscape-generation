extern crate rand;
extern crate nalgebra as na;
extern crate image;

use image::{GenericImage, ImageBuffer};
use rand::Rng;
use na::{Matrix4, Vector4, DMatrix};

fn main() {
    let size = 522;
    let lattice_n = 10;
    let m = bilinear_interpolation(size as usize, lattice_n, 255);
    let img = ImageBuffer::from_fn((size - size/lattice_n) as u32, (size - size/lattice_n) as u32, |x, y| {
        image::Luma([m[(x as usize,y as usize)] as u8])
    });
    img.save("heightmap.png").unwrap();
}

fn bilinear_interpolation(size: usize, lattice_n: usize, max_height: usize) -> DMatrix<f64> {
    let mut height_map: DMatrix<f64> = DMatrix::zeros(size, size);
    let step = size / lattice_n;
    for i in 0..lattice_n {
        for j in 0..lattice_n {
            height_map[(i * step, j * step)] = rand::thread_rng().gen_range(0f64, max_height as f64);
            //println!("{}, {}", i * step, j * step);
        }
    }
    for i in 0..(size-step) {
        for j in 0..(size-step) {
            let x1 = ((i / step)*step) as f64;;
            let x2 = ((i / step + 1)*step) as f64;
            let y1 = ((j / step)*step) as f64;
            let y2 = ((j / step + 1)*step) as f64;
            //println!("{}, {}", x2, y2);
            let m = Matrix4::new(1f64, x1, y1, x1 * y1,
                                 1f64, x1, y2, x1 * y2,
                                 1f64, x2, y1, x2 * y1,
                                 1f64, x2, y2, x2 * y2);

            let v = Vector4::new(1f64, i as f64, j as f64, (i*j) as f64);
            let b = m.try_inverse().unwrap().transpose() * v;
            height_map[(i, j)] = b[0] * height_map[(x1 as usize, y1 as usize)] + b[1] * height_map[(x1 as usize, y2 as usize)]
                + b[2] * height_map[(x2 as usize, y1 as usize)]
                + b[3] * height_map[(x2 as usize, y2 as usize)];
        }
    }
    height_map
}