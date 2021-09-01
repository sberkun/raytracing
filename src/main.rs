
mod vecs;
mod objects;
use vecs::{Color, Point, Ray, Vec3};
use objects::{LightBlocker, Sphere, Material, Mirror};

const IMAGE_WIDTH: usize = 1280;
const IMAGE_HEIGHT: usize = 720;
const ASPECT_RATIO: f64 = (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);
const SUBSAMPLES:usize = 10;

fn main() {
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;

    let mut image = vec![Color::new(); IMAGE_WIDTH*IMAGE_HEIGHT];
    let w = IMAGE_WIDTH as f64;
    let h = IMAGE_HEIGHT as f64;
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
    let mut seed = 0.7;
    eprint!("Rendering... ");
    for i in 0..IMAGE_HEIGHT {
        eprint!("\rRendering... {}/{}", i, IMAGE_HEIGHT);
        for j in 0..IMAGE_WIDTH {
            let mut total = Color{x:0.0,y:0.0,z:0.0};
            for _ in 0..SUBSAMPLES {
                seed = rand(seed);
                let r1 = seed;
                seed = rand(seed);
                let r2 = seed;
                let dir = Point{
                    x:(j as f64 - w/2.0 + r1)*viewport_width/w,
                    y:(i as f64 - h/2.0 + r2)*viewport_height/h,
                    z:focal_length
                };
                total += ray_color(
                    Ray::new(Point::new(), dir),
                    &spheres, 100
                );
            }
            total /= SUBSAMPLES as f64;
            image[i*IMAGE_WIDTH + j] = total
        }
    }
    eprintln!("\rRendering... Done!         ");
    print_image(&image);
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

fn print_image(image:&Vec<Color>) {
    eprint!("Printing... ");
    let mut outp = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let c = image[i*IMAGE_WIDTH + j];
            let r:u8 = scale(c.x);
            let g:u8 = scale(c.y);
            let b:u8 = scale(c.z);
            outp.push_str(&format!("{} {} {}\n",r,g,b));
        }
    }
    eprintln!("Done!");
    print!("{}", outp);
}

fn scale(n: f64) -> u8 {
    if n <= 0.0 {0} else if n >= 1.0 {255} else {(256.0 * n) as u8}
}
