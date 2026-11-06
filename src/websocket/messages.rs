use serde::{Deserialize, Serialize};

use crate::operations::Operation;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    JoinDocument {
        document_id: String,
        user_id: String,
    },

    SubmitOperation {
        document_id: String,
        revision: u64,
        operation: Operation,
    },
}
// {
//   "type": "JoinDocument",
//   "data": {
//     "document_id": "doc1",
//     "user_id": "alice"
//   }
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    DocumentState {
        document_id: String,
        content: String,
        revision: u64,
    },

    OperationAccepted {
        revision: u64,
        operation: Operation,
    },

    Error {
        message: String,
    },
}
// {
//   "type": "SubmitOperation",
//   "data": {
//     "document_id": "doc1",
//     "revision": 5,
//     "operation": { "type": "Insert", "pos": 3, "text": "hi" }
//   }
// }
