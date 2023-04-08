use std::fs::File;
use std::io::{Read, Result};
use std::mem::size_of;
use std::convert::TryInto; 

#[derive(Debug)]
struct CData {
    data_type: i32,
    value: [f32; 10],
    timestamp: i64,
    message: String,
}

impl CData {
    fn from_bytes(bytes: &[u8]) -> Self {
        let data_type = i32::from_ne_bytes(bytes[..4].try_into().unwrap());
        let value = [
            f32::from_ne_bytes(bytes[4..44].try_into().unwrap()),
            f32::from_ne_bytes(bytes[44..84].try_into().unwrap()),
            f32::from_ne_bytes(bytes[84..124].try_into().unwrap()),
            f32::from_ne_bytes(bytes[124..164].try_into().unwrap()),
            f32::from_ne_bytes(bytes[164..204].try_into().unwrap()),
            f32::from_ne_bytes(bytes[204..244].try_into().unwrap()),
            f32::from_ne_bytes(bytes[244..284].try_into().unwrap()),
            f32::from_ne_bytes(bytes[284..324].try_into().unwrap()),
            f32::from_ne_bytes(bytes[324..364].try_into().unwrap()),
            f32::from_ne_bytes(bytes[364..404].try_into().unwrap()),
        ];
        let timestamp = i64::from_ne_bytes(bytes[404..412].try_into().unwrap());
        let message_bytes = &bytes[412..432];
        let null_byte_index = message_bytes.iter().position(|&b| b == 0).unwrap_or(20);
        let message = String::from_utf8_lossy(&message_bytes[..null_byte_index])
            .trim()
            .to_owned();
        CData {
            data_type,
            value,
            timestamp,
            message,
        }
    }
}

fn main() -> Result<()> {
    //let file_name = std::env::args().nth(1).expect("Missing file name");
    let mut file = File::open("data.bin")?;//File::open(file_name)?;

    let mut buffer = vec![0; size_of::<CData>() * 100];
    file.read_exact(&mut buffer)?;

    let mut cdata_vec = Vec::with_capacity(100);
    for chunk in buffer.chunks_exact(size_of::<CData>()) {
        let cdata = CData::from_bytes(chunk);
        cdata_vec.push(cdata);
    }

    println!("{:?}", cdata_vec);

    Ok(())
}