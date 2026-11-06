use crate::operations::Operation;

#[derive(Debug)]
pub struct OperationRecord {
    pub revision: u64,
    pub user_id: String, // will change to uuid
    pub operation: Operation,
}
