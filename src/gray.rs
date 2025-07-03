use image::{ImageBuffer, RgbImage, Rgb};

pub fn grayify(img: &RgbImage) -> RgbImage {
    let (w,h)=img.dimensions();
    let mut gimg=ImageBuffer::new(w,h);
    for x in 0..w{
        for y in 0..h{
            let pix=img.get_pixel(x,y);
            let Rgb([r,g,b])=*pix;
            let gray=((r as u16+g as u16+b as u16)/3) as u8;
            gimg.put_pixel(x,y,Rgb([gray,gray,gray]));
        }
    }
    gimg
}
