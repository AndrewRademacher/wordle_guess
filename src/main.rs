use anyhow::Result;

use crate::app::App;

mod app;
mod dictionary;
mod guess;

fn main() -> Result<()> {
    App::new()?.run()
}
