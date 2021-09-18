use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::fmt;

#[derive(Debug, Clone)]
pub struct SummationError {
    msg: String,
}

impl std::error::Error for SummationError {}
impl fmt::Display for SummationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl From<std::io::Error> for SummationError {
    fn from(e: std::io::Error) -> SummationError {
        SummationError {
            msg: format!("io::Error: {}", e),
        }
    }
}
impl From<std::num::ParseIntError> for SummationError {
    fn from(e: std::num::ParseIntError) -> SummationError {
        SummationError {
            msg: format!("ParseIntError: {}", e),
        }
    }
}

pub fn sum_file_1(file_path: &std::path::Path) -> Result<i64, SummationError> {
    //Check if file exists
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(SummationError::from(e)),
    };

    let mut data = String::new();
    match BufReader::new(file).read_to_string(&mut data) {
        Err(e) => return Err(SummationError::from(e)),
        Ok(_) => {
            let mut sum: i64 = 0;
            //Collect each line by spliting the string by new line (\n) and only take the string value
            //io::BufReader::new(file).lines() won't work in this case, because reading the file to string (read_to_string).
            let lines: Vec<&str> = data.trim().split('\n').collect();

            for i in lines {
                let num = match i.parse::<i64>() {
                    Ok(num) => num,
                    Err(e) => return Err(SummationError::from(e)),
                };
                sum += num;
            }
            return Ok(sum);
        }
    }
}

//Same as above, but use ? operators instead of pattern matching
pub fn sum_file_2(file_path: &std::path::Path) -> Result<i64, SummationError> {
    let file = File::open(file_path)?;

    let mut data = String::new();
    let _read_file = BufReader::new(file).read_to_string(&mut data)?;

    let mut sum: i64 = 0;
    let lines: Vec<&str> = data.trim().split('\n').collect();

    for i in lines {
        let num = i.parse::<i64>()?;
        sum += num;
    }
    return Ok(sum);
}