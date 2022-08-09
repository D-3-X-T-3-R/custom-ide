#[derive(Debug, Clone)]
pub struct Command {
    pub operation: String,
    pub operand_1: Option<String>,
    pub operand_2: Option<String>,
}
pub fn validate_command(val: String) -> Result<Command, String> {
    let cmd: Vec<&str> = val.trim().split(' ').collect();
    match cmd.len() {
        1 => {
            return Ok(Command::new(cmd[0].trim().to_lowercase(), None, None));
        }
        2 => {
            return Ok(Command::new(
                cmd[0].trim().to_lowercase(),
                Some(cmd[1].trim().to_string()),
                None,
            ));
        }
        3 => {
            return Ok(Command::new(
                cmd[0].trim().to_lowercase(),
                Some(cmd[1].trim().to_string()),
                Some(cmd[2].trim().to_string()),
            ));
        }
        _ => Err(format!("Invalid command: {}", val)),
    }
}

impl Command {
    fn new(operation: String, operand_1: Option<String>, operand_2: Option<String>) -> Self {
        Self {
            operation,
            operand_1,
            operand_2,
        }
    }
}
