# Image to ASCII Converter

This is a command-line tool written in Rust that converts an image to ASCII art.

## Features

- Converts various image formats to ASCII art.
- Resizes the image for better representation.
- Supports common ASCII characters for the conversion.
- Outputs the ASCII art to the console.

## Usage

1. Make sure you have Rust and Cargo installed on your system.

2. Clone the repository:

git clone https://github.com/k0ez13/image-to-ascii.git

3. Navigate to the project directory:

cd image-to-ascii

4. Build the project:

cargo build --release

5. Run the tool with an image file as the argument:

cargo run --release path/to/image.jpg


Replace `path/to/image.jpg` with the actual path to your image file.

6. The ASCII representation of the image will be displayed in the console.

## Supported Image Formats

The tool supports the following image formats:

- BMP
- GIF
- JPEG
- PNG
- TIFF

Please note that additional dependencies are included to handle different formats.

## Customization

You can customize the ASCII characters used for the conversion by modifying the `ASCII_CHARS` array in the source code.

## License

This project is licensed under the [MIT License](LICENSE).





