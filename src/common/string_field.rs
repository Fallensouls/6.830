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
 * Instance of Field that stores a single String of a fixed length.
 */
#[derive(Downcast)]
pub struct StringField {
    value: String,
    max_size: usize,
}

impl StringField {
    pub fn new(value: String, max_size: usize) -> Self {
        let value = if value.len() < max_size {
            value
        } else {
            value[0..max_size].to_string()
        };
        Self { value, max_size }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl Field for StringField {
    fn serialize(&self, output: &mut File) -> io::Result<()> {
        let overflow = self.max_size as i32 - self.value.len() as i32;
        let s = if overflow < 0 {
            self.value[0..self.max_size].to_string()
        } else {
            self.value.clone()
        };
        let l = s.len() as i32;
        output.write_all(l.to_string().as_bytes())?;
        output.write_all(s.as_bytes())?;
        if overflow > 0 {
            let zeros: Vec<u8> = vec![0; overflow as usize];
            output.write_all(&zeros)?;
        }
        Ok(())
    }

    fn compare(&self, op: Op, other: &Self) -> bool {
        match op {
            Op::Equals => self.value == other.value,
            Op::GreaterThan => self.value > other.value,
            Op::GreaterThanOrEq => self.value >= other.value,
            Op::LessThan => self.value < other.value,
            Op::LessThanOrEq => self.value <= other.value,
            Op::Like => self.value.contains(&other.value),
            Op::NotEquals => self.value != other.value,
        }
    }

    fn equals(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn get_type(&self) -> Type {
        Type::Str
    }

    fn to_string(&self) -> String {
        self.value.clone()
    }

    fn hash_code(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.value.hash(&mut s);
        s.finish()
    }
}
