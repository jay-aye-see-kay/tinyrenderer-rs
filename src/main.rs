use image::{imageops, ImageBuffer, Pixel, Rgb};

fn main() {
    let white = Rgb::from_channels(255, 255, 255, 255);
    let red = Rgb::from_channels(255, 0, 0, 255);

    let (width, height) = (100, 100);
    let image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let image = line(10, 10, 90, 90, image, red);
    let image = line(80, 20, 90, 90, image, white);

    // flip so 0,0 is in bottom left
    imageops::flip_horizontal(&image).save("out.png").unwrap();
}

fn line(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: Rgb<u8>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image = image.clone();
    for step in 0..100 {
        let t = f64::from(step) * 0.01;
        let x = f64::from(x0) + (f64::from(x1 - x0) * t);
        let y = f64::from(y0) + (f64::from(y1 - y0) * t);
        image.put_pixel(x as u32, y as u32, color);
    }
    image
}
