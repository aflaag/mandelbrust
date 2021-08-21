use criterion::{criterion_group, criterion_main, Criterion};
use rayon::{iter::{IndexedParallelIterator, ParallelIterator}, slice::ParallelSliceMut};
use mandelbrust::utils::*;

/// This benchmark shows the best approach
/// to render the Mandelbrot set in this
/// specific context, with the tools that
/// this version of `ggez` provides. The commented lines
/// show the other method that this one was compared with.
fn pixel_rendering() {
    // let _ = (0..H).into_par_iter().map(|y| {
    //     (0..W).into_par_iter().map(|x| {
    //         let pixel = Point::new((x, y));

    //         let mapped_pixel = pixel.into();

    //         let iter = MandelIter::new(mapped_pixel);

    //         let iterations = iter.enumerate().take_while(|(idx, _)| *idx <= ESCAPE_POINT).count();

    //         if iterations < ESCAPE_POINT {
    //             [0, 0, 0, 255]
    //         } else {
    //             [255, 255, 255, 255]
    //         }
    //     }).flatten().collect::<Vec<_>>()
    // }).flatten().collect::<Vec<_>>();

    let mut rgba = vec![0; H * W * 4];

    rgba.par_chunks_mut(W * 4).enumerate().for_each(|(y, chunks_row)| {
        let mut row = [0; W * 4];

        row.par_chunks_mut(4).enumerate().for_each(|(x, chunks_pixel)| {
            let pixel = Point::new((x, y));

            let mapped_pixel = pixel.into();

            let iter = MandelIter::new(mapped_pixel);

            let iterations = iter.enumerate().take_while(|(idx, _)| *idx <= ESCAPE_POINT).count();

            // no need to call the associated function,
            // just a showcase for the speed of the method
            let colored_pixel = if iterations < ESCAPE_POINT {
                [0, 0, 0, 255]
            } else {
                [255, 255, 255, 255]
            };

            chunks_pixel.iter_mut().zip(colored_pixel).for_each(|(ch, co)| *ch = co);
        });

        chunks_row.iter_mut().zip(row).for_each(|(ch, p)| *ch = p);
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pixel rendering", |b| b.iter(|| pixel_rendering()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);