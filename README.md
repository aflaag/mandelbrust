# mandelbrust

A Mandelbrot set explorer written in Rust, using `ggez` as renderer.

**Note**: this program uses `ggez 0.5.1`, but the current latest version
is `0.6.0`, and this is due to a heavy drop in performance.

## TODO list

- [x] fix missing first point, the cursor
- [x] rename `Plane` to `MandelPlane`
- [x] invert x and y, lmao
- [x] add mandelbrot set
- [x] width and height as usize and const
- [x] rename "center" into "cursor"
- [x] fix color thing
- [x] color the mandelbrot set
- [x] fix the bottom right corner thing
- [x] fix inverted y
- [x] implement the line
- [ ] implement zoom (?)
- [ ] documentation
