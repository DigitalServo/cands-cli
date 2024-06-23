use std::str::FromStr;

use clap::{Parser, ValueEnum};

use cands_cyphal::CANInterface;

#[derive(Debug, Clone, ValueEnum)]
enum Command {
    Start,
    Stop,
    Message,
    Request,
    Response
}

#[derive(Debug, Clone, ValueEnum)]
enum DataType {
    Bool,
    String,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,    
    F32,
    F64
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(help = "Communication command")]
    command: Command,
    #[arg(short, long, help = "Desination node id which you want to communicate, type: u8")]
    dest: Option<u8>,
    // #[arg(short, long, help = "Port ID (subject id for message, service id for request/response), type: u16")]
    // port: Option<u16>,
    #[arg(short, long, help = "Key, type: String")]
    key: Option<String>,
    #[arg(short, long, help = "Value, type: String")]
    value: Option<String>,
    #[arg(short, long, help = "Value type")]
    type_: Option<DataType>,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Cli = Cli::parse();

    let mut can_interface: CANInterface = CANInterface::new()?;

    match args.command {

        Command::Start => {
            match args.dest {
                Some(dest_id) => can_interface.drive_enable(dest_id)?,
                None => can_interface.drive_enable_all()?
            }                
        },

        Command::Stop => match args.dest {
            Some(dest_id) => can_interface.drive_disable(dest_id)?,
            None => can_interface.drive_disable_all()?
        },

        Command::Request => {
            let dest: u8 = args.dest.unwrap();
            let key: String = args.key.unwrap();

            can_interface.send_digitalservo_request(dest, &key)?;
            std::thread::sleep(std::time::Duration::from_millis(50));
            
            can_interface.load_frames()?;
            let ret = can_interface.get_key_value()?;

            if let Some(values) = ret {
                for value in values {
                    println!("{:?}", value.data);
                    println!("{:?}", value.props);
                }
            } else {
                println!("No data received.");
            }
        },

        Command::Response => {
            let dest: u8 = args.dest.unwrap();
            let key: String = args.key.unwrap();
            let data_type: DataType = args.type_.unwrap();
            let value_str: String = args.value.unwrap();

            match data_type {
                DataType::Bool => {
                    let x: bool = FromStr::from_str(&value_str)?;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                },
                DataType::String => {
                    let x = value_str;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                },
                DataType::U8 => {
                    let x: u8 = value_str.parse()?;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                },
                DataType::U16 => {
                    let x: u16 = value_str.parse()?;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                },
                DataType::U32 => {
                    let x: u32 = value_str.parse()?;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                },
                DataType::U64 => {
                    let x: u64 = value_str.parse()?;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                },
                DataType::I8 => {
                    let x: i8 = value_str.parse()?;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                },
                DataType::I16 => {
                    let x: i16 = value_str.parse()?;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                },
                DataType::I32 => {
                    let x: i32 = value_str.parse()?;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                },
                DataType::I64 => {
                    let x: i64 = value_str.parse()?;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                },    
                DataType::F32 => {
                    let x: f32 = value_str.parse()?;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                },
                DataType::F64 => {
                    let x: f64 = value_str.parse()?;
                    can_interface.send_digitalservo_response(dest, &key, &[x])?;
                }
            };
        }

        Command::Message => {
            let key: String = args.key.unwrap();
            let data_type: DataType = args.type_.unwrap();
            let value_str: String = args.value.unwrap();

            match data_type {
                DataType::Bool => {
                    let x: bool = FromStr::from_str(&value_str)?;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                },
                DataType::String => {
                    let x = value_str;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                },
                DataType::U8 => {
                    let x: u8 = value_str.parse()?;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                },
                DataType::U16 => {
                    let x: u16 = value_str.parse()?;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                },
                DataType::U32 => {
                    let x: u32 = value_str.parse()?;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                },
                DataType::U64 => {
                    let x: u64 = value_str.parse()?;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                },
                DataType::I8 => {
                    let x: i8 = value_str.parse()?;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                },
                DataType::I16 => {
                    let x: i16 = value_str.parse()?;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                },
                DataType::I32 => {
                    let x: i32 = value_str.parse()?;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                },
                DataType::I64 => {
                    let x: i64 = value_str.parse()?;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                },    
                DataType::F32 => {
                    let x: f32 = value_str.parse()?;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                },
                DataType::F64 => {
                    let x: f64 = value_str.parse()?;
                    can_interface.send_digitalservo_message(&key, &[x])?;
                }
            };
        },
    }

    Ok(())
}