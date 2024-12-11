use std::io::stdin;
use image::{open, DynamicImage, GrayImage, Luma};

const ASCII_CHARS: [&str; 11] = ["@", "#", "S", "%", "?", "*", "+", ";", ":", ",", "."];

fn main() {
    let mut inputed_image_path = String::new();

    println!("Insira o nome e a extensão do arquivo");
    stdin().read_line(&mut inputed_image_path).expect("Erro");

    let img: DynamicImage = open(inputed_image_path.trim()).expect("Erro ao abrir a imagem");

    let gray_img: GrayImage = img.into_luma8();

    // Loop over each pixel and print its ASCII representation
    for y in 0..gray_img.height() {
        for x in 0..gray_img.width() {
            let pixel: &Luma<u8> = gray_img.get_pixel(x, y);
            let intesity = pixel[0] as usize; // The intensity ranges from 0-255
            let ascii_index = intesity * (ASCII_CHARS.len() - 1) / 255; // Scale to ASCII range

            if ascii_index > 11 {
                continue;
            }

            print!("{}", ASCII_CHARS[ascii_index]);
        }

        println!(); // New line at the end of each row
    }
}
