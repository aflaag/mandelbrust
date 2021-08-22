use num::Complex;
use std::{fmt, ops};

/// The range of values of the x-axis of the Mandelbrot set.
const X_RANGE: (f32, f32) = (-2.0, 1.0);

/// The range of values of the y-axis of the Mandelbrot set.
const Y_RANGE: (f32, f32) = (-1.0, 1.0);

/// The length of the x-axis of the Mandelbrot set.
const X_DIFF: f32 = X_RANGE.1 - X_RANGE.0;

/// The length of the y-axis of the Mandelbrot set.
const Y_DIFF: f32 = Y_RANGE.1 - Y_RANGE.0;

/// The scaling factor, used to calculate `W` and `H`,
/// to make sure that the right proportions are mantained.
const SCALING_FACTOR: usize = 350;

/// The width of the window.
pub const W: usize = X_DIFF as usize * SCALING_FACTOR;

/// The height of the window.
pub const H: usize = Y_DIFF as usize * SCALING_FACTOR;

/// The value after which the points are no longer
/// iterated through the Mandelbrot set equation.
pub const ESCAPE_POINT: usize = 128;

/// A constant used to check if the cursor
/// is at the center of the Mandelbrot plane,
/// to avoid crashes while rendering the red line.
pub const CUSTOM_EPSILON: f32 = 0.065;

/// The color gradient used in the [Wikipedia page of
/// the Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set),
/// which seems to macth the color gradient used in Ultra Fractal.
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

/// The default position of the `Cursor`, which is `(0, 0)`.
pub const CURSOR_ZERO: Cursor = Cursor { coordinates: (0, 0) };

/// The default position of a point on
/// the Mandelbrot plane, which is `(0.0, 0.0)`.
pub const MANDELPOINT_ZERO: MandelPoint = MandelPoint { coordinates: (0.0, 0.0) };

/// The default position of a generic 2D point, which is `(0, 0)`.
pub const POINT_ZERO: Point = Point { coordinates: (0, 0) };

/// A trait implemented by any entity that
/// can be expressed using two coordinates.
pub trait Plottable {
    /// The associated type that represents
    /// the 2 coordinates of the 2D entity.
    type Coordinates;

    /// Returns a new instance of the 2D entity.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use mandelbrust::utils::{Plottable, Point};
    /// let point = Point::new((2, 6));
    /// ```
    fn new(coordinates: Self::Coordinates) -> Self;

    /// Returns the coordinates of the 2D entity.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use mandelbrust::utils::{Plottable, MandelPoint};
    /// let mandelpoint = MandelPoint::new((5.0, 3.0));
    /// 
    /// assert_eq!(mandelpoint.coordinates(), (5.0, 3.0));
    /// ```
    fn coordinates(&self) -> Self::Coordinates;

    /// Returns a mutable reference of the
    /// coordinates of the entity.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use mandelbrust::utils::{Plottable, Cursor};
    /// let mut cursor = Cursor::new((1, 8));
    /// 
    /// let (x, y) = cursor.coordinates_mut();
    /// 
    /// *x += 5;
    /// 
    /// assert_eq!(cursor.coordinates(), (6, 8));
    /// ```
    fn coordinates_mut(&mut self) -> &mut Self::Coordinates;

    /// A method used to update
    /// the coordinates of the entity.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use mandelbrust::utils::{Plottable, Point};
    /// let mut point = Point::new((2, 7));
    /// 
    /// point.update((3, 5));
    /// 
    /// assert_eq!(point.coordinates(), (3, 5));
    /// ```
    fn update(&mut self, coordinates: Self::Coordinates);

    /// A method that returns `true` if the
    /// distance between `self` and `other`
    /// is less than `distance`.
    ///
    /// # Examples
    /// 
    /// ```
    /// # pub use mandelbrust::utils::{Plottable, Cursor, Point};
    /// let cursor = Cursor::new((9, 9));
    /// let point = Point::default(); // (0, 0)
    /// 
    /// // note: using two 2D entities that have the same coordinates types
    /// assert!(cursor.is_distance_less_than(point, 13.0)) // 9.0 * f32::SQRT_2 < 13.0
    /// ```
    fn is_distance_less_than<P: Plottable<Coordinates = Self::Coordinates>>(&self, other: P, distance: f32) -> bool;
}

