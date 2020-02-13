use super::record_id::RecordId;
use super::tuple_desc::TupleDesc;
use crate::common::field::Field;
use crate::downcast;
use std::slice::Iter;

/**
 * Tuple maintains information about the contents of a tuple. Tuples have a
 * specified schema specified by a TupleDesc object and contain Field objects
 * with the data for each field.
 */
pub struct Tuple {
    td: TupleDesc,
    fields: Vec<Box<dyn Field>>,
    // record_id: RecordId,
}

impl Tuple {
    pub fn new(td: TupleDesc) -> Self {
        Self {
            td,
            fields: vec![],
            // record_id,
        }
    }

    pub fn get_tuple_desc(&self) -> &TupleDesc {
        &self.td
    }

    // pub fn get_record_id(&self) -> &RecordId {
    //     &self.record_id
    // }

    pub fn set_tuple_desc(&mut self, td: TupleDesc) {
        self.td = td
    }

    // pub fn set_record_id(&mut self, rid: RecordId) {
    //     self.record_id = rid
    // }

    pub fn set_field(&mut self, i: usize, field: Box<dyn Field>) {
        if i > self.fields.len() {
            return;
        } else if i == self.fields.len() {
            self.fields.push(field);
        } else {
            self.fields[i] = field;
        }
    }

    pub fn get_field(&self, i: usize) -> Option<&Box<dyn Field>> {
        self.fields.get(i)
    }

    pub fn to_string(&self) -> String {
        "".to_string()
    }

    pub fn get_fields(&self) -> Iter<'_, Box<dyn Field>> {
        self.fields.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::int_field::IntField;
    use crate::common::ty::Type;

    fn get_type(len: usize) -> Vec<Type> {
        let mut types = Vec::with_capacity(len);
        for _ in 0..len {
            types.push(Type::Int);
        }
        types
    }

    fn get_string(len: usize, val: &str) -> Vec<String> {
        let mut strings = Vec::with_capacity(len);
        let combine = |i: usize| val.to_string() + i.to_string().as_str();
        for i in 0..len {
            strings.push(combine(i));
        }
        strings
    }

    #[test]
    fn test_modify_fields() {
        let td = TupleDesc::default_new(get_type(2));
        let mut tup = Tuple::new(td);
        tup.set_field(0, Box::new(IntField::new(-1)));
        tup.set_field(1, Box::new(IntField::new(0)));

        assert_eq!(
            IntField::new(-1),
            downcast!(tup.get_field(0).unwrap(), IntField)
        );
        assert_eq!(
            IntField::new(0),
            downcast!(tup.get_field(1).unwrap(), IntField)
        );

        tup.set_field(0, Box::new(IntField::new(1)));
        tup.set_field(1, Box::new(IntField::new(37)));

        assert_eq!(
            IntField::new(1),
            downcast!(tup.get_field(0).unwrap(), IntField)
        );
        assert_eq!(
            IntField::new(37),
            downcast!(tup.get_field(1).unwrap(), IntField)
        );
    }

    #[test]
    fn test_get_tuple_desc() {
        let td = TupleDesc::default_new(get_type(5));
        let tup = Tuple::new(td);
        assert_eq!(TupleDesc::default_new(get_type(5)), *tup.get_tuple_desc());
    }
}
