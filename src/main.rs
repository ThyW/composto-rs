mod backends;
mod focus;
mod state;
mod windowdata;

use crate::backends::{udev::run_udev, x11::run_x11};
use anyhow::Result;

const USAGE: &'_ str = "usage: composto {{x11 | udev}}";

fn main() -> Result<()> {
    let args: Vec<String> = ::std::env::args().collect();

    if let Some(arg) = args.get(1) {
        match arg.as_ref() {
            "x11" => run_x11()?,
            "udev" => run_udev()?,
            _ => eprintln!("{}", USAGE),
        }
    } else {
        eprintln!("{}", USAGE);
    }

    Ok(())
}
