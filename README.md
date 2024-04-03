# OpenCV Rust Line Splitting

This is a simple port of the C++ version of the line splitting code of OpenCV.

It takes a text with multiple lines and write in the `output` folder the separate lines. (`src/lines.rs`)
Another version using bounding box is also provided, this version could in theory produce better results, and the rectangle returned can be used to embed the coordinates of the text in a PDF for example. (`src/main.rs`)
