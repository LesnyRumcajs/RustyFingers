use rustyfingers::game::Game;
use std::path::Path;

fn main() -> Result<(), failure::Error> {
    let stdout = std::io::stdout();
    let stdout = stdout.lock();

    let mut game = Game::from_file(stdout, Path::new("res/eng/top1000"));

    if let Ok(mut game) = game {
        game.start();
    }

    Ok(())
}
