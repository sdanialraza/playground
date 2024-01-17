use image::{DynamicImage, GenericImageView, ImageBuffer, Luma};
use std::path::Path;

fn main() {
    let image_path = Path::new("assets/port.jpg");

    let image = image::open(image_path).expect("Unable to open image");
    let grayscale_image = convert_to_grayscale(image);

    grayscale_image
        .save("assets/port_grayscale.jpg")
        .expect("Unable to save image");
}

fn convert_to_grayscale(image: DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();

    let mut grayscale_image = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel = image.get_pixel(x, y).0;

            let [red, green, blue, _] = pixel;

            let average = ((red as f32 + green as f32 + blue as f32) / 3.0).round() as u8;

            grayscale_image.put_pixel(x, y, Luma([average]));
        }
    }

    grayscale_image
}
