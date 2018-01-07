use std::error::Error;

#[derive(Debug,Display)]
pub struct JumpOutOfRangeError
{
    pc : isize,
}

impl Error for OutOfRangeError {
    fn description(&self) -> &str
    {
        &format_err!("Jumped out of range: {}", self.pc)
    }
} 