pub struct MyVec3 {
    x: f64,
    y: f64,
    z: f64
}

impl MyVec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }
}

pub fn write_color(pixel_color: MyVec3) {
    println!("{} {} {}", (255.999 * pixel_color.x) as i32, (255.999 * pixel_color.y) as i32, (255.999 * pixel_color.z) as i32);
}
