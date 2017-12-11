/// A project to generate fractal images

extern crate triangle;

use std::env;

fn help() {
    println!("Expects 3 command line arguments:\n\
\tSize: The height and width of the image\n\
\tIterations: The number of iterations of the fractal\n\
\tFilename: The filename to save the image\n\
\n\
Example: ./triangle 800 6 sierpinski_6.png");
}

/// main projgram
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        help();
        return;
    }

    let size_argument = &args[1];
    let size: u32 = match size_argument.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            println!("Size arguments must be a number\n");
            help();
            return;
        }
    };

    let iterations_argument = &args[2];
    let iterations: i32 = match iterations_argument.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            println!("Iterations arguments must be a number\n");
            help();
            return;
        }
    };

    let filename = &args[3];

    println!("Screen size is {}, iterations is {}, filename {}", size, iterations, filename);

    let img = triangle::draw_image(size, iterations);
    triangle::write_to_file(img, filename);
}
