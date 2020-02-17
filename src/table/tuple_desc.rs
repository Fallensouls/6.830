use crate::common::ty::Type;
use std::fmt;
use std::slice::Iter;

#[derive(PartialEq, Debug, Clone)]
pub struct TDItem {
    field_name: String,
    field_type: Type,
}

impl TDItem {
    pub fn new(t: Type, n: String) -> Self {
        Self {
            field_type: t,
            field_name: n,
        }
    }
}

impl fmt::Display for TDItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.field_name, self.field_type.to_string())
    }
}

#[derive(PartialEq, Debug)]
pub struct TupleDesc {
    items: Vec<TDItem>,
}

impl TupleDesc {
    /**
     * type_ar: array specifying the number of and types of fields in this TupleDesc.
     * It must contain at least one entry.
     *
     * field_ar: array specifying the names of the fields. Note that names may be null.
     */
    pub fn new(type_ar: Vec<Type>, field_ar: Vec<&str>) -> Self {
        Self {
            items: type_ar
                .into_iter()
                .zip(field_ar)
                .map(|(ty, s)| TDItem::new(ty, s.to_string()))
                .collect(),
        }
    }

    pub fn default_new(type_ar: Vec<Type>) -> Self {
        Self {
            items: type_ar
                .into_iter()
                .map(|ty| TDItem::new(ty, "".to_string()))
                .collect(),
        }
    }

    pub fn iterator(&self) -> Iter<'_, TDItem> {
        self.items.iter()
    }

    /// Return the number of fields in this TupleDesc.
    pub fn num_fields(&self) -> usize {
        self.items.len()
    }

    /// Gets the (possibly null) field name of the ith field of this TupleDesc.
    pub fn get_field_name(&self, i: usize) -> Option<&str> {
        match self.items.get(i) {
            Some(item) => Some(&item.field_name),
            None => None,
        }
    }

    /// Gets the type of the ith field of this TupleDesc.
    pub fn get_field_type(&self, i: usize) -> Option<Type> {
        match self.items.get(i) {
            Some(item) => Some(item.field_type),
            None => None,
        }
    }

    /// Find the index of the field with a given name.
    pub fn field_name_to_index(&self, target: String) -> Option<usize> {
        for (index, item) in self.items.iter().enumerate() {
            if target.eq(&item.field_name) {
                return Some(index);
            }
        }
        None
    }

    pub fn get_size(&self) -> i32 {
        self.items.iter().map(|item| item.field_type.len()).sum()
    }

    /**
     * Merge two TupleDescs into one, with td1.numFields + td2.numFields fields,
     * with the first td1.numFields coming from td1 and the remaining from td2.
     */
    pub fn merge(td1: TupleDesc, td2: TupleDesc) -> Self {
        Self {
            items: td1.items.into_iter().chain(td2.items).collect(),
        }
    }
}

