#[derive(Debug, Clone, PartialEq)]
pub enum TimerStateAction {
    Stop,
    Start,
}

impl Default for TimerStateAction {
    fn default() -> Self {
        Self::Stop
    }
}
