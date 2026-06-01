// bevy_input/src/lib.rs
pub mod action;
pub mod bindings;

pub use action::Action;
pub use bindings::Binding;

use bevy::prelude::*;
use bevy::input::ButtonInput;

#[derive(Resource)]
pub struct InputMap {
    pub bindings: Vec<(Action, Binding)>,
}

impl InputMap {
    pub fn rebind(&mut self, action: Action, bindings: Vec<Binding>) {
        self.bindings.retain(|(a, _)| *a != action);
        for b in bindings {
            self.bindings.push((action, b));
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputMap {
            bindings: bindings::default_bindings()
        })
            .init_resource::<ButtonInput<Action>>()
            .add_systems(PreUpdate, update_action_input);
    }
}

fn update_action_input(/* ... 和之前一样 ... */) { /* ... */ }

pub fn move_dir(actions: &ButtonInput<Action>) -> Vec2 { /* ... */ }