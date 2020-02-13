use super::int_field::IntField;
use super::string_field::StringField;

const STRING_LEN: i32 = 128;

/**
 * Enum representing a type in SimpleDB.
 */
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Type {
    Int,
    Str,
}

impl Type {
    pub fn len(&self) -> i32 {
        match self {
            Self::Int => 4,
            Self::Str => STRING_LEN + 4,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Int => String::from("Int"),
            Self::Str => String::from("Str"),
        }
    }

    pub fn parse_int() -> Result<IntField, &'static str> {
        Err("not implemented error")
    }

    pub fn parse_str() -> Result<StringField, &'static str> {
        Err("not implemented error")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_len() {
        assert_eq!(Type::Int.len(), 4);
        assert_eq!(Type::Str.len(), 128 + 4);
    }
}
