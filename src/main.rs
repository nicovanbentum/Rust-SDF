use std::time::{Instant};
use std::process::Command;

use glam::{Vec3, vec2, uvec2, vec3};
use rayon::prelude::*;

mod sdf;
use sdf::{get_normal, map, soft_shadow};

fn main() {
    let width  = 1920;
    let height  = 1080;

    let aspect_ratio = width as f32 / height as f32;

    let eye = vec3(0.0, 0.0, -1.0);
    let up = vec3(0.0, -1.0, 0.0);
    let right = vec3(1.0 * aspect_ratio, 0.0, 0.0);
    let forward = up.cross(right).normalize();

    let now = Instant::now();

  let pixels = (0..width * height)
    .into_par_iter()
    .map(|i| {
        let frag_coord = uvec2((i % width) as u32, (i / width) as u32);

        let uv = vec2 ( 
            frag_coord.x as f32 / width as f32 * 2.0 - 1.0, 
            frag_coord.y as f32 / height as f32 * 2.0 - 1.0
        );

        let origin = eye + forward * 1.0 + right * uv.x + up * uv.y;
        let direction = (origin - eye).normalize();

        let mut t  = 0.0;

        let light_dir = vec3(-0.8, -1.0, 0.0).normalize();

        let mut color = Vec3::splat(0.0);

        let sky_color = vec3(1.0, 1.0, 1.0).lerp(vec3(0.67, 0.84, 0.9), uv.y * 0.5 + 1.250);

        for _ in 0..64 {
            let pos = origin + direction * t;

            let distance : f32 = map(pos);

            let normal = get_normal(pos);

            let n_dot_l = f32::powi(-light_dir.dot(normal) * 0.5 + 0.5, 2); 

            if distance < 0.0001 {
                color = Vec3::splat(n_dot_l);
                color *= soft_shadow(pos, -light_dir);
                color = color.lerp(sky_color, 1.0 - f32::exp2(-0.911 * distance * distance));
            } else {
                color = sky_color;
            }

            t += distance;
        }
        
        color

    }).collect::<Vec<Vec3>>();
    
    let raw : Vec<u8> = pixels.iter().fold(Vec::new(), |mut v, c| {
        v.push((c.x * 255.0) as u8);
        v.push((c.y * 255.0) as u8);
        v.push((c.z * 255.0) as u8);
        v
    });

    image::save_buffer("sdf.png", raw.as_slice(), width, height, image::ColorType::Rgb8).unwrap();
    
    println!("Finished in {} ms", now.elapsed().as_millis());

    Command::new("explorer").arg("sdf.png").output().expect("Failed to open file sdf.png from Explorer");
}
