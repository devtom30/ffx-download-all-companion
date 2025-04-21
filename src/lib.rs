extern crate serde_json;

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use serde_json::Value;
use std::io;
use std::io::{Read, Write};

pub fn read_input<R: Read>(mut input: R) -> io::Result<serde_json::Value> {
    let length = input.read_u32::<NativeEndian>().unwrap();
    let mut buffer = vec![0; length as usize];
    input.read_exact(&mut buffer)?;
    let json_val: serde_json::Value = serde_json::from_slice(&buffer).unwrap();
    Ok(json_val)
}

pub fn write_output<W: Write>(mut output: W, value: &serde_json::Value) -> io::Result<()> {
    let msg = serde_json::to_string(value)?;
    let len = msg.len();
    // Chrome won't accept a message larger than 1MB
    if len > 1024 * 1024 {
        panic!("Message was too large, length: {}", len)
    }
    output.write_u32::<NativeEndian>(len as u32)?;
    output.write_all(msg.as_bytes())?;
    output.flush()?;
    Ok(())
}

#[derive(Debug, Clone)]
pub enum TaskType {
    PARSE,
    ATTACH
}

#[derive(Debug, Clone)]
pub struct Task {
    task_type: TaskType,
    url: String
}

impl TryFrom<Option<&str>> for TaskType {
    type Error = DESERIALIZATION_ERROR;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value { 
            Some("parse") => Ok(TaskType::PARSE),
            Some("attach") => Ok(TaskType::ATTACH),
            _ => Err(DESERIALIZATION_ERROR::UNKNOWN_TASK_TYPE)
        }
    }
}

pub enum DESERIALIZATION_ERROR {
    NO_TASK_TYPE,
    MISSING_FIELD,
    UNKNOWN_TASK_TYPE
}
impl TryFrom<&Value> for Task {
    type Error = DESERIALIZATION_ERROR;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(task_type) = value.get("task_type") {
            if let Ok(task_type) = TaskType::try_from(task_type.as_str()) {
                if let Some(url) = value.get("url") {
                    Ok(Self {
                        task_type: task_type,
                        url: url.to_string()
                    })
                } else {
                    Err(DESERIALIZATION_ERROR::MISSING_FIELD)
                }
            } else {
                Err(DESERIALIZATION_ERROR::UNKNOWN_TASK_TYPE)
            }
        } else {
            return Err(DESERIALIZATION_ERROR::NO_TASK_TYPE);
        }
    }
}