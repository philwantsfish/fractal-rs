/// A project to generate fractal images

extern crate triangle;


/// main projgram
fn main() {
    let img = triangle::draw_image(800);
    triangle::write_to_file(img, "line.png");
}
