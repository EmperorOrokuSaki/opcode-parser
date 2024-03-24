use std::collections::HashMap;

use alloy_primitives::U256;

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
}

pub fn parse_args(opcode: &Operations, stack: Option<Vec<U256>>) -> HashMap<String, String> {
    HashMap::new()
}
