pub mod action;
pub mod bindings;

pub use action::Action;
pub use bindings::Binding;

use bevy::prelude::*;
use bevy::input::ButtonInput;

/// 运行时按键映射表 Resource。
/// 改键功能直接修改这个 Resource 即可，无需重启。
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
        app
            .insert_resource(InputMap { bindings: bindings::default_bindings() })
            // 直接复用 Bevy 内置类型，它会自动维护 pressed/just_pressed/just_released
            .init_resource::<ButtonInput<Action>>()
            .add_systems(PreUpdate, update_action_input);
    }
}

/// 唯一的逻辑：每帧把原始输入翻译成 Action 输入。
/// Bevy 的 ButtonInput::press/release 会自动处理 just_pressed / just_released。
fn update_action_input(
    input_map: Res<InputMap>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse:    Res<ButtonInput<MouseButton>>,
    mut actions: ResMut<ButtonInput<Action>>,
) {
    // 必须调用 clear()，否则 just_pressed 会一直返回 true
    actions.clear();

    // 收集本帧每个 Action 的激活状态
    // 同一 Action 多个绑定，只要任一按下就算激活
    use std::collections::HashSet;
    let mut active: HashSet<Action> = HashSet::new();

    for (action, binding) in &input_map.bindings {
        let pressed = match binding {
            Binding::Key(k)   => keyboard.pressed(*k),
            Binding::Mouse(m) => mouse.pressed(*m),
        };
        if pressed {
            active.insert(*action);
        }
    }

    // 同步到 ButtonInput<Action>
    // press/release 自动处理 just_pressed / just_released 状态机
    let all_actions = [
        Action::MoveUp, Action::MoveDown,
        Action::MoveLeft, Action::MoveRight,
        Action::Attack, Action::Dodge,
        Action::Interact, Action::Pause,
    ];
    for action in all_actions {
        if active.contains(&action) {
            actions.press(action);
        } else {
            actions.release(action);
        }
    }
}

/// 辅助函数：从输入计算移动方向向量（归一化）。
/// 游戏逻辑 System 直接调用这个，不用自己写。
pub fn move_dir(actions: &ButtonInput<Action>) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if actions.pressed(Action::MoveUp)    { dir.y += 1.0; }
    if actions.pressed(Action::MoveDown)  { dir.y -= 1.0; }
    if actions.pressed(Action::MoveLeft)  { dir.x -= 1.0; }
    if actions.pressed(Action::MoveRight) { dir.x += 1.0; }
    if dir != Vec2::ZERO { dir.normalize() } else { dir }
}