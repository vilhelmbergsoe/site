---
title: Making an ASCII webcam for the console
date: 26-10-2022
archived: false
tags: [rust, asciicam, ascii, webcam]
---

![picture of my guitar in ASCII](/assets/pictures/ascii-guitar.webp)
*picture of my guitar in ASCII*

---

[This](https://www.youtube.com/watch?v=QMYfkOtYYlg) video recently popped up in
my recommended section, and it gave me a cool idea for a project.

The idea was to make a program that accesses your webcam, captures a grayscale
image and maps each luma pixel value to a character in a character set that
incrementally fills the font space and gives the illusion of a brighter pixel.

For example, a " " character would represent a darker pixel and "@" would
represent a brighter pixel.

I chose [Rust](https://www.rust-lang.org/) 🦀 for implementing the project,
mainly because I haven't had a chance to use it much, and I believed it would be
a fun learning experience.

The initial implementation of the program used a cross-platform webcam API
called [nokhwa](https://crates.io/crates/nokhwa). Sadly, compiling for macOS and
Windows didn't work, and the API design got in the way of some possible
optimizations I wanted to do.

So I finally decided to only support Linux for now and just used the
[v4l](https://crates.io/crates/v4l) (video 4 linux) crate for getting the webcam
frame buffers.

For profiling my application, I used
[flamegraph](https://github.com/flamegraph-rs/flamegraph). A [Flame
graph](https://www.brendangregg.com/flamegraphs.html) is a visualization of
hierarchical data, created to visualize stack traces of profiled software so
that the most frequent code-paths to be identified quickly and accurately.

This makes it more obvious what the main time-takers of your application are and
allows you to cut down their execution time.

For my initial implementation, the main culprit, taking up around 82 pct of the
program, was image resizing. That's because you have to downscale the camera
input to the size of the console, and only after can you map each pixel value to
an ASCII character. I used [image-rs's](https://crates.io/crates/image)
[resize_exact()](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.resize_exact),
with the Gaussian filter.

I eliminated a lot wasted time by using the
[fast_image_resize](https://crates.io/crates/fast_image_resize) crate with the
nearest image resampling filter.

This improved the performance greatly, but there were still unnecessary
time-takers I could cut down on…

As mentioned before, I switched to v4l in order to improve performance. I only
needed a grayscale image, but nokhwa only supported decompressing in the RGB
format. By using v4l and manually decompressing the MJPEG stream I could skip
the forward-backward which also helped out a good bit.

The last time-taker was writing the characters to the terminal buffer. I tried
different combinations of stdout being wrapped in a BufWriter, using a buffer
for each line, and finally just allocating enough space to fit the entire
terminal buffer. This also helped out a good deal and allowed you to have a
higher resolution "image" without noticable lag or scanlines.

## Conclusion

Benchmarking the program before and after the optimizations mentioned above and
a few more in a well lit room to account for auto exposure, I observed a 230%
increase in average fps, skyrocketing from 13 to 30. It is important to note
that 30 fps is the max that my webcam is rated for, and it's likely that it
could exceed 30 fps with a faster webcam.

Overall, this was a really fun project and I learned a ton about rust and
optimization techniques. I got to use cool tools in order to profile my
applications and observed significant improvement in fps which makes my efforts
worth it.

<!-- It would be cool to have an image of something from the ASCIICAM tool here -->

You can check out the repository
[here](https://github.com/vilhelmbergsoe/asciicam.git).


[^1]: hey there
