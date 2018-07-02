use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct InvalidReference;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reference(u32, u32);

impl FromStr for Reference {
    type Err = InvalidReference;

    fn from_str(value: &str) -> Result<Reference, InvalidReference> {
        let maybe_split_point = value.chars().position(|c| !c.is_ascii_alphabetic());

        let split_point = match maybe_split_point {
            None | Some(0) => return Err(InvalidReference),
            Some(x) => x,
        };

        let (column_chars, row_chars) = value.split_at(split_point);

        let column = match column_chars {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            "D" => 4,
            "E" => 5,
            _ => return Err(InvalidReference),
        };

        let row = match row_chars.parse() {
            Err(_) => return Err(InvalidReference),
            Ok(x) => x,
        };

        Ok(Reference(column, row))
    }
}

impl<'a> From<&'a str> for Reference {
    fn from(src: &'a str) -> Reference {
        src.parse().unwrap()
    }
}
