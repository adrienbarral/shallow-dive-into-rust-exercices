use std::{env::args, path::{Path}};
use anyhow::Result;
use image::DynamicImage;

// this function open the "source_file", apply grayscale on it and save the result in the destination file.
// Replace "xxxxx" by the appropriate type. We will handle errors in the main function.
// To open the image use the method "open" in the crate image.
// The "grayscale" method on the DynamicImage will convert the image in grayscale
// Then, save with the "save_with_format" method of dynamic image.
fn open_gray_and_save(source_file: &Path, destination_file: &Path) -> xxxxx {

}
fn main() -> Result<()> {

    let src_file = args().nth(1);
    let dst_file = args().nth(2);

    // check that src and dst file contains data, if not abort the application by returning an error from the main function.
    // In this case, you can return an error thanks to the bail! macro (https://docs.rs/anyhow/latest/anyhow/index.html)

    
    // You can create a Path from a String, thanks to : Path::new(&src_file)

    Ok(())
}