use std::fs::File;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;


const NUM_SENSORS: usize = 10;
const DATA_SIZE: usize = 4; 
fn producer() {
    let mut file = File::create("data.bin").unwrap();
    loop {
        let mut data = [0u8; NUM_SENSORS * DATA_SIZE];
        for sensor in 0..NUM_SENSORS {
            let value = rand::random::<i32>();
            let offset = sensor * DATA_SIZE;
            data[offset..offset+DATA_SIZE].copy_from_slice(&value.to_ne_bytes());
           
        }
        file.write_all(&data).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

fn consumer() {
    loop {
        let mut file = File::open("./data.bin").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        let num_records = data.len() / (NUM_SENSORS * DATA_SIZE);
        if num_records > 0 {
            for sensor in 0..NUM_SENSORS {
                let mut values = Vec::new();
                for record in 0..num_records {
                    let offset = record * NUM_SENSORS * DATA_SIZE + sensor * DATA_SIZE;
                    let value = i32::from_le_bytes(data[offset..offset+DATA_SIZE].try_into().unwrap());
                    values.push(value);
                }
                let min = values.iter().min().unwrap();
                let max = values.iter().max().unwrap();
                let avg = values.iter().sum::<i32>() / num_records as i32;
                println!("Sensor {}: min={}, max={}, avg={}", sensor, min, max, avg);
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}

#[repr(C)]
struct SensorData {
seq: u32, // sequenza letture
values: [f32; 10],
timestamp: u32
}


fn main() {
    let producer_handle = thread::spawn(producer);
    let consumer_handle = thread::spawn(consumer);
    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}

