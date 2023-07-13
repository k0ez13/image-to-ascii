use std::env;
use image::GenericImageView;

const ASCII_CHARS: [char; 11] = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.'];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide an image file as an argument.");
        return;
    }

    let image_path = &args[1];
    let img = match image::open(image_path) {
        Ok(img) => img,
        Err(err) => {
            println!("Failed to open image file: {}", err);
            return;
        }
    };

    let (width, height) = img.dimensions();
    let img = img.resize_exact(width / 2, height / 2, image::imageops::FilterType::Gaussian);

    let grayscale = img.to_luma8();
    let pixels = grayscale.into_raw();

    for (i, pixel) in pixels.iter().enumerate() {
        let pixel_char = ASCII_CHARS[*pixel as usize * ASCII_CHARS.len() / 256];
        print!("{}", pixel_char);

        if (i + 1) % (width as usize / 2) == 0 {
            println!();
        }
    }
}


