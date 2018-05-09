use compile::scope::{Scope, Variable};

#[derive(Debug, Default)]
pub struct Context<'a> {
    scope: Scope<'a>,
    condition_counter: u64,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context::default()
    }

    /// Declare a variable in the context
    pub fn add_variable(&mut self, name: &str) {
        self.scope.add_variable(Variable::new(name))
    }

    /// Says if a variable name is already defined in the current scope.
    pub fn variable_is_defined(&self, name: &str) -> bool {
        self.scope.contains(name)
    }

    pub fn get_variable_index(&self, name: &str) -> Option<usize> {
        self.scope.get_variable_index(name)
    }

    pub fn get_scope_size(&self) -> usize {
        self.scope.get_size()
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
