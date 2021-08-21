use num::Complex;
use std::fmt;

const X_RANGE: (f32, f32) = (-2.0, 1.0);
const Y_RANGE: (f32, f32) = (-1.0, 1.0);
const X_DIFF: f32 = X_RANGE.1 - X_RANGE.0;
const Y_DIFF: f32 = Y_RANGE.1 - Y_RANGE.0;

const SCALING_FACTOR: usize = 350;

pub const W: usize = X_DIFF as usize * SCALING_FACTOR;
pub const H: usize = Y_DIFF as usize * SCALING_FACTOR;
pub const ESCAPE_POINT: usize = 128;

/// A color gradient used in the Wikipedia page
/// of the Mandelbrot set, which seems to match
/// the color gradient used in Ultra Fractal.
/// 
/// (*Check [this](https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia)
/// Stack Overflow question for reference*).
pub const COLOR_MAP: [[u8; 4]; 16] = [
    [ 66,  30,  15, 255], // brown 3
    [ 25,   7,  26, 255], // dark violett
    [  9,   1,  47, 255], // darkest blue
    [  4,   4,  73, 255], // blue 5
    [  0,   7, 100, 255], // blue 4
    [ 12,  44, 138, 255], // blue 3
    [ 24,  82, 177, 255], // blue 2
    [ 57, 125, 209, 255], // blue 1
    [134, 181, 229, 255], // blue 0
    [211, 236, 248, 255], // lightest blue
    [241, 233, 191, 255], // lightest yellow
    [248, 201,  95, 255], // light yellow
    [255, 170,   0, 255], // dirty yellow
    [204, 128,   0, 255], // brown 0
    [153,  87,   0, 255], // brown 1
    [106,  52,   3, 255], // brown 2
];

pub trait Plottable {
    type Coordinates;

    fn new(coordinates: Self::Coordinates) -> Self;
    fn coordinates(&self) -> Self::Coordinates;
    fn coordinates_mut(&mut self) -> &mut Self::Coordinates;
    fn update(&mut self, coordinates: Self::Coordinates);
}

macro_rules! impl_plottable {
    ($struct:ty, $type:ty) => {
        impl Plottable for $struct {
            type Coordinates = $type;

            fn new(coordinates: Self::Coordinates) -> Self {
                Self { coordinates }
            }

            fn coordinates(&self) -> Self::Coordinates {
                self.coordinates
            }

            fn coordinates_mut(&mut self) -> &mut Self::Coordinates {
                &mut self.coordinates
            }

            fn update(&mut self, coordinates: Self::Coordinates) {
                self.coordinates = coordinates
            }
        }

        impl fmt::Display for $struct {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self.coordinates)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cursor {
    coordinates: (usize, usize)
}

impl_plottable!(Cursor, (usize, usize));

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MandelPoint {
    coordinates: (f32, f32)
}

impl_plottable!(MandelPoint, (f32, f32));

impl From<Complex<f32>> for MandelPoint {
    fn from(complex: Complex<f32>) -> Self {
        MandelPoint::new((complex.re, complex.im))
    }
}

impl From<Point> for MandelPoint {
    fn from(point: Point) -> Self {
        let coordinates = point.coordinates();

        MandelPoint::new((
            X_DIFF * coordinates.0 as f32 / W as f32 + X_RANGE.0,
            Y_DIFF * coordinates.1 as f32 / H as f32 + Y_RANGE.0
        ))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    coordinates: (usize, usize)
}

impl_plottable!(Point, (usize, usize));

impl From<MandelPoint> for Point {
    fn from(mandelpoint: MandelPoint) -> Self {
        let coordinates = mandelpoint.coordinates();

        Point::new((
            (W as f32 * (coordinates.0 - X_RANGE.0) as f32 / X_DIFF as f32) as usize,
            (H as f32 * (coordinates.1 - Y_RANGE.0) as f32 / Y_DIFF as f32) as usize,
        ))
    }
}

pub struct MandelIter {
    curr: Complex<f32>,
    c: Complex<f32>,
}

impl MandelIter {
    pub fn new(mandel_c: MandelPoint) -> Self {
        let c = mandel_c.coordinates();

        Self {
            curr: Complex { re: 0.0, im: 0.0 },
            c: Complex { re: c.0, im: c.1 },
        }
    }
}

impl Iterator for MandelIter {
    type Item = MandelPoint;

    fn next(&mut self) -> Option<Self::Item> {
        // checks if the distance between the origin
        // and the current point is more than 2
        if self.curr.re * self.curr.re + self.curr.im * self.curr.im > 4.0 {
            None
        } else {
            self.curr = self.curr * self.curr + self.c;

            let next = MandelPoint::from(self.curr);

            Some(next)
        }
    }
}