mod with_std { pub mod tesseract; }
use with_std::tesseract;

fn main() {
    tesseract::run_animation();
}
