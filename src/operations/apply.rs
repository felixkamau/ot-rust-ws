use crate::operations::Operation;

pub fn apply_op(op: &Operation, data: &str) -> String {
    let mut result = data.to_string();

    match op {
        // Insert text at the current cursor
        Operation::Insert { position, text } => {
            // let (left, right) = result.split_at(*position as usize);
            // result = format!("{left}{text}{right}");
            let mut chars: Vec<char> = result.chars().collect();

            for (i, ch) in text.chars().enumerate() {
                chars.insert(*position as usize + i, ch);
            }

            result = chars.into_iter().collect();
        }

        // Delete text at n to cursor
        Operation::Delete { position, length } => {
            let mut chars: Vec<char> = result.chars().collect();
            chars.drain(*position..(*position + *length));

            result = chars.into_iter().collect();
        }

        // Retain n text to cursor
        // Will fix this impl laters
        Operation::Retain { length } => {
            let chars: Vec<char> = result.chars().collect();
            result = chars.iter().take(*length).collect();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_op() {
        let data = String::from("Hello world.");
        // let corpus = " beautiful";
        assert_eq!(
            apply_op(
                &Operation::Insert {
                    position: 5,
                    text: String::from(" beautiful")
                },
                &data
            ),
            String::from("Hello beautiful world.")
        );
    }

    #[test]
    fn test_delete_op() {
        let data = String::from("Hello beautiful world.");
        assert_eq!(
            apply_op(
                &Operation::Delete {
                    position: 5,
                    length: 10
                },
                &data
            ),
            String::from("Hello world.")
        );
    }

    #[test]
    fn test_retain_op() {
        let data = String::from("Hello beautiful world.");
        let n = 5;
        assert_eq!(
            apply_op(&Operation::Retain { length: n }, &data),
            String::from("Hello")
        );
    }

    #[test]
    fn test_retain_insert_op() {
        let data = String::from("Hello beautiful world.");
        let n = 5;
        let pos = 5;
        let retain = apply_op(&Operation::Retain { length: n }, &data);
        assert_eq!(retain, String::from("Hello"));

        let insertion = apply_op(
            &Operation::Insert {
                position: pos,
                text: String::from(" world."),
            },
            &retain,
        );

        assert_eq!(insertion, String::from("Hello world."));
    }
}
