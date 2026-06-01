/// 游戏所有可触发的抽象动作。
/// 实现 Eq + Hash 才能作为 ButtonInput<Action> 的索引。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Attack,
    Dodge,
    Interact,
    Pause,
}