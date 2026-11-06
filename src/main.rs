use std::char;

// OT Operation Transfromation demo with websockets
#[allow(dead_code)]
#[derive(Debug)]
enum Operations<'a> {
    Insert { position: usize, text: &'a str },
    Delete { position: usize, length: usize },
    Retain { length: usize },
}

#[allow(dead_code)]
#[derive(Debug)]
struct Document {
    id: String,
    content: String,
    revision: u64,
}

fn apply_op(op: &Operations, data: String) -> String {
    let mut result = data;

    // check idx positionI
    match op {
        // Insert text at the current cursor
        Operations::Insert { position, text } => {
            // let (left, right) = result.split_at(*position as usize);
            // result = format!("{left}{text}{right}");
            let mut chars: Vec<char> = result.chars().collect();

            for (i, ch) in text.chars().enumerate() {
                chars.insert(*position as usize + i, ch);
            }

            result = chars.into_iter().collect();
        }
        // Delete text at n to cursor
        Operations::Delete { position, length } => {
            let mut chars: Vec<char> = result.chars().collect();

            chars.drain(*position..(*position + *length));

            result = chars.into_iter().collect();
        }

        // Retain n text to cursor
        Operations::Retain { length } => {
            let chars: Vec<char> = result.chars().collect();

            result = chars.iter().take(*length).collect();
        }
    }
    result
}

#[tokio::main]
async fn main() {
    let data = String::from("Hello world");
    let op_result = apply_op(
        &Operations::Insert {
            position: 5,
            text: " beautiful",
        },
        data,
    );

    println!("OT'ed text: {:?}", op_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_op() {
        let data = String::from("Hello world.");
        let corpus = " beautiful";
        assert_eq!(
            apply_op(
                &Operations::Insert {
                    position: 5,
                    text: corpus
                },
                data
            ),
            String::from("Hello beautiful world.")
        );
    }

    #[test]
    fn test_delete_op() {
        let data = String::from("Hello beautiful world.");
        assert_eq!(
            apply_op(
                &Operations::Delete {
                    position: 5,
                    length: 10
                },
                data
            ),
            String::from("Hello world.")
        );
    }
}
