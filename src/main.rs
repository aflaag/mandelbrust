#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

use std::convert::TryInto;

use mandelbrust::utils::*;
use ggez::{Context, ContextBuilder, GameResult, conf, event, graphics::{self, DrawParam}, input::mouse};
use rayon::{iter::{IndexedParallelIterator, ParallelIterator}, slice::ParallelSliceMut};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct MandelPlane<const W: usize, const H: usize> {
    cursor: Cursor,
}

impl<const W: usize, const H: usize> MandelPlane<W, H> {
    fn new() -> GameResult<MandelPlane<W, H>> {
        Ok(Self {
            cursor: Cursor::new((0, 0))
        })
    }

    fn map_color(iterations: usize) -> [u8; 4] {
        // if iterations < ESCAPE_POINT {
        //     [0, 0, 0, 255]
        // } else {
        //     [255, 255, 255, 255]
        // }

        // let hue = 255 * iterations / ESCAPE_POINT;
        // let saturation = 255;
        // let value = if iterations == ESCAPE_POINT { 255 } else { 0 };

        // let hsv = HSV::new(hue.try_into().unwrap(), saturation, value);

        // let rgb = hsv.to_rgb();

        // [rgb.r(), rgb.g(), rgb.b(), 255]

        if iterations < ESCAPE_POINT / 512 {
            [255, 255, 255, 255]
        } else if iterations < ESCAPE_POINT / 300 {
            [150, 150, 150, 255]
        } else if iterations < ESCAPE_POINT / 256 {
            [128, 128, 128, 255]
        } else if iterations < ESCAPE_POINT / 128 {
            [64, 64, 64, 255]
        } else if iterations < ESCAPE_POINT / 64 {
            [32, 32, 32, 255]
        } else if iterations < ESCAPE_POINT / 16 {
            [16, 16, 16, 255]
        } else {
            [0, 0, 0, 255]
        }
    }
}

impl<const W: usize, const H: usize> event::EventHandler for MandelPlane<W, H>
where
    [(); H * W * 4]: ,
    [(); W * 4]: ,
{
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // let coords = mouse::position(ctx);

        // self.cursor.update((coords.x as usize, coords.y as usize));

        println!("refresh");

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // draw background
        graphics::clear(ctx, graphics::BLACK);

        // build the Mandelbrot set
        let mut rgba = vec![0; H * W * 4];

        rgba.par_chunks_mut(W * 4).enumerate().for_each(|(y, chunks_row)| {
            let mut row = [0; W * 4];

            row.par_chunks_mut(4).enumerate().for_each(|(x, chunks_pixel)| {
                let pixel = Point::new((x, y));

                let mapped_pixel = pixel.to_mandelpoint((W, H));

                let iter = MandelIter::new(mapped_pixel);

                let iterations = iter.enumerate().take_while(|(idx, _)| *idx <= ESCAPE_POINT).count();

                let colored_pixel = MandelPlane::<W, H>::map_color(iterations);

                chunks_pixel.iter_mut().zip(colored_pixel).for_each(|(ch, co)| *ch = co);
            });

            chunks_row.iter_mut().zip(row).for_each(|(ch, p)| *ch = p);
        });

        let screen = graphics::Image::from_rgba8(ctx, W.try_into().unwrap(), H.try_into().unwrap(), &rgba).unwrap();

        graphics::draw(ctx, &screen, DrawParam::default())?;

        // let cursor = self.cursor.coordinates();

        // invert the y coordinate of the center to preserve
        // the canonical orientation of the axis of the Mandelbrot
        // set (in the case of the mandelbrot set visually
        // nothing changes since the fractal is symmetric
        // with respect to the y-axis)
        // let inverted_cursor = Point::new((cursor.0, self.height - cursor.1));

        // map the position of the cursor
        // to a point in the Mandelbrot plane
        // let mapped_center = inverted_cursor.to_mandelpoint((self.width, self.height));

        // let iter = MandelIter::new(mapped_center);

        // let mut points: Vec<na::Point2<f32>> = vec![na::Point2::new(self.center.0, self.center.1)];
        
        // let mut last = (0.0, 0.0); // debug

        // for (idx, next_mapped) in iter.enumerate() {
        //     // there must be a maximum value
        //     // of plotted segments
        //     if idx == 256 {
        //         break;
        //     }

        //     // map the next point of the mandelbrot plane
        //     // to a position on the screen
        //     let mut next = self.mandel_to_screen((next_mapped.re, next_mapped.im));
        //     // println!("CENTER = {:?}", self.center);
            
        //     next.0 += self.center.0;
        //     next.1 += self.center.1;
        //     // next.1 += self.height - self.center.1;
        //     // next.1 += self.center.1;
        //     // point.1 += self.height - self.center.1;
        //     // point.1 = -point.1;
        //     // point.1 += self.height - self.center.1;

        //     // println!("CENTER = {:?}, POINT = {:?}", mapped_center, point);

        //     // if (point.0 - self.center.0).abs() < f32::EPSILON && (point.1 - self.center.1).abs() < f32::EPSILON && !points.is_empty() {
        //     //     println!("REACHED");
        //     //     break;
        //     // }

        //     last = next;
        //     // push the point
        //     points.push(na::Point2::new(next.0, next.1));
        // }
        // // let last = (points.iter().last().unwrap().coords.data.)
        // println!("{:?}", self.screen_to_mandel(last)); // debug
        // // println!("{:?}", self.screen_to_mandel(last);

        // println!("{}", points.len());
        // let line = graphics::Mesh::new_line(ctx, &points, 1.0, graphics::WHITE)?; // TODO: red

        // // TODO: what the fuck is this
        // graphics::draw(ctx, &line, (na::Point2::new(0.0, 0.0),))?;
        graphics::present(ctx)?;
        
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("MandelbRust", "ph04")
        .window_setup(conf::WindowSetup {
            title: "MandelbRust".to_owned(),
            samples: conf::NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        }).window_mode(conf::WindowMode {
            width: W as f32,
            height: H as f32,
            maximized: false,
            fullscreen_type: conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: false,
        });

    let (ctx, event_loop) = &mut cb.build()?;
    
    let state = &mut MandelPlane::<W, H>::new()?;
    
    event::run(ctx, event_loop, state)
}