extern crate piston_window;
extern crate find_folder;

mod game;

use piston_window::*;

fn load_image(window: &mut PistonWindow, folder: &str, file: &str) -> Result<G2dTexture, &'static str> {
    let asset_folder = match find_folder::Search::ParentsThenKids(3, 3).for_folder(folder) {
        Ok(f) => f,
        Err(find_folder::Error::IO(_)) => return Err("IO error on trying to find asset folder"),
        Err(find_folder::Error::NotFound) => return Err("Could not find asset folder")
    };

    let asset_path = asset_folder.join(file);
    
    match Texture::from_path(&mut window.factory, &asset_path, Flip::None, &TextureSettings::new()) {
        Ok(t) => Ok(t),
        Err(_) => Err("Something went wrong opening the asset")
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("rocket-game", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let texture = load_image(&mut window, "assets", "rocket.png").unwrap();
    let mut game = game::Game::new();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            game.draw(context, graphics);
        });
    }
}
