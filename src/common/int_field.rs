use super::field::Field;
use super::predicate::Op;
use super::ty::Type;
use crate::downcast::Downcast;
use downcast_macro_derive::Downcast;
use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};

/**
 * Instance of Field that stores a single integer.
 */
#[derive(PartialEq, Debug, Downcast)]
pub struct IntField {
    value: i32,
}

impl IntField {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl Field for IntField {
    fn serialize(&self, output: &mut File) -> io::Result<()> {
        output.write_all(self.to_string().as_bytes())?;
        Ok(())
    }

    fn compare(&self, op: Op, other: &Self) -> bool {
        match op {
            Op::Equals => self.value == other.value,
            Op::GreaterThan => self.value > other.value,
            Op::GreaterThanOrEq => self.value >= other.value,
            Op::LessThan => self.value < other.value,
            Op::LessThanOrEq => self.value <= other.value,
            Op::Like => self.value == other.value,
            Op::NotEquals => self.value != other.value,
        }
    }

    fn equals(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn get_type(&self) -> Type {
        Type::Int
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn hash_code(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.value.hash(&mut s);
        s.finish()
    }
}
