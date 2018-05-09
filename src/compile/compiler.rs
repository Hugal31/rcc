#[derive(Debug, Default)]
pub struct Compiler {
    condition_counter: u64,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler::default()
    }

    /// Returns the number associated with the condition.
    ///
    /// Update the internal counter.
    pub fn new_condition(&mut self) -> u64 {
        let count = self.condition_counter;
        self.condition_counter += 1;
        count
    }
}
