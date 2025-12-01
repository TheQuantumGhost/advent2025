pub trait FromChar: Sized {
    type Err;

    fn from_char(c: char) -> Result<Self, Self::Err>;
}

/// Char to bool
pub enum ParseBoolError {
    NotDigit(char),
    NotValid(char),
}

impl FromChar for bool {
    type Err = ParseBoolError;
    fn from_char(c: char) -> Result<Self, Self::Err> {
        if !c.is_ascii_digit() {
            Err(Self::Err::NotDigit(c))
        } else if c == '0' {
            Ok(false)
        } else if c == '1' {
            Ok(true)
        } else {
            Err(Self::Err::NotValid(c))
        }
    }
}
