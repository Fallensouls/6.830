use super::predicate::Op;
use super::ty::Type;
use crate::downcast::Downcast;
use std::fs::File;
use std::io;

/**
 * Interface for values of fields in tuples in SimpleDB.
 */
pub trait Field: Downcast {
    /// Write the bytes representing this field to the specified file.
    fn serialize(&self, output: &mut File) -> io::Result<()>;

    /// Compare the value of this field object to the passed in value.
    fn compare(&self, op: Op, other: &Self) -> bool
    where
        Self: Sized;

    /// Returns the type of this field
    fn get_type(&self) -> Type;

    /**
     * Hash code.
     * Different Field objects representing the same value should probably
     * return the same hashCode.
     */
    fn hash_code(&self) -> u64;
    fn equals(&self, other: &Self) -> bool
    where
        Self: Sized;
    fn to_string(&self) -> String;
}
