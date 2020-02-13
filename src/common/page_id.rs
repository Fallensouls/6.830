/**
 * PageId is an interface to a specific page of a specific table.
 */
pub trait PageId {
    /** Return a representation of this page id object as a collection of
        integers (used for logging).
        This class MUST have a constructor that accepts n integer parameters,
        where n is the number of integers returned in the array from serialize.
    */
    fn serialize(&self) -> Vec<i32>;

    /// Return the unique tableid hashcode with this PageId
    fn get_table_id(&self) -> u64;

    fn hash_code(&self) -> u64;
    fn equals(&self, other: &Self) -> bool
    where
        Self: Sized;
    fn get_page_number(&self) -> i32;
}
