# `tesseract.rs`

A simple reimplementation of [@Mashpoe](https://github.com/Mashpoe)'s **ASCII Tesseract Rotation** program in Rust.

![A 2D ASCII projection of a four-dimensional hypercube, also called a tesseract.](/meta/tesseract_rs.gif)

You can find the original source code on GitHub [here](https://github.com/Mashpoe/hypercube), or the YouTube video where the author demonstrated the program [here](https://www.youtube.com/watch?v=48cz9sOd4s8). The original code is based on Steven Richard Hollasch's paper **Four-Space Visualization of 4D Objects**, which can be found [here](https://hollasch.github.io/ray4/Four-Space_Visualization_of_4D_Objects.html).


## Explanation

This program simulates a 4-dimensional hypercube (called a "tesseract") rotating in 4-dimensional space, which looks like the "inner cube" moving outward and stretching to become the "outer cube" (a gross oversimplification of 4-dimensional geometry).

The tesseract is then projected into 3-dimensional space, where it is rotated again. This is what causes the entire projection to look like it's spinning while it rotates through 4-dimensional space.

Finally, the tesseract is projected again, this time into 2-dimensional space, which is printed to the terminal with ASCII characters.


## Running `tesseract.rs`

Despite the tongue-and-cheek name of the repository, the actual code runs from a `main.rs` file like any regular Rust project created from running `cargo new`. Running the code is as simple as downloading the repository, navigating to the project folder in a terminal, and running `cargo run`. You may need to resize your terminal to fit the animation. To quit the animation, just use `ctrl + c` to end the program.

The majority of the code currently exists in a `tesseract.rs` file inside a `with_std` folder, which is imported into the `main.rs` file. Eventually, I'd like to reimplement this program using a linear algebra crate (e.g. `nalgebra`), which I'll put into a separate folder to differentiate the two, and possible provide individual `README` files for each. I've organized the file structure of this repository to make that addition easy, but at the moment, the organization is admittedly unnecessary.


## License

I am unaware of any licenses on the original code. Therefore, since I firmly believe that choosing any license is better than having no license at all, I have chosen to follow much of the Rust community and license this work under the dual MIT / Apache-2.0 license. If it comes to my attention that the original work is licensed in some way, I'll reevaluate this section and update it as appropriate.

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.