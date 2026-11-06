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
    let len = data.len();
    let mut result = data;

    // check idx positionI
    match op {
        Operations::Insert { position, text } => {
            // let (left, right) = result.split_at(*position as usize);
            // result = format!("{left}{text}{right}");
            let mut chars: Vec<char> = result.chars().collect();

            for (i, ch) in text.chars().enumerate() {
                chars.insert(*position as usize + i, ch);
            }

            result = chars.into_iter().collect();
        }
        Operations::Delete { position, length } => {}
        Operations::Retain { length: _ } => {}
    }
    result
}

fn main() {
    let data = String::from("Hello world");
    let corpus = "Hello World";
    let op_result = apply_op(
        &Operations::Insert {
            position: 5,
            text: " beautiful",
        },
        data,
    );

    println!("OT'ed text: {:?}", op_result)
}
