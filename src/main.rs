use std::convert::TryInto;

use wasm_bindgen::prelude::*;
use crate::vecs::{Color, Point, Ray, Vec3};
use crate::objects::{LightBlocker, Sphere, Material, Mirror};

const TILESIZE:usize = 20;

#[wasm_bindgen]
pub extern {
    fn export_tile(x: u32, y: u32, w:u32, h:u32, ar: &[u8]);
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}


#[wasm_bindgen]
pub struct Universe {
    image_width: usize,
    image_height: usize,
    focal_length: f64,
    viewport_width: f64,
    viewport_height: f64,
    image: Vec<Color>,
    spheres: Vec<(Sphere, Mirror)>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let mut spheres: Vec<(Sphere, Mirror)> = Vec::new();
        spheres.push((Sphere::new(0.0,0.0,1.0,0.5), Mirror{x:0.5,y:0.5,z:0.5}));
        spheres.push((Sphere::new(0.0,1000.5, 1.0, 1000.0), Mirror{x:0.9,y:0.9,z:0.9}));
        spheres.push((Sphere::new(-1.0, 0.3,0.0, 0.2), Mirror{x:0.5,y:0.8,z:0.5}));
        spheres.push((Sphere::new(-1.0, 0.3,2.0, 0.2), Mirror{x:0.5,y:0.8,z:0.5}));
        spheres.push((Sphere::new(1.0, 0.3,0.0, 0.2), Mirror{x:0.5,y:0.8,z:0.5}));
        spheres.push((Sphere::new(1.0, 0.3,2.0, 0.2), Mirror{x:0.5,y:0.8,z:0.5}));
        spheres.push((Sphere::new(1.414, 0.3, 1.0, 0.2), Mirror{x:0.5,y:0.8,z:0.5}));
        spheres.push((Sphere::new(-1.414, 0.3, 1.0, 0.2), Mirror{x:0.5,y:0.8,z:0.5}));
        spheres.push((Sphere::new(0.0, 0.3, 2.414, 0.2), Mirror{x:0.5,y:0.8,z:0.5}));
        spheres.push((Sphere::new(0.0, 0.3, 1.0 - 1.414, 0.2), Mirror{x:0.5,y:0.8,z:0.5}));
        Universe {
            image_width: 0,
            image_height: 0,
            focal_length: 0.0,
            viewport_width: 0.0,
            viewport_height: 0.0,
            image: Vec::new(),
            spheres,
        }
    }
    
    pub fn render(&mut self, image_width: usize, image_height: usize) {
        if image_width == 0 || image_height == 0 {
            return;
        }
        self.image_width = image_width;
        self.image_height = image_height;
        self.focal_length = 1.0;
        self.viewport_height = 2.0;
        self.viewport_width = (image_width as f64) * self.viewport_height / (image_height as f64);    
        self.image = vec![Color::new(); image_width*image_height];
        let mut seed = 0.60653066; //sqrt 1/e

        for b in 0..(image_height-1)/TILESIZE + 1 {
            for a in 0..(image_width-1)/TILESIZE + 1 {
                let w = if (a+1)*TILESIZE > image_width {image_width - a*TILESIZE} else {TILESIZE};
                let h = if (b+1)*TILESIZE > image_height {image_height - b*TILESIZE} else {TILESIZE};
                self.calculate_tile(&mut seed, a*TILESIZE, b*TILESIZE, w, h);
                print_tile(a*TILESIZE, b*TILESIZE, w, h, &self.image, image_width)
            }
        }
    }
}

impl Universe {
    fn calculate_tile(&mut self, mut seed:&mut f64, x: usize, y: usize, w:usize, h:usize) {
        let wf = self.image_width as f64;
        let hf = self.image_height as f64;
        for i in y..y+h {
            for j in x..x+w {
                let mut total = Color{x:0.0, y:0.0, z:0.0};
                let subsamples = 4;
                for _ in 0..subsamples {
                    *seed = rand(*seed);
                    let r1 = *seed;
                    *seed = rand(*seed);
                    let r2 = *seed;
                    let dir = Point{
                        x:(j as f64 - wf/2.0 + r1)*self.viewport_width/wf,
                        y:(i as f64 - hf/2.0 + r2)*self.viewport_height/hf,
                        z:self.focal_length
                    };
                    total += ray_color(
                        Ray::new(Point::new(), dir),
                        &self.spheres, 100
                    );
                }
                total /= subsamples as f64;
                self.image[i*self.image_width + j] = total;
            }
        }
    }
}

fn print_tile(x: usize, y: usize, w:usize, h:usize, image:&Vec<Color>, image_width:usize) {
    let mut ar = [0;TILESIZE*TILESIZE*3];
    for i in 0..h {
        for j in 0..w {
            ar[(i*w+j)*3 + 0] = scale(image[(y + i)*image_width + (x + j)].x);
            ar[(i*w+j)*3 + 1] = scale(image[(y + i)*image_width + (x + j)].y);
            ar[(i*w+j)*3 + 2] = scale(image[(y + i)*image_width + (x + j)].z);
        }
    }
    export_tile(
        x.try_into().unwrap(),
        y.try_into().unwrap(),
        w.try_into().unwrap(),
        h.try_into().unwrap(), &ar);
}

fn rand(prev: f64) -> f64 {
    (201.7 * prev + 12.3*prev*prev*prev + 0.9) % 1.0
}




fn ray_color(r:Ray, spheres: &Vec<(Sphere, Mirror)>, recursion_depth: usize) -> Color {
    if recursion_depth <= 0 {
        return Color{x:0.0, y:0.0, z:0.0};
    }
    let mut best_ind = 0;
    let mut best:f64 = 0.0;
    let mut got = false;
    for a in 0..spheres.len() {
        let d = spheres[a].0.dist_to(&r);
        if d > 0.0 && (!got || d < best) {
            got = true;
            best = d;
            best_ind = a;
        }
    }
    if !got {
        sky_color(r)
    } else {
        let hit_ray = spheres[best_ind].0.hit(&r);
        let reflected_ray = spheres[best_ind].1.reflection(&r, &hit_ray);
        let reflected_color = ray_color(reflected_ray, spheres, recursion_depth - 1);
        spheres[best_ind].1.color(&reflected_color)
    }
}


fn sky_color(mut r:Ray) -> Color {
    r.direction /= r.direction.length();
    let t = 1.0 - 0.5 * r.direction.y;
    Color{x:1.0,y:1.0,z:1.0} * (1.0-t) + Color{x:0.5,y:0.7,z:1.0} * t 
}

fn scale(n: f64) -> u8 {
    if n <= 0.0 {0} else if n >= 1.0 {255} else {(256.0 * n) as u8}
}
