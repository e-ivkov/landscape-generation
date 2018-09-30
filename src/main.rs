extern crate rand;
extern crate nalgebra as na;
extern crate image;
extern crate ndarray;

pub mod agents;
pub mod posutils;

use image::ImageBuffer;
use rand::Rng;
use na::{Matrix4, Vector4, DMatrix};
use ndarray::Array2;
use agents::generate_landscape;

fn main() {
    let size = 100;
    let lattice_n = 10;
    /*let m = bilinear_interpolation(size as usize, lattice_n, 255);
    let img = ImageBuffer::from_fn((size - size/lattice_n) as u32, (size - size/lattice_n) as u32, |x, y| {
        image::Luma([m[(x as usize,y as usize)] as u8])
    });
    img.save("heightmap_1.png").unwrap();
    let m = gradient_interpolation(size as usize, lattice_n);
    let img = ImageBuffer::from_fn((size - size/lattice_n) as u32, (size - size/lattice_n) as u32, |x, y| {
        image::Luma([((m[[x as usize,y as usize]]/2f64 + 0.5f64) * 255f64) as u8])
    });
    img.save("heightmap_2.png").unwrap();*/
    let m = generate_landscape(size, 100, 1, size*size/2, size*size/500);
    let img = ImageBuffer::from_fn(size as u32, size as u32, |x, y| {
        image::Luma([m[[x as usize,y as usize]] as u8])
    });
    img.save("heightmap_3.png").unwrap();
}

fn gradient_interpolation(size: usize, lattice_n: usize) -> Array2<f64> {
    let mut height_map = Array2::<f64>::zeros((size, size));
    let mut gradients: Vec<(f64,f64)> = Vec::new();
    let step = size / lattice_n;
    for _i in 0..(lattice_n*(lattice_n + 2)) {
        gradients.push((rand::thread_rng().gen_range(-1f64, 1f64), rand::thread_rng().gen_range(-1f64, 1f64)));
    }
    for i in 0..(size-step) {
        for j in 0..(size-step) {
            let i_f = i as f64;
            let j_f = j as f64;
            let x1 = (i / step) as f64;
            let x2 = (i / step + 1) as f64;
            let y1 = (j / step) as f64;
            let y2 = (j / step + 1) as f64;
            let lattice_n_f = lattice_n as f64;
            let step_f = step as f64;
            let h1_1 = gradients[(x1 + lattice_n_f*y1) as usize].0*(i_f-x1*step_f)/step_f
                 + gradients[(x1 + lattice_n_f*y1) as usize].1*(j_f-y1*step_f)/step_f;
            let h1_2 = gradients[(x1 + lattice_n_f*y2) as usize].0*(i_f-x1*step_f)/step_f
                 + gradients[(x1 + lattice_n_f*y2) as usize].1*(y2*step_f-j_f)/step_f;
            let h2_1 = gradients[(x2 + lattice_n_f*y1) as usize].0*(x2*step_f-i_f)/step_f
                 + gradients[(x2 + lattice_n_f*y1) as usize].1*(j_f-y1*step_f)/step_f;
            let h2_2 = gradients[(x2 + lattice_n_f*y2) as usize].0*(x2*step_f-i_f)/step_f
                 + gradients[(x2 + lattice_n_f*y2) as usize].1*(y2*step_f-j_f)/step_f;
            /*let a1 = lerp(h1_1, h2_1, fade(i_f-x1*step_f/step_f));
            let a2 = lerp(h1_2, h2_2, fade(i_f-x1*step_f/step_f));
            height_map[[i,j]] = lerp(a1, a2, fade((j_f-y1*step_f)/step_f));*/
            height_map[[i,j]] = blerp((i_f,j_f), (x1*step_f, x2*step_f), (y1*step_f, y2*step_f), (h1_1, h1_2, h2_1, h2_2));
            //println!("{}", height_map[[i,j]]);
        }
    }
    height_map
}

fn fade(t: f64) -> f64 {                                     
    return t * t * t * (t * (t * 6f64 - 15f64) + 10f64);
}

fn lerp(a: f64, b: f64, x: f64) -> f64 {
    return a + x * (b - a);
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
            let f = (height_map[(x1 as usize, y1 as usize)], height_map[(x1 as usize, y2 as usize)],
                height_map[(x2 as usize, y1 as usize)], height_map[(x2 as usize, y2 as usize)]);
            height_map[(i, j)] = blerp((i as f64, j as f64), (x1,x2), (y1,y2), f)
        }
    }
    height_map
}

fn blerp(p:(f64, f64), x: (f64, f64), y: (f64,f64), f:(f64, f64, f64, f64)) -> f64{
    let m = Matrix4::new(1f64, x.0, y.0, x.0 * y.0,
                         1f64, x.0, y.1, x.0 * y.1,
                         1f64, x.1, y.0, x.1 * y.0,
                         1f64, x.1, y.1, x.1 * y.1);
    let v = Vector4::new(1f64, p.0, p.1, (p.0*p.1));
    let b = m.try_inverse().unwrap().transpose() * v;
    b[0] * f.0 + b[1] * f.1 + b[2] * f.2 + b[3] * f.3
}