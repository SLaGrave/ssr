// Vector3 Import code and utility
mod my_vec3;
use my_vec3::MyVec3 as MyPoint3;
use my_vec3::MyVec3 as MyColor;
use my_vec3::write_color;

fn main() {

    // Image
    const IMAGEWIDTH: i32 = 256;
    const IMAGEHEIGHT: i32 = 256;

    // Render
    println!("P3");
    println!("{} {}", IMAGEWIDTH, IMAGEHEIGHT);
    println!("255");

    for j in (0..IMAGEHEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGEWIDTH {
            let tmp = MyColor::new((i as f64)/((IMAGEWIDTH-1) as f64), (j as f64)/((IMAGEHEIGHT-1) as f64), 0.25);
            write_color(tmp);
        }
    }
    eprintln!("Done.");
}
