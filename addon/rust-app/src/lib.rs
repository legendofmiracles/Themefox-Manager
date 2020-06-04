use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use serde_json;
use std::io::{self, Read, Write};

pub fn read_input<R: Read>(mut input: R) -> io::Result<serde_json::Value> {
    let length = input.read_u32::<NativeEndian>().unwrap();
    let mut buffer = vec![0; length as usize];
    input.read_exact(&mut buffer)?;
    let json_val: serde_json::Value = serde_json::from_slice(&buffer).unwrap();
    Ok(json_val)
}

pub fn write_output<W: Write>(mut output: W, value: &serde_json::Value) -> io::Result<()> {
    let msg = serde_json::to_string(value)?;
    let len = msg.len();
    // Chrome won't accept a message larger than 1MB
    if len > 1024 * 1024 {
        panic!("Message was too large, length: {}", len)
    }
    output.write_u32::<NativeEndian>(len as u32)?;
    output.write_all(msg.as_bytes())?;
    output.flush()?;
    Ok(())
}
