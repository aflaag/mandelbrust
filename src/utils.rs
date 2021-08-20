use num::Complex;

const X_RANGE: (f32, f32) = (-2.0, 1.0);
const Y_RANGE: (f32, f32) = (-1.0, 1.0);
const X_DIFF: f32 = X_RANGE.1 - X_RANGE.0;
const Y_DIFF: f32 = Y_RANGE.1 - Y_RANGE.0;
const FACTOR: usize = 350;
pub const W: usize = X_DIFF as usize * FACTOR;
pub const H: usize = Y_DIFF as usize * FACTOR;
pub const ESCAPE_POINT: usize = 1024;

pub trait Plottable {
    type Coordinates;

    fn new(coordinates: Self::Coordinates) -> Self;
    fn coordinates(&self) -> Self::Coordinates;
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

            fn update(&mut self, coordinates: Self::Coordinates) {
                self.coordinates = coordinates
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    coordinates: (usize, usize)
}

impl Point {
    pub fn to_mandelpoint(&self, screen: (usize, usize)) -> MandelPoint {
        MandelPoint::new((
            X_DIFF * self.coordinates.0 as f32 / screen.0 as f32 + X_RANGE.0,
            Y_DIFF * self.coordinates.1 as f32 / screen.1 as f32 + Y_RANGE.0
        ))
    }
}

impl_plottable!(Point, (usize, usize));

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
    type Item = Complex<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        // if self.curr.re < X_RANGE.0 ||
        //    self.curr.re > X_RANGE.1 ||
        //    self.curr.im < Y_RANGE.0 ||
        //    self.curr.im > Y_RANGE.1
        // checks if the distance between the origin
        // and the current point is more than 2
        if self.curr.re * self.curr.re + self.curr.im * self.curr.im > 4.0 {
            None
        } else {
            self.curr = self.curr * self.curr + self.c;

            Some(self.curr)
        }
    }
}

// macro_rules! impl_fields {
//     ($field:ident, $type:ty) => {
//         pub fn $field(&self) -> $type {
//             self.$field
//         }
//     };
// }

// pub struct HSV {
//     h: u16,
//     s: u8,
//     v: u8,
// }

// impl HSV {
//     pub fn new(h: u16, s: u8, v: u8) -> Self {
//         Self { h, s, v }
//     }

//     pub fn to_rgb(&self) -> RGB {
//         let c = self.v as isize * self.s as isize;
//         let x = c * (1 - ((self.h as isize / 60) % 2 - 1).abs());
//         let m = self.v as isize - c as isize;

//         let triple = if self.h < 60 {
//             (c, x, 0)
//         } else if self.h < 120 {
//             (x, c, 0)
//         } else if self.h < 180 {
//             (0, c, x)
//         } else if self.h < 240 {
//             (0, x, c)
//         } else if self.h < 300 {
//             (x, 0, c)
//         } else if self.h < 360 {
//             (c, 0, x)
//         } else {
//             (0, 0, 0) // idk
//         };

//         println!("{:?}", triple);

//         RGB::new(
//             ((triple.0 + m) * 255).try_into().unwrap(),
//             ((triple.1 + m) * 255).try_into().unwrap(),
//             ((triple.2 + m) * 255).try_into().unwrap(),
//         )
//     }

//     impl_fields!(h, u16);
//     impl_fields!(s, u8);
//     impl_fields!(v, u8);
// }


// pub struct RGB {
//     r: u8,
//     g: u8,
//     b: u8,
// }

// impl RGB {
//     pub fn new(r: u8, g: u8, b: u8) -> Self {
//         Self { r, g, b }
//     }

//     impl_fields!(r, u8);
//     impl_fields!(g, u8);
//     impl_fields!(b, u8);
// }
