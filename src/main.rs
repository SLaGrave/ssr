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
            let r: f32 = (i as f32)/((IMAGEWIDTH-1) as f32);
            let g: f32 = (j as f32)/((IMAGEHEIGHT-1) as f32);
            let b: f32 = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Done.");
}
