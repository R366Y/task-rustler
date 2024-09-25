use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {

    pub fn next(&self) -> Self {
        match self {
            Priority::Low => Priority::Medium,
            Priority::Medium => Priority::High,
            Priority::High => Priority::Low,
        }
    }

    pub fn from_u8(value: u8) -> Option<Priority> {
        match value {
            0 => Some(Priority::Low),
            1 => Some(Priority::Medium),
            2 => Some(Priority::High),
            _ => None,
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Priority::Low => 0,
            Priority::Medium => 1,
            Priority::High => 2,
        }
    }
}

impl From<&str> for Priority {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "L" => Priority::Low,
            "M" => Priority::Medium,
            "H" => Priority::High,
            _ => unreachable!("Invalid priority",),
        }
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "L"),
            Priority::Medium => write!(f, "M"),
            Priority::High => write!(f, "H"),
        }
    }
}

impl PartialOrd<Self> for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_usize().cmp(&other.to_usize())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub priority: Priority,
}

impl Task {
    pub fn new(id: i32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
            priority: Priority::Medium,
        }
    }
}