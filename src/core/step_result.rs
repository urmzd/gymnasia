/// The result of a single environment step.
#[derive(Clone, Debug, PartialEq)]
pub struct StepResult<O> {
    /// The observation produced after the action was applied.
    pub observation: O,
    /// The scalar reward earned by the action.
    pub reward: f64,
    /// Whether the episode reached a terminal state.
    pub terminated: bool,
    /// Whether the episode was cut short (e.g. time limit).
    pub truncated: bool,
}
