use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process::exit;


enum Content
{
    ValueStruct
    {
        _type: i32,
        val: f32,
        timestamp: i64,
    },
    MValueStruct
    {
        _type: i32,
        vals: [f32; 10],
        timestamp: i64,
    },
    MessageStruct
    {
        _type: i32,
        message: [char; 21],
    }, 
}

pub struct CData
{
    _type: u8,
    content: Content,
}

impl CData
{
    pub fn from_file(input: BufReader<File>, size: usize, output: &mut Vec<CData>)
    {
        let mut _type: u8;
        let mut inner_type: i32;
        let mut content: Content = Content::ValueStruct { _type: 0, val: 0.0, timestamp: 0 };

        let mut offset: usize = 0;
        let mut first_field_start: usize = 0;
        let mut first_field_end = 0; 

        let bytes: Vec<u8> = input.bytes()
            .map(|r| r.unwrap())
            .collect(); 

        for _ in 0..size
        {
            _type = bytes[offset];
            first_field_end = first_field_start + 4;

            inner_type = i32::from_le_bytes(bytes[first_field_start..first_field_end].try_into().unwrap());
            match _type
            {
                0 =>
                {
                    offset += 24;
                    let second_field_end = first_field_end + 4;
                    let third_field_end = second_field_end + 8;

                    let val: f32 = f32::from_le_bytes(bytes[first_field_end..second_field_end].try_into().unwrap());
                    let timestamp: i64 = i64::from_le_bytes(bytes[second_field_end..third_field_end].try_into().unwrap());
                    
                    content = Content::ValueStruct { _type: inner_type, val: val, timestamp: timestamp };
                }

                1 =>
                {
                    offset += 64;
                    let second_field_end = first_field_end + 40;
                    let third_field_end = second_field_end + 8;

                    let mut vals: [f32; 10] = [0.0; 10];
                    for j in 0..10
                    {
                        vals[j] = f32::from_le_bytes(bytes[first_field_end + j .. second_field_end + j].try_into().unwrap());
                    }
                    let timestamp: i64 = i64::from_le_bytes(bytes[second_field_end..third_field_end].try_into().unwrap());
                    content = Content::MValueStruct { _type: inner_type, vals: vals, timestamp: timestamp };

                } 

                2 =>
                {
                    offset += 88;
                    let second_field_end = first_field_end + 84;

                    let mut message: [char; 21] = ['\0'; 21];
                    for j in 0..21
                    {
                        let ch = u32::from_le_bytes(bytes[first_field_end + j ..second_field_end + j].try_into().unwrap());
                        message[j] = char::from_u32(ch).unwrap();
                    }
                    content = Content::MessageStruct { _type: inner_type, message: message };
                }

                _ =>
                {
                    eprintln!("Unsupported structure");
                    exit(1);
                }
                
            }
            output.push(CData { _type: _type, content: content });
            first_field_start += offset;
        }
        
    }

    
}