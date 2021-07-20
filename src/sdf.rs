
use glam::{Vec2Swizzles, Vec3, vec2, vec3};

pub fn sphere(p : Vec3, radius : f32) -> f32 {
    p.length() - radius
}

pub fn plane(p : Vec3, n : Vec3, h : f32) -> f32 {
    p.dot(n) + h
}

pub fn map(pos : Vec3) -> f32 {
    f32::min(sphere(pos , 0.25), 
    plane(pos, vec3(0.0, 1.0, 0.0), 0.25))
}

pub fn get_normal(pos : Vec3) -> Vec3 {
    let e = vec2(0.0001, 0.0);
    return vec3(map(pos + e.xyy()) - map(pos - e.xyy()),
                map(pos + e.yxy()) - map(pos - e.yxy()),
                map(pos + e.yyx()) - map(pos - e.yyx()))
                .normalize();
}

pub fn soft_shadow(origin : Vec3, direction : Vec3 ) -> f32 {
    let mut res = 1.0;
    let mut t = 0.1;
    let mut ph = 1e20;

    while t < 8.0 {
        let h = map(origin + direction * t);

        if h < 0.0001 {
            return 0.0;
        }

        let y = h * h / (2.0 * ph);
        let d = f32::sqrt(h * h - y * y);
        res = f32::min(res, 4.0 * d / f32::max(0.0, t - y));
        ph = h;
        t += h;
    }

    res
}