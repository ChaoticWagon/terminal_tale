use std::collections::VecDeque;
use std::io;
use std::path::Path;
use std::time::Duration;

use bevy::prelude::*;
use serde::{Deserialize, Deserializer};

#[derive(Resource, Default)]
pub struct ScriptDispatch {
    pub phases: VecDeque<Phase>,
    pub message_queue: VecDeque<(String, MessageMode, Timer)>,
    pub current_timer: Timer,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Phase {
    pub name: String, // name of the csv file
    #[serde(deserialize_with = "deserialize_command")]
    pub commands: VecDeque<Command>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub command: String,
    pub username: String,
    pub messages: Vec<Message>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub message: String,
    pub delay: f32,
    pub mode: MessageMode,
}

fn deserialize_command<'de, D>(deserializer: D) -> Result<VecDeque<Command>, D::Error>
where
    D: Deserializer<'de>,
{
    let commands: Vec<Command> = Deserialize::deserialize(deserializer)?;
    Ok(VecDeque::from(commands))
}


#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MessageMode {
    Print,
    Replace,
}

impl ScriptDispatch {
    pub(crate) fn new() -> io::Result<Self> {
        // read yaml files in scripts/*
        // get all filenames in scripts dir
        let path = Path::new("scripts");

        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "scripts directory not found"));
        }

        let mut phases: VecDeque<Phase> = VecDeque::new();

        for entry in path.read_dir()? {
            let entry = entry?;
            let f_type = entry.file_type()?;

            if f_type.is_dir() {
                continue;
            }

            let data = &std::fs::read_to_string(entry.path())?;
            let phase = serde_yaml::from_str::<Phase>(data);
            
            if let Ok(phase) = phase {
                phases.push_back(phase);
            } else {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid data in script file");
            }
        }

        Ok(Self {
            phases,
            message_queue: VecDeque::new(),
            current_timer: Timer::new(Duration::from_secs(0), TimerMode::Once),
        })
    }
}
// the ending of the story should not be a life lesson or a reflection, but an analogy, reflection, metaphor.