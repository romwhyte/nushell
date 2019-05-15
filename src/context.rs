use crate::prelude::*;
use std::error::Error;

pub struct Context {
    commands: indexmap::IndexMap<String, Box<dyn crate::CommandBlueprint>>,
    crate host: Box<dyn crate::Host>,
    crate env: Environment,
}

impl Context {
    crate fn basic() -> Result<Context, Box<Error>> {
        Ok(Context {
            commands: indexmap::IndexMap::new(),
            host: Box::new(crate::env::host::BasicHost),
            env: crate::Environment::basic()?,
        })
    }

    pub fn add_commands(&mut self, commands: Vec<(&str, Box<dyn crate::CommandBlueprint>)>) {
        for (name, command) in commands {
            self.commands.insert(name.to_string(), command);
        }
    }

    crate fn has_command(&mut self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    crate fn create_command(
        &mut self,
        name: &str,
        arg_list: Vec<Value>,
    ) -> Result<Box<dyn Command>, ShellError> {
        match self.commands.get(name) {
            Some(command) => Ok(command.create(arg_list, &self.host, &mut self.env)?),
            None => Err(ShellError::string(format!("Missing command {}", name))),
        }
    }
}
