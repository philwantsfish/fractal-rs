extern crate image;
extern crate rand;
extern crate line_drawing;

use std::fs::File;
use rand::Rng;
use std::path::Path;

use image::ImageBuffer;
use image::Luma;

use line_drawing::Bresenham;
use line_drawing::Point;


pub fn print_something() {
    println!("Printing from lib.rs");
}

//pub fn sierpinski_triangle(width: i32, height: i32, iterations: i32) -> &ImageBuffer {
//
//}

pub fn make_line(start: Point<i32>, end: Point<i32>) -> Vec<Point<i32>> {
    Bresenham::new(start, end).collect::<Vec<_>>()
}

#[test]
fn test_make_line() {
    assert_eq!(
        draw_line((0,0), (5,5)),
        [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]
    )
}


fn draw_vec(img: &mut ImageBuffer<Luma<u8>, Vec<u8>>, vec: Vec<Point<i32>>) {
    let black_point = image::Luma([0u8]);

    for point in vec {
        let (x, y) = point;
        let x1 = x as u32;
        let y1 = y as u32;
        img.put_pixel(x1, y1, black_point);
    }
}

fn mid(start: Point<i32>, end: Point<i32>) -> Point<i32> {
    // Draw the line between the two points and take the middle one. This ensures the mid-point
    // is actually part of the drawn line.
    let line = make_line(start, end);
    line.get(line.len()/2).unwrap().clone()
}

pub fn draw_image(size: u32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let black_point = image::Luma([0u8]);
    let white_point = image::Luma([255u8]);

    // Get an ImageBuffer that is all white
    let mut img = image::ImageBuffer::from_fn(size, size, |x, y| {
        white_point
    });

    // Draw the initial triangle
    let size = size as i32 - 1 ;
    let top = mid((0,0), (size,0));
    let bottom_left = (0, size);
    let bottom_right = (size, size);

    let line1 = make_line(top, bottom_left);
    let line2 = make_line(top, bottom_right);
    let line3 = make_line(bottom_left, bottom_right);

    draw_vec(&mut img, line1);
    draw_vec(&mut img, line2);
    draw_vec(&mut img, line3);

    draw_sierpinski(&mut img, 10, top, bottom_left, bottom_right);

    img
}

fn draw_sierpinski(img: &mut ImageBuffer<Luma<u8>, Vec<u8>>, iteration_count: i32, top: Point<i32>, bottom_left: Point<i32>, bottom_right: Point<i32>) {
    if iteration_count == 0 {
        return;
    }

    let p1 = mid(top, bottom_left);
    let p2 = mid(top, bottom_right);
    let p3 = mid(bottom_left, bottom_right);

    let line1 = make_line(p1, p2);
    let line2 = make_line(p2, p3);
    let line3 = make_line(p1, p3);

    draw_vec(img, line1);
    draw_vec(img, line2);
    draw_vec(img, line3);

    let iteration_count = iteration_count - 1;
    draw_sierpinski(img, iteration_count, top, p1, p2);
    draw_sierpinski(img, iteration_count, p1, bottom_left, p3);
    draw_sierpinski(img, iteration_count, p2, p3, bottom_right);
}


pub fn write_to_file(img: ImageBuffer<Luma<u8>, Vec<u8>>, filename: &str) {
    let ref mut fout = File::create(&Path::new(filename)).unwrap();
    let _ = image::ImageLuma8(img).save(fout, image::PNG);
}

pub fn online_sierpinski_triangle() {
    /// Points used to build the fractal images
    pub struct Point {
        x: u32,
        y: u32,
    }

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let mut cnt: u32 = 10_000;

    let pts: [Point; 3] = [
        Point {x: WIDTH / 2, y: 0},
        Point {x: 0, y: HEIGHT},
        Point {x: WIDTH, y: HEIGHT},
    ];

    let mut num: usize;

    let mut p = Point { x: 0, y: 0 };
    let pixel = img[(0, 0)];

    while cnt > 0 {
        cnt = cnt - 1;
        num = rand::thread_rng().gen_range(0,3);
        p.x = (p.x + pts[num].x) / 2;
        p.y = (p.y + pts[num].y) / 2;
        img.put_pixel(p.x, p.y, pixel);
    }

    let ref mut fout = File::create(&Path::new("tri.png")).unwrap();
    let _ = image::ImageLuma8(img).save(fout, image::PNG);
}