/// A macro used to implement:
/// - `Add<usize>`, `Sub<usize>`, `Mul<usize>` and `Div<usize>` for `Cursor`
/// - `Add<f32>`, `Sub<f32>`, `Mul<f32>` and `Div<f32>` for `MandelPoint`
/// - `Add<usize>`, `Sub<usize>`, `Mul<usize>` and `Div<usize>` for `Point`
macro_rules! impl_ops {
    ($struct:ty, $trait:ident, $type:ty, $op:tt, $func:ident) => {
        impl ops::$trait<$type> for $struct {
            type Output = Self;

            fn $func(self, other: $type) -> Self {
                let coordinates = self.coordinates();

                Self::new((coordinates.0 $op other, coordinates.1 $op other))
            }
        }
    };
}

/// A macro used to implement `Plottable`,
/// some `std::ops` traits, `Default` and `Display`
/// to `Cursor`, `MandelPoint` and `Point`.
macro_rules! impl_2d_entity {
    ($struct:ty, $type:ty, $const:ident) => {
        impl Plottable for $struct {
            type Coordinates = ($type, $type);

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

            fn is_distance_less_than<P: Plottable<Coordinates = Self::Coordinates>>(&self, other: P, distance: f32) -> bool {
                let coords_self = self.coordinates();
                let coords_other = other.coordinates();

                let x_diff = (coords_self.0 - coords_other.0) as f32;
                let y_diff = (coords_self.1 - coords_other.1) as f32;

                x_diff * x_diff + y_diff * y_diff < distance * distance
            }
        }

        impl_ops!($struct, Add, $type, +, add);
        impl_ops!($struct, Sub, $type, -, sub);
        impl_ops!($struct, Mul, $type, *, mul);
        impl_ops!($struct, Div, $type, /, div);

        impl Default for $struct {
            #[doc = "Returns `"]
            #[doc = stringify!($const)]
            #[doc = "` ."]
            fn default() -> Self {
                $const
            }
        }

        impl fmt::Display for $struct {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self.coordinates)
            }
        }
    };
}

/// A struct used to store the position
/// of the cursor on the screen.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cursor {
    coordinates: (usize, usize),
}

impl_2d_entity!(Cursor, usize, CURSOR_ZERO);

/// A struct used to represent any point
/// on the Mandelbrot plane.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MandelPoint {
    coordinates: (f32, f32),
}

impl_2d_entity!(MandelPoint, f32, MANDELPOINT_ZERO);

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

impl From<MandelPoint> for Complex<f32> {
    fn from(mandelpoint: MandelPoint) -> Self {
        let coordinates = mandelpoint.coordinates();

        Complex { re: coordinates.0, im: coordinates.1 }
    }
}

/// A struct used to represent a generic 2D point.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    coordinates: (usize, usize),
}

impl_2d_entity!(Point, usize, POINT_ZERO);

impl From<MandelPoint> for Point {
    fn from(mandelpoint: MandelPoint) -> Self {
        let coordinates = mandelpoint.coordinates();

        Point::new((
            (W as f32 * (coordinates.0 - X_RANGE.0) as f32 / X_DIFF as f32) as usize,
            (H as f32 * (coordinates.1 - Y_RANGE.0) as f32 / Y_DIFF as f32) as usize,
        ))
    }
}

/// An iterator that, at each step,
/// calculates the next point of the
/// equation of the Mandelbrot set
/// (`z = z^2 + c`, starting with `z = 0`).
/// `next()` returns `None` if the next value
/// is out of the area of radius 2.
/// 
/// # Examples
/// 
/// ```
/// # pub use mandelbrust::utils::{Plottable, MandelIter, MandelPoint};
/// let mandelpoint = MandelPoint::new((1.0, 1.0));
/// 
/// let mut iter = MandelIter::new(mandelpoint);
/// 
/// assert_eq!(iter.next(), Some(MandelPoint::new((1.0, 1.0)))); // at the beginning, `z = c`
/// assert_eq!(iter.next(), Some(MandelPoint::new((1.0, 3.0))));
/// assert_eq!(iter.next(), None); // the point exits from the area of radius 2
/// ```
pub struct MandelIter {
    curr: Complex<f32>,
    c: Complex<f32>,
}

impl MandelIter {
    /// Returns a new iterator of the Mandelbrot equation.
    /// At the beginning, `z = 0` and `c` is the given
    /// `MandelPoint`; then, the iteration proceeds
    /// with the formula `z = z^2 + c`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use mandelbrust::utils::{Plottable, MandelPoint, MandelIter};
    /// let mandelpoint = MandelPoint::new((0.2, 3.4));
    ///
    /// let mut iter = MandelIter::new(mandelpoint);
    /// ```
    pub fn new(mandel_c: MandelPoint) -> Self {
        Self {
            curr: Complex { re: 0.0, im: 0.0 },
            c: mandel_c.into(),
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

            Some(self.curr.into())
        }
    }
}