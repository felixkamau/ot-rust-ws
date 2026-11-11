use crate::operations::Operation;

pub fn transform(a: &Operation, b: &Operation) -> Vec<Operation> {
    // tranform a against b if b is applied first
    // tie-breaking handling
    match (a, b) {
        // Insert vs Insert
        (
            Operation::Insert {
                position: a_pos,
                text: a_text,
            },
            Operation::Insert {
                position: b_pos,
                text: b_text,
            },
        ) => {
            let new_position = if b_pos <= a_pos {
                a_pos + b_text.chars().count()
            } else {
                *a_pos
            };

            vec![Operation::Insert {
                position: new_position,
                text: a_text.clone(),
            }]
        }

        // Insert vs Delete
        (
            Operation::Insert {
                position: a_pos,
                text,
            },
            Operation::Delete {
                position: b_pos,
                length: b_len,
            },
        ) => {
            let new_position = if a_pos >= &(b_pos + b_len) {
                a_pos - b_len
            } else if a_pos >= b_pos {
                *b_pos
            } else {
                *a_pos
            };

            vec![Operation::Insert {
                position: new_position,
                text: text.clone(),
            }]
        }

        // Delete vs Insert
        (
            Operation::Delete {
                position: a_pos,
                length: a_len,
            },
            Operation::Insert {
                position: b_pos,
                text,
            },
        ) => {
            let insert_len = text.chars().count();
            let delete_end = a_pos + a_len;

            if b_pos <= a_pos {
                vec![Operation::Delete {
                    position: a_pos + insert_len,
                    length: *a_len,
                }]
            } else if b_pos >= &delete_end {
                vec![Operation::Delete {
                    position: *a_pos,
                    length: *a_len,
                }]
            } else {
                let before = b_pos - a_pos;
                let after = a_len - before;

                vec![
                    Operation::Delete {
                        position: *a_pos,
                        length: before,
                    },
                    Operation::Delete {
                        position: a_pos + before + insert_len,
                        length: after,
                    },
                ]
            }
        }

        // Delete vs Delete
        (
            Operation::Delete {
                position: a_pos,
                length: a_len,
            },
            Operation::Delete {
                position: b_pos,
                length: b_len,
            },
        ) => {
            let a_end = a_pos + a_len;
            let b_end = b_pos + b_len;

            if a_end <= *b_pos {
                vec![Operation::Delete {
                    position: *a_pos,
                    length: *a_len,
                }]
            } else if *a_pos >= b_end {
                vec![Operation::Delete {
                    position: a_pos - b_len,
                    length: *a_len,
                }]
            } else {
                let overlap_start = (*a_pos).max(*b_pos);
                let overlap_end = a_end.min(b_end);
                let overlap = overlap_end.saturating_sub(overlap_start);
                let remaining = a_len - overlap;

                if remaining == 0 {
                    vec![]
                } else if *a_pos < *b_pos {
                    vec![Operation::Delete {
                        position: *a_pos,
                        length: remaining,
                    }]
                } else {
                    vec![Operation::Delete {
                        position: *b_pos,
                        length: remaining,
                    }]
                }
            }
        }

        // Retain is not handled in this position-based model.
        _ => vec![a.clone()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::Operation;
    #[test]
    // Assuming B gets applied first.`
    // When another insertion happens before or at your insertion
    // postion your position must move forward.
    fn test_insert_vs_insert() {
        let op_a = Operation::Insert {
            position: 6,
            text: "beautiful ".to_string(),
        };

        let op_b = Operation::Insert {
            position: 6,
            text: "amazing ".to_string(),
        };

        let result = transform(&op_a, &op_b);
        assert_eq!(
            result,
            vec![Operation::Insert {
                position: 14,
                text: "beautiful ".to_string()
            }]
        );
    }

    #[test]
    fn test_insert_vs_delete() {
        let op_a = Operation::Insert {
            position: 6,
            text: "amazing ".to_string(),
        };

        let op_b = Operation::Delete {
            position: 6,
            length: 10,
        };

        let result = transform(&op_a, &op_b);

        assert_eq!(
            result,
            vec![Operation::Insert {
                position: 6,
                text: "amazing ".to_string()
            }]
        );
    }

    #[test]
    fn test_delete_vs_delete() {
        let op_a = Operation::Delete {
            position: 6,
            length: 5,
        };

        let op_b = Operation::Delete {
            position: 7,
            length: 4,
        };

        let result = transform(&op_a, &op_b);

        assert_eq!(
            result,
            vec![Operation::Delete {
                position: 6,
                length: 1
            }]
        );
    }
}
