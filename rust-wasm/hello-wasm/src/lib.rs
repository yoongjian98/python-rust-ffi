use fractal_gen::{fractal::Fractal, image::pixel::Pixel};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn fibonacci(nth: u64) -> u64 {
    fibonacci_impl(nth)
}

fn fibonacci_impl(nth: u64) -> u64 {
    if nth <= 2 {
        return 1;
    }

    fibonacci_impl(nth - 1) + fibonacci_impl(nth - 2)
}

#[wasm_bindgen]
pub fn draw_fractal() -> Vec<u8> {
    let mut pixels = vec![];

    // Height = 1000px
    for _i in 0..1000 {
        let mut row = vec![];
        // Width = 1000px
        for _j in 0..1000 {
            row.push(Pixel::new(255, 255, 255));
        }
        pixels.push(row);
    }
    // Create a Fractal from the pixels.
    let mut image = Fractal::new(pixels);

    // Draw the Mandelbrot Set on the image.
    image.mandelbrot(Pixel::new(250, 0, 0));

    // Draw a Sierpinksi Triangle on the image.
    image.sierpinski_triangle(180, 180, 100, Pixel::new(0, 0, 250));

    // Draw a Koch Curve on the image.
    image.koch_curve(675, 75, 925, 325, 5, Pixel::new(0, 250, 0));

    image.image.get_binary_data()
}
