use std::collections::{HashMap, VecDeque};
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
    pub(crate) fn new(script_dir: HashMap<&Path, &[u8]>) -> Self {
        // read yaml files in scripts/*
        // get all filenames in scripts directory

        let mut phases: VecDeque<Phase> = VecDeque::new();
        
        let order = vec![
            "phase_1.yaml",
            "phase_2.yaml",
            "phase_3.yaml",
        ];

        for file in order {
            let path = Path::new(file);
            let data = script_dir.get(path).unwrap();
            let phase: Phase = serde_yaml::from_slice(data).unwrap();
            phases.push_back(phase);
        }

        Self {
            phases,
            message_queue: VecDeque::new(),
            current_timer: Timer::new(Duration::from_secs(0), TimerMode::Once),
        }
    }
}
// the ending of the story should not be a life lesson or a reflection, but an analogy, reflection, metaphor.