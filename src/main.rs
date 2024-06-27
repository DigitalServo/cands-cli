use std::str::FromStr;
use serde::Deserialize;
use clap::{Parser, ValueEnum};
use cands_cyphal::CANInterface;


#[derive(Debug, Clone, ValueEnum)]
enum Command {
    Start,
    Stop,
    SetParam,
    ReadParam,
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

impl TryFrom<&str> for DataType {
    type Error = ();
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Ok(match str {
            "bool" => Self::Bool,
            "string" => Self::String,
            "u8"  => Self::U8,
            "u16" => Self::U16,
            "u32" => Self::U32,
            "u64" => Self::U64,
            "i8"  => Self::I8,
            "i16" => Self::I16,
            "i32" => Self::I32,
            "i64" => Self::I64,
            "f32" => Self::F32,
            "f64" => Self::F64,
            _ => return Err(())
        })
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct ParameterTable {
    parameter: String,
    datatype: String,
    value: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct ParameterList {
    parameter: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(help = "Communication command")]
    command: Command,
    #[arg(short, long, help = "Desination node id which you want to communicate, type: u8")]
    dest: Option<u8>,
    #[arg(short, long, help = "Key, type: String")]
    key: Option<String>,
    #[arg(short, long, help = "Value, type: String")]
    value: Option<String>,
    #[arg(short, long, help = "Value type")]
    type_: Option<DataType>,
    #[arg(short, long, help = "Path of source file")]
    path: Option<std::path::PathBuf,>,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Cli = Cli::parse();

    let mut can_interface: CANInterface = CANInterface::new()?;

    match args.command {

        Command::Start => {
            #[cfg(any(feature="v1", feature="v2"))]
            match args.dest {
                Some(dest_id) => can_interface.drive_enable(dest_id)?,
                None => can_interface.drive_enable_all()?
            }    
        },

        Command::Stop => {
            #[cfg(any(feature="v1", feature="v2"))]
            match args.dest {
                Some(dest_id) => can_interface.drive_disable(dest_id)?,
                None => can_interface.drive_disable_all()?
            }
        },

        Command::SetParam => {
            
            let dest: u8 = args.dest.unwrap();
            let path: std::path::PathBuf = args.path.unwrap();

            for row in csv::Reader::from_path(&path)?.deserialize::<ParameterTable>() {
                let x: ParameterTable = row.unwrap();
                let key: String = x.parameter;
                let data_type: DataType = DataType::try_from(x.datatype.replace(' ', "").as_str()).unwrap();
                let value_str: String = x.value;

                match data_type {
                    DataType::Bool => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<bool>(value_str))?,
                    DataType::String => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<String>(value_str))?,
                    DataType::U8  => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<u8>(value_str))?,
                    DataType::U16 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<u16>(value_str))?,
                    DataType::U32 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<u32>(value_str))?,
                    DataType::U64 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<u64>(value_str))?,
                    DataType::I8  => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<i8>(value_str))?,
                    DataType::I16 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<i16>(value_str))?,
                    DataType::I32 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<i32>(value_str))?,
                    DataType::I64 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<i64>(value_str))?,
                    DataType::F32 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<f32>(value_str))?,
                    DataType::F64 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<f64>(value_str))?,
                };

                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }

        Command::ReadParam => {

            let dest: u8 = args.dest.unwrap();
            let path: std::path::PathBuf = args.path.unwrap();

            for row in csv::Reader::from_path(&path)?.deserialize::<ParameterList>() {
                let x: ParameterList = row.unwrap();
                let key: String = x.parameter;

                can_interface.send_digitalservo_request(dest, &key)?;
                std::thread::sleep(std::time::Duration::from_millis(100));
                
                can_interface.load_frames()?;

                if let Some(values) = can_interface.get_key_value()? {
                    for value in values {
                        println!("{:?}", value.data);
                        println!("{:?}", value.props);
                    }
                }
            }
        }

        Command::Request => {

            let dest: u8 = args.dest.unwrap();
            let key: String = args.key.unwrap();

            can_interface.send_digitalservo_request(dest, &key)?;
            std::thread::sleep(std::time::Duration::from_millis(100));
            
            can_interface.load_frames()?;

            if let Some(values) = can_interface.get_key_value()? {
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
                DataType::Bool => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<bool>(value_str))?,
                DataType::String => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<String>(value_str))?,
                DataType::U8  => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<u8>(value_str))?,
                DataType::U16 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<u16>(value_str))?,
                DataType::U32 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<u32>(value_str))?,
                DataType::U64 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<u64>(value_str))?,
                DataType::I8  => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<i8>(value_str))?,
                DataType::I16 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<i16>(value_str))?,
                DataType::I32 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<i32>(value_str))?,
                DataType::I64 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<i64>(value_str))?,
                DataType::F32 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<f32>(value_str))?,
                DataType::F64 => can_interface.send_digitalservo_response(dest, &key, &parse_value_str::<f64>(value_str))?,
            };
        }

        Command::Message => {
            let key: String = args.key.unwrap();
            let data_type: DataType = args.type_.unwrap();
            let value_str: String = args.value.unwrap();

            match data_type {
                DataType::Bool => can_interface.send_digitalservo_message(&key, &parse_value_str::<bool>(value_str))?,
                DataType::String => can_interface.send_digitalservo_message(&key, &parse_value_str::<String>(value_str))?,
                DataType::U8  => can_interface.send_digitalservo_message(&key, &parse_value_str::<u8>(value_str))?,
                DataType::U16 => can_interface.send_digitalservo_message(&key, &parse_value_str::<u16>(value_str))?,
                DataType::U32 => can_interface.send_digitalservo_message(&key, &parse_value_str::<u32>(value_str))?,
                DataType::U64 => can_interface.send_digitalservo_message(&key, &parse_value_str::<u64>(value_str))?,
                DataType::I8  => can_interface.send_digitalservo_message(&key, &parse_value_str::<i8>(value_str))?,
                DataType::I16 => can_interface.send_digitalservo_message(&key, &parse_value_str::<i16>(value_str))?,
                DataType::I32 => can_interface.send_digitalservo_message(&key, &parse_value_str::<i32>(value_str))?,
                DataType::I64 => can_interface.send_digitalservo_message(&key, &parse_value_str::<i64>(value_str))?,
                DataType::F32 => can_interface.send_digitalservo_message(&key, &parse_value_str::<f32>(value_str))?,
                DataType::F64 => can_interface.send_digitalservo_message(&key, &parse_value_str::<f64>(value_str))?,
            };
        },
    }

    Ok(())
}

pub fn parse_value_str<T>(x: String) -> Vec<T>
where T: FromStr, <T as FromStr>::Err: std::fmt::Debug
{
    let x_vectorized: Vec<String> = if x.starts_with('[') & x.ends_with(']') {
        x
            .replace(' ', "")
            .strip_prefix('[').unwrap()
            .strip_suffix(']').unwrap()
            .split(',')
            .map(|str| String::from(str))
            .collect()
    } else {
        Vec::from([x.replace(' ', "")])
    };

    x_vectorized.iter().map(|x| <T as FromStr>::from_str(x).unwrap()).collect()
}