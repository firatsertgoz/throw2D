mod character;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use self::character::setup_character;

#[derive(Component)]
pub struct Character;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_character);
    }
}
