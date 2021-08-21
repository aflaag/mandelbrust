#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

use std::{convert::TryInto};

use ggez::{Context, ContextBuilder, GameResult, conf, event, graphics::{self, Color, DrawParam}, input::mouse, nalgebra::Point2};
use rayon::{iter::{IndexedParallelIterator, ParallelIterator}, slice::ParallelSliceMut};
use mandelbrust::utils::*;

const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

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
        let idx = iterations % 16;

        COLOR_MAP[idx]
    }
}

impl<const W: usize, const H: usize> event::EventHandler for MandelPlane<W, H>
where
    [(); H * W * 4]: ,
    [(); W * 4]: ,
{
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let coords = mouse::position(ctx);

        let x = coords.x as usize;
        let y = coords.y as usize;

        // update the cursor position
        // if the cursor is not out of
        // the boundaries of the window
        if x < W && y < H {
            self.cursor.update((x, y));
        }

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

                let mapped_pixel = pixel.into();

                let iter = MandelIter::new(mapped_pixel);

                let iterations = iter.enumerate().take_while(|(idx, _)| *idx <= ESCAPE_POINT).count();

                let colored_pixel = MandelPlane::<W, H>::map_color(iterations);

                chunks_pixel.iter_mut().zip(colored_pixel).for_each(|(ch, co)| *ch = co);
            });

            chunks_row.iter_mut().zip(row).for_each(|(ch, p)| *ch = p);
        });

        // create the image of the Mandelbrot set
        let screen = graphics::Image::from_rgba8(ctx, W.try_into().unwrap(), H.try_into().unwrap(), &rgba).unwrap();

        let cursor = self.cursor.coordinates();

        // invert the y coordinate of the center to preserve
        // the canonical orientation of the axis of the Mandelbrot
        // set (in the case of the Mandelbrot set visually
        // nothing changes since the fractal is symmetric
        // with respect to the x-axis)
        let inverted_cursor = Point::new((cursor.0, H - cursor.1));

        // map the position of the cursor
        // to a point in the Mandelbrot plane
        // let mapped_cursor = inverted_cursor.to_mandelpoint();
        let mapped_cursor = inverted_cursor.into();

        let iter = MandelIter::new(mapped_cursor);

        // build the set of points for the segments
        let mut points = vec![Point2::new(cursor.0 as f32, cursor.1 as f32)];
        
        for (idx, next_mapped) in iter.enumerate() {
            // there must be a maximum value of plotted segments
            if idx == ESCAPE_POINT {
                break;
            }

            // remap the value back to the screen
            let mut next: Point = next_mapped.into();
            
            let (x, y) = next.coordinates_mut();

            // invert the position to map correctly
            // the point on the screen
            *y = H - *y;

            points.push(Point2::new(*x as f32, *y as f32));
        }

        // build the line
        let line = graphics::Mesh::new_line(ctx, &points, 1.0, RED)?;

        // draw the fractal
        graphics::draw(ctx, &screen, DrawParam::default())?;

        // draw the line
        graphics::draw(ctx, &line, DrawParam::default())?;

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