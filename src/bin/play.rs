use rustyfingers::game::Game;
use std::path::Path;

fn main() -> Result<(), failure::Error> {
    let stdout = std::io::stdout();
    let stdout = stdout.lock();

    let game = Game::from_file(stdout, Path::new("res/rust/top200"));

    if let Ok(mut game) = game {
        game.play()?;
    }

    Ok(())
}
