#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    InProgress,
    GameOver,
    Paused,
}
