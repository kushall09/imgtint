use image;
mod tint;
mod gray;
fn main() -> Result<(), image::ImageError> {
    //println!("Hello, world!");
    let img=image::open("input.jpg")?.to_rgb8();
    let tinted=tint::tintify(&img,0,0,50);
    let grayed=gray::grayify(&img);
    tinted.save("tinted.jpg")?;
    grayed.save("grayed.jpg")?;
    println!("Images Processed and saved✅");
    Ok(())
}
