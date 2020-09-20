mod color;
mod vec3;

use color::*;
use std::io::Write;
use std::io::{stderr, stdout};

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush()?;

        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );
            write_color(&mut stdout(), pixel_color)?;
        }
    }

    eprintln!("\nDone.");

    Ok(())
}
