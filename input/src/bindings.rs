use bevy::prelude::*;
use super::action::Action;

/// 单个按键源（键盘或鼠标），直接用 Bevy 原生类型。
#[derive(Clone, Copy)]
pub enum Binding {
    Key(KeyCode),
    Mouse(MouseButton),
}

/// 默认按键映射表。
/// 修改按键 → 改这个函数 → 重新编译。
/// 后续要做运行时改键，把这份数据放进 InputMap Resource 即可。
pub fn default_bindings() -> Vec<(Action, Binding)> {
    use Action::*;
    use Binding::*;
    vec![
        (MoveUp,    Key(KeyCode::KeyW)),
        (MoveUp,    Key(KeyCode::ArrowUp)),
        (MoveDown,  Key(KeyCode::KeyS)),
        (MoveDown,  Key(KeyCode::ArrowDown)),
        (MoveLeft,  Key(KeyCode::KeyA)),
        (MoveLeft,  Key(KeyCode::ArrowLeft)),
        (MoveRight, Key(KeyCode::KeyD)),
        (MoveRight, Key(KeyCode::ArrowRight)),
        (Attack,    Mouse(MouseButton::Left)),
        (Attack,    Key(KeyCode::KeyJ)),
        (Dodge,     Key(KeyCode::ShiftLeft)),
        (Interact,  Key(KeyCode::KeyF)),
        (Pause,     Key(KeyCode::Escape)),
    ]
}