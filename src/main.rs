use std::error::Error;

use ot_rust_ws::{
    config,
    operations::{Operation, apply_op},
};

// OT Operation Transfromation demo with websockets

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _config = config::Config::from_env().expect("Failed to load application coniguration");
    let data = String::from("Hello world");
    let op_result = apply_op(
        &Operation::Insert {
            position: 5,
            text: String::from(" beautiful"),
        },
        &data,
    );

    println!("OT'ed text: {:?}", op_result);
    Ok(())
}
