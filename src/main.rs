use crate::rasterizer::rasterizer::Rasterizer;
use crate::rasterizer::example1;

mod matrix;
mod rasterizer;
mod geometry;

fn main() {
    let args = vec!["", "", "200.0", "output.png"];
    example1::draw_green_triangle(args);
}
