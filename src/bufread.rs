
use std::io::prelude::*;
use std::fs::File;

pub struct HexDump;

fn sanitize(hex: &str) -> String {
    let mut temp = String::from("0");
    if hex.trim().len() == 1 {
        temp.push_str(hex);
        return temp;
    }

    else {
        return hex.to_string();
    }
}

fn sanitize_chars(c: String) -> String {
    let mut temp: String = String::new();
    let count = 0;
    for chr in c.chars().into_iter() {
        let value = chr as u32;
        if count == 8 {
            temp.push('|');
        }

        if value > 32 && value < 128 {
            temp.push(chr);
        }
            else {
                temp.push('.');
            }
        }
    return temp;

}

impl HexDump {

    pub fn new() -> Self {
        HexDump {}
    }
    pub fn read(&mut self, path: &str) -> Result<(), std::io::Error>{
        let mut output: String = String::new();
        let mut file = File::open(path)?;
        let mut offset: u32 = 0;

        loop {
            let mut temp_chunks = Vec::with_capacity(16); //load new byte chunk
            let n = std::io::Write::by_ref(&mut file).take(16 as u64).read_to_end(&mut temp_chunks)?;
            let mut chars = sanitize_chars(temp_chunks.iter().map(|x| *x as char).collect::<String>());
            let mut count = 0;
            output.push_str(format!("{:08x}: ", offset).as_str());

            if n == 0 {
                break;
            }
            
            else {
                for byte in &temp_chunks {
                    if count == 8 {output.push_str("| ")}
                    output.push_str(sanitize(format!("{:X?} ", byte).as_str()).as_str());
                    
                    count += 1;
                }

                if temp_chunks.len()%16 != 0 {
                    output.push_str("   ".repeat(16-temp_chunks.len()).as_str()); 
                    output.push_str(chars.as_str());
                }
                else {
                    output.push_str(chars.as_str());
                }

                println!("{}", output);
                
            }
            offset += 16;
            output.clear();
            chars.clear();
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{any::Any, fmt::Result, io::Bytes};

    #[test]
    fn test_byte_struct()  {
        let test_struct = HexDump::new();
        assert_eq!(1, 1);
    }
}