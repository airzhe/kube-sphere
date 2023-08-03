use serde::Deserialize;

/// Parameters for executing a command in a specified container
#[derive(Deserialize)]
pub struct ExecParams {
    /// The target container's name, if specific container is required
    pub container: Option<String>,
    /// The commands to be executed
    pub commands: Vec<String>,
}
