pub mod  game;
pub mod  rl_agent;

use bevy::prelude::*;
use game::RocketGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RocketGamePlugin)
        .run();
}