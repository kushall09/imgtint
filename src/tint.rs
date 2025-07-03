use image::{ImageBuffer, RgbImage, Rgb}; 

pub fn tintify(img:&RgbImage, rincr:u8, gincr:u8, bincr:u8) -> RgbImage {
    let (w,h)=img.dimensions();
    let mut tintimg=ImageBuffer::new(w,h);

    for x in 0..w {
        for y in 0..h {
            let pix=img.get_pixel(x, y);
            let Rgb([r, g, b])=*pix;
            let nr=r.saturating_add(rincr);
            let ng=g.saturating_add(gincr);
            let nb=b.saturating_add(bincr);
            tintimg.put_pixel(x, y, Rgb([nr, ng, nb]));
        }
    }
    tintimg
}
