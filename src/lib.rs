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

struct Line {
    start: Point<i32>,
    end: Point<i32>,
}

impl Line {
    fn new(p1: Point<i32>, p2: Point<i32>) -> Line {
        Line { start: p1, end: p2 }
    }

    fn points(&self) -> Vec<Point<i32>> {
        Bresenham::new(self.start, self.end).collect::<Vec<_>>()
    }

    fn mid(&self) -> Point<i32> {
        // Draw the line between the two points and take the middle one. This ensures the mid-point
        // is actually part of the drawn line.
        let points = self.points();
        points.get(points.len()/2).unwrap().clone()
    }
}


struct Triangle {
    p1: Point<i32>,
    p2: Point<i32>,
    p3: Point<i32>,
}


impl Triangle {
    fn new(p1: Point<i32>, p2: Point<i32>, p3: Point<i32>) -> Triangle {
        Triangle {
            p1: p1,
            p2: p2,
            p3: p3,
        }
    }
    fn lines(&self) -> Vec<Line> {
        vec![
            Line { start: self.p1, end: self.p2 },
            Line { start: self.p3, end: self.p2 },
            Line { start: self.p1, end: self.p3 }
        ]
    }
}

struct Image {
    img: ImageBuffer<Luma<u8>, Vec<u8>>
}

impl Image {
    fn new(size: u32) -> Image {
        // Create an ImageBuffer that is all white
        let white_point = image::Luma([255u8]);
        let mut _white_canvas = image::ImageBuffer::from_fn(size, size, |_x, _y| {
            white_point
        });
        Image { img: _white_canvas }
    }

    fn draw_vec(&mut self, vec: Vec<Point<i32>>) {
        let black_pixel: Luma<u8> = image::Luma([0u8]);
        for point in vec {
            let (x, y) = point;
            let x1 = x as u32;
            let y1 = y as u32;
            self.img.put_pixel(x1, y1, black_pixel);
        }
    }

    fn draw_line(&mut self, line: Line) {
        let points = line.points();
        Image::draw_vec(self, points);
    }

    fn draw_triangle(&mut self, triangle: &Triangle) {
        let lines = triangle.lines();
        for line in lines {
            Image::draw_line(self, line);
        }
    }
}

//pub fn make_line(start: Point<i32>, end: Point<i32>) -> Vec<Point<i32>> {
//    Bresenham::new(start, end).collect::<Vec<_>>()
//}
//
//#[test]
//fn test_make_line() {
//    assert_eq!(
//        draw_line((0,0), (5,5)),
//        [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]
//    )
//}


pub fn draw_image(size: u32, iteration_count: i32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut image = Image::new(size);

    // Create the initial triangle
    let size = size as i32 - 1 ;
    let triangle = Triangle::new(
        Line::new((0,0), (size,0)).mid(),
        (0, size),
        (size, size),
    );

    // Draw the triangle onto the canvas
    image.draw_triangle(&triangle);

    // Draw the sierpinski triangle
    draw_sierpinski(&mut image, iteration_count, triangle);

    image.img
}

fn draw_sierpinski(img: &mut Image, iteration_count: i32, triangle: Triangle) {
    if iteration_count == 0 {
        return;
    }

    let p12 = Line::new(triangle.p1, triangle.p2).mid();
    let p13 = Line::new(triangle.p1, triangle.p3).mid();
    let p23 = Line::new(triangle.p2, triangle.p3).mid();

    // Draw the inner triangle
    let inner_triangle = Triangle::new(p12, p13, p23);
    img.draw_triangle(&inner_triangle);

    // Go to next iteration
    let iteration_count = iteration_count - 1;
    draw_sierpinski(img, iteration_count, Triangle::new(triangle.p1, p12, p13));
    draw_sierpinski(img, iteration_count, Triangle::new(p12, triangle.p2, p23));
    draw_sierpinski(img, iteration_count, Triangle::new(p13, p23, triangle.p3));
}

pub fn write_to_file(img: ImageBuffer<Luma<u8>, Vec<u8>>, filename: &str) {
    let ref mut fout = File::create(&Path::new(filename)).unwrap();
    let _ = image::ImageLuma8(img).save(fout, image::PNG);
}

// Uses chaos method.
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