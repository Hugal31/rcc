#[derive(Debug)]
pub struct Variable {
    pub name: String,
}

impl Variable {
    pub fn new(name: &str) -> Variable {
        Variable {
            name: name.to_owned(),
        }
    }

    /// Return the size in bytes
    pub fn get_size(&self) -> usize {
        4
    }
}

#[derive(Debug, Default)]
pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: Vec<Variable>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Scope<'a> {
        Scope::default()
    }

    #[allow(dead_code)]
    pub fn new_child(&'a self) -> Scope<'a> {
        Scope {
            parent: Some(self),
            variables: Vec::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        self.get_parent_size() + self.variables.iter().map(|v| v.get_size()).sum::<usize>()
    }

    fn get_parent_size(&self) -> usize {
        self.parent.map(|s| s.get_size()).unwrap_or(0)
    }

    pub fn add_variable(&mut self, variable: Variable) {
        self.variables.push(variable);
    }

    pub fn contains(&self, name: &str) -> bool {
        self.parent.map(|s| s.contains(name)).unwrap_or(false)
            || self.variables.iter().any(|v| v.name == name)
    }

    #[allow(dead_code)]
    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables
            .iter()
            .find(|v| v.name == name)
            .or_else(|| self.parent.map(|s| s.get_variable(name)).unwrap_or(None))
    }

    pub fn get_variable_index(&self, name: &str) -> Option<usize> {
        let mut index = self.get_parent_size();
        for var in &self.variables {
            if var.name == name {
                return Some(index);
            }
            index += var.get_size();
        }

        self.parent
            .map(|s| s.get_variable_index(name))
            .unwrap_or(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_variable_in_simple_scope() {
        let mut scope = Scope::new();
        scope.add_variable(Variable::new("a"));
        scope.add_variable(Variable::new("b"));
        assert_eq!(scope.get_variable_index("a"), Some(0));
        assert_eq!(scope.get_variable_index("b"), Some(4));
        assert_eq!(scope.get_variable_index("c"), None);
        assert!(scope.contains("a"));
        assert!(!scope.contains("c"));
    }

    #[test]
    fn test_variable_super_scope() {
        let mut parent_scope = Scope::new();
        parent_scope.add_variable(Variable::new("a"));
        parent_scope.add_variable(Variable::new("b"));
        let mut child_scope = parent_scope.new_child();
        child_scope.add_variable(Variable::new("a"));
        assert!(child_scope.contains("a"));
        assert!(child_scope.contains("b"));
        assert_eq!(child_scope.get_variable_index("a"), Some(8));
        assert_eq!(child_scope.get_variable_index("b"), Some(4));
    }
}
