use std::collections::HashMap;

use alloy_primitives::{Uint, U256};

#[derive(Debug, Clone, PartialEq)]
pub enum Operations {
    MSTORE,
    MSTORE8,
    MLOAD,
    CALLDATACOPY,
    MSIZE,
    EXTCODECOPY,
    CODECOPY,
    RETURNDATACOPY,
    MCOPY,
    OTHER(String),
}

impl Operations {
    pub fn text(&self) -> &str {
        match self {
            Operations::MSTORE => "MSTORE",
            Operations::MSTORE8 => "MSTORE8",
            Operations::MLOAD => "MLOAD",
            Operations::MCOPY => "MCOPY",
            Operations::CALLDATACOPY => "CALLDATACOPY",
            Operations::MSIZE => "MSIZE",
            Operations::EXTCODECOPY => "EXTCODECOPY",
            Operations::CODECOPY => "CODECOPY",
            Operations::RETURNDATACOPY => "RETURNDATACOPY",
            Operations::OTHER(op) => op.as_str(),
        }
    }
    pub fn from_text(op: &str) -> Self {
        match op {
            "MSTORE" => Operations::MSTORE,
            "MSTORE8" => Operations::MSTORE8,
            "MLOAD" => Operations::MLOAD,
            "MCOPY" => Operations::MCOPY,
            "CALLDATACOPY" => Operations::CALLDATACOPY,
            "MSIZE" => Operations::MSIZE,
            "EXTCODECOPY" => Operations::EXTCODECOPY,
            "CODECOPY" => Operations::CODECOPY,
            "RETURNDATACOPY" => Operations::RETURNDATACOPY,
            _ => Operations::OTHER(op.to_string()),
        }
    }
    pub fn parse_args(&self, stack: Option<Vec<U256>>) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();
        let unwrapped_stack = stack.as_ref().unwrap();

        // handle other changes that are applied to the already existing slots
        match &self {
            Operations::MCOPY => {
                params.insert(
                    "Destination Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 1)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
                params.insert(
                    "Source Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 2)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
                params.insert(
                    "Byte Size".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 3)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
            }
            Operations::MSTORE => {
                params.insert(
                    "Destination Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 1)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
                params.insert(
                    "Value".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 2)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
            }
            Operations::EXTCODECOPY => {
                //params.insert("Address".to_string(), unwrapped_stack.get(unwrapped_stack.len() - 1).unwrap().to_string());
                params.insert(
                    "Destination Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 2)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
                params.insert(
                    "Source Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 3)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
                params.insert(
                    "Byte Size".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 4)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
            }
            Operations::CODECOPY => {
                params.insert(
                    "Destination Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 1)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
                params.insert(
                    "Source Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 2)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
                params.insert(
                    "Byte Size".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 3)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
            }
            Operations::RETURNDATACOPY => {
                params.insert(
                    "Destination Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 1)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
                params.insert(
                    "Source Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 2)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
                params.insert(
                    "Byte Size".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 3)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
            }
            Operations::MSTORE8 => {
                params.insert(
                    "Destination Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 1)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );

                params.insert(
                    "Value".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 2)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
            }
            Operations::MLOAD => {
                params.insert(
                    "Source Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 1)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
            }
            Operations::CALLDATACOPY => {
                params.insert(
                    "Destination Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 1)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
                params.insert(
                    "Source Offset".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 2)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
                params.insert(
                    "Byte Size".to_string(),
                    unwrapped_stack
                        .get(unwrapped_stack.len() - 3)
                        .unwrap()
                        .to::<u64>()
                        .to_string(),
                );
            }
            _ => {}
        };
        params
    }
}
