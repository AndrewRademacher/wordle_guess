use anyhow::Result;

use wordle_guess::app::App;

fn main() -> Result<()> {
    App::new()?.run()
}