impl fmt::Display for TupleDesc {
    /**
     * Returns a String describing this descriptor. It should be of the form
     * "fieldType[0] (fieldName[0]), ..., fieldType[M] (fieldName[M])", although
     * the exact format does not matter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let item = TDItem::new(Type::Int, "Age".to_string());
        assert_eq!(item.to_string(), "Age(Int)".to_string());
    }

    #[test]
    fn test_iterator() {
        let tuple_desc = TupleDesc::new(
            vec![Type::Str, Type::Str, Type::Int],
            vec!["School", "Student", "Age"],
        );
        let mut i = tuple_desc.iterator();
        assert_eq!(
            i.next(),
            Some(&TDItem {
                field_type: Type::Str,
                field_name: "School".to_string()
            })
        );
        assert_eq!(
            i.next(),
            Some(&TDItem {
                field_type: Type::Str,
                field_name: "Student".to_string()
            })
        );
        assert_eq!(
            i.next(),
            Some(&TDItem {
                field_type: Type::Int,
                field_name: "Age".to_string()
            })
        );
        assert_eq!(i.next(), None);
    }

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

    fn combined_strings(td1: &TupleDesc, td2: &TupleDesc, td3: TupleDesc) -> bool {
        for i in 0..td1.num_fields() {
            if td1.get_field_name(i) != td3.get_field_name(i) {
                return false;
            }
        }

        for i in td1.num_fields()..td1.num_fields() + td2.num_fields() {
            if td2.get_field_name(i - td1.num_fields()) != td3.get_field_name(i) {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_merge() {
        let str1 = get_string(1, "td1");
        let str2 = get_string(2, "td2");
        let td1 = TupleDesc::new(get_type(1), str1.iter().map(|n| n.as_str()).collect());
        let td2 = TupleDesc::new(get_type(2), str2.iter().map(|n| n.as_str()).collect());
        let td3 = TupleDesc::merge(td1, td2);
        assert_eq!(3, td3.num_fields());
        assert_eq!(3 * Type::Int.len(), td3.get_size());
        for i in 0..3 {
            assert_eq!(Some(Type::Int), td3.get_field_type(i));
        }

        let td1 = TupleDesc::new(get_type(1), str1.iter().map(|n| n.as_str()).collect());
        let td2 = TupleDesc::new(get_type(2), str2.iter().map(|n| n.as_str()).collect());
        assert_eq!(combined_strings(&td1, &td2, td3), true);

        let td3 = TupleDesc::merge(td2, td1);
        assert_eq!(3, td3.num_fields());
        assert_eq!(3 * Type::Int.len(), td3.get_size());
        for i in 0..3 {
            assert_eq!(Some(Type::Int), td3.get_field_type(i));
        }

        let td1 = TupleDesc::new(get_type(1), str1.iter().map(|n| n.as_str()).collect());
        let td2 = TupleDesc::new(get_type(2), str2.iter().map(|n| n.as_str()).collect());
        let td2_clone = TupleDesc::new(get_type(2), str2.iter().map(|n| n.as_str()).collect());
        assert_eq!(combined_strings(&td2, &td1, td3), true);

        let td3 = TupleDesc::merge(td2, td2_clone);
        assert_eq!(4, td3.num_fields());
        assert_eq!(4 * Type::Int.len(), td3.get_size());
        for i in 0..4 {
            assert_eq!(Some(Type::Int), td3.get_field_type(i));
        }
        let td2 = TupleDesc::new(get_type(2), str2.iter().map(|n| n.as_str()).collect());
        assert_eq!(combined_strings(&td2, &td2, td3), true);
    }

    #[test]
    fn test_get_type() {
        let lengths = [1, 2, 1000];
        for len in lengths.iter() {
            let td = TupleDesc::default_new(get_type(*len as usize));
            for i in 0..*len {
                assert_eq!(Some(Type::Int), td.get_field_type(i));
            }
        }
    }

    #[test]
    fn test_name_to_id() {
        let lengths = [1, 2, 1000];
        let prefix = "test";
        for len in lengths.iter() {
            let str = get_string(*len, prefix);
            let td = TupleDesc::new(get_type(*len), str.iter().map(|n| n.as_str()).collect());
            for i in 0..*len {
                assert_eq!(Some(i), td.field_name_to_index(format!("{}{}", prefix, i)));
            }

            assert_eq!(
                None,
                td.field_name_to_index("foo".to_string()),
                "foo is not a valid field name"
            );

            let td = TupleDesc::default_new(get_type(*len));
            assert_eq!(
                None,
                td.field_name_to_index(prefix.to_string()),
                "no fields are named, so you can't find it"
            );
        }
    }

    #[test]
    fn test_get_size() {
        let lengths = [1, 2, 1000];
        for len in lengths.iter() {
            let td = TupleDesc::default_new(get_type(*len));
            assert_eq!(*len as i32 * Type::Int.len(), td.get_size());
        }
    }

    #[test]
    fn test_get_num_fields() {
        let lengths = [1, 2, 1000];
        for len in lengths.iter() {
            let td = TupleDesc::default_new(get_type(*len));
            assert_eq!(*len, td.num_fields());
        }
    }

    #[test]
    fn test_equals() {
        let single_int = TupleDesc::default_new(vec![Type::Int]);
        let single_int2 = TupleDesc::default_new(vec![Type::Int]);
        let int_string = TupleDesc::default_new(vec![Type::Int, Type::Str]);

        assert_eq!(single_int, single_int);
        assert_eq!(single_int, single_int2);
        assert_eq!(single_int2, single_int);
        assert_eq!(int_string, int_string);

        assert_ne!(single_int, int_string);
        assert_ne!(single_int2, int_string);
        assert_ne!(int_string, single_int);
        assert_ne!(int_string, single_int2);
    }
}
