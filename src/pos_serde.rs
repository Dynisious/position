//! Additional implementations for parsing strings as `Pos` values based on `Display`.
//! 
//! Gated by the `pos-serde` feature.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/07/16

#![cfg(feature = "pos-serde")]

use super::*;
use std::{str::FromStr, fmt,};

impl<T: FromStr> FromStr for Pos<T> {
    type Err = ParsePosErr;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        static ERR_STR: &str = "Error parsing `s`; expected str of form `(T, T)`";
        
        //Trim the string.
        s = s.trim();

        //Check whether the value is surrounded by braces.
        if s.starts_with('(') && s.ends_with(')') {
            //Trim the surrounding braces.
            s = &s[1..(s.len() - 1)];

            //Search the string for the seperator.
            for sep in 1..s.len() {
                //Check if this index is a seperator candidate.
                if &s[sep..=sep] == "," {
                    //Attempt to parse the [`Pos`] using this seperator.
                    //  Parse the `x` value.
                    let pos = s[0..sep].parse::<T>()
                        //Parse the `y` value.
                        .and_then(|x| s[(sep + 1)..s.len()].trim().parse::<T>()
                            //Combine the values as a [`Pos`] instance.
                            .map(|y| Pos { x, y, })
                        );
                    
                    //If the parsing was successfull, return the value.
                    if let Ok(pos) = pos { return Ok(pos) }
                }
            }
        }

        //No attempts to parse the value succeeded.
        Err(ParsePosErr(ERR_STR))
    }
}

#[derive(Clone, Debug,)]
pub struct ParsePosErr(&'static str);

impl fmt::Display for ParsePosErr {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result { self.0.fmt(fmt) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_from_str() {
        let test_pos = Pos::new(1, 2);
        let ser = format!("{}", &test_pos);

        assert_eq!(ser, "(1, 2)", "Unexpected display value");

        let de = ser.parse::<Pos>()
            .expect("Failed to parse `ser`");
        
        assert_eq!(de, test_pos, "parsing `ser` returned a bad value");
    }
}
