# Test job, the Rust part

This applicaton should download images from the linebreak-separated text file of the urls.

One line represents one url.

The application should download the images with GET request.

Downloaded images should be named locally as `<number-in-the-list>-<hash-of-the-url>.<extension>`.

Doubles should not be processed.

## How to use

`cargo run [--file <source_file>] [--outdir <out_dir>] [--threads <threads>]`

By default source file is [turtles.txt](turtles.txt), out dir is `out` (will be created automatically), threads number is 5.
