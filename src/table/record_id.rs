use crate::common::page_id::PageId;

/**
 * A RecordId is a reference to a specific tuple on a specific page of a
 * specific table.
 */
pub struct RecordId {
    pid: Box<dyn PageId>,
    tupleno: i32,
}

impl RecordId {
    pub fn new(pid: Box<dyn PageId>, tupleno: i32) -> Self {
        Self { pid, tupleno }
    }

    pub fn get_tuple_number(&self) -> i32 {
        self.tupleno
    }

    pub fn get_page_id(&self) -> &Box<dyn PageId> {
        &self.pid
    }

    pub fn equals(&self, other: &Self) -> bool {
        false
    }
}
