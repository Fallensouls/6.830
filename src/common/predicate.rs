use super::field::Field;
use super::int_field::IntField;
use super::string_field::StringField;
use crate::table::tuple::Tuple;

/**
 * Predicate compares tuples to a specified Field value.
 */
struct Predicate<T: Field> {
    /// field number of passed in tuples to compare against
    field: i32,
    /// operation to use for comparison
    op: Op,
    /// field value to compare passed in tuples to
    operand: T,
}

impl<T> Predicate<T>
where
    T: Field,
{
    pub fn new(field: i32, op: Op, operand: T) -> Self {
        Self { field, op, operand }
    }

    pub fn get_operand(&self) -> Option<T> {
        // some code goes here
        None
    }

    pub fn get_field(&self) -> i32 {
        // some code goes here
        -1
    }

    pub fn get_op(&self) -> Option<Op> {
        // some code goes here
        None
    }

    pub fn filter(&self, t: Tuple) -> bool {
        // some code goes here
        false
    }

    pub fn to_string(&self) -> String {
        // some code goes here
        String::from("")
    }
}

// impl Predicate<IntField> {
//     pub fn new(field: i32, op: Op, operand: IntField) -> Self {
//         Self { field, op, operand }
//     }

//     pub fn get_operand(&self) -> Option<IntField> {
//         // some code goes here
//         None
//     }
// }

// impl Predicate<StringField> {
//     pub fn new(field: i32, op: Op, operand: StringField) -> Self {
//         Self { field, op, operand }
//     }

//     pub fn get_operand(&self) -> Option<StringField> {
//         // some code goes here
//         None
//     }
// }

pub enum Op {
    Equals,
    GreaterThan,
    LessThan,
    LessThanOrEq,
    GreaterThanOrEq,
    Like,
    NotEquals,
}

impl Op {
    pub fn get_op(i: i32) -> Option<Op> {
        match i {
            0 => Some(Self::Equals),
            1 => Some(Self::GreaterThan),
            2 => Some(Self::LessThan),
            3 => Some(Self::LessThanOrEq),
            4 => Some(Self::GreaterThanOrEq),
            5 => Some(Self::Like),
            6 => Some(Self::NotEquals),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Equals => String::from("="),
            Self::GreaterThan => String::from(">"),
            Self::LessThan => String::from("<"),
            Self::LessThanOrEq => String::from("<="),
            Self::GreaterThanOrEq => String::from(">="),
            Self::Like => String::from("LIKE"),
            Self::NotEquals => String::from("<>"),
        }
    }
}
