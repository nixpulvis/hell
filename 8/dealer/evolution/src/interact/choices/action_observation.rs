use interact::*;

impl Choices<ActionChoice> for ActionObservation {
    fn choices(&self) -> Vec<ActionChoice> {
        vec![]
    }
}

// TODO: Beef these tests way up. They are now used for validation.
#[cfg(test)]
mod tests {
    // use game::*;
    // use interact::*;
    // use object::*;

}
