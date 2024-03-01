#[derive(Debug, PartialEq)]
enum ParseError {
    ParseError,
    NotEnoughElementError,
}


fn main() {}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::*;

    #[test]
    fn ex6_parse_well_formated_line() {
        assert_eq!(parse_line("12.5;-5.2"), Ok((12.5, -5.2)));
    }

    #[test]
    fn ex6_parse_non_float_number() {
        assert_eq!(parse_line("12.5xezf;-5.2"), Err(ParseError::ParseError));
    }

    #[test]
    fn ex6_parse_line_with_1_elemnt() {
        assert_eq!(parse_line("-5.2"), Err(ParseError::NotEnoughElementError));
    }

    #[test]
    fn ex6_parse_from_correct_file() {
        let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_resources")
            .join("correct_file.mir");

        // Uncomment and create method making this test passing :
        // assert_eq!(parse_from_file(&file_path), Ok((12.5, -2.45)));
    }

    #[test]
    fn ex6_parse_from_incorrect_file() {
        let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_resources")
            .join("dontexists.mir");

        // Uncomment and create method making this test passing :
        // assert_eq!(parse_from_file(&file_path), Err(ParseError::CantOpenFile)));
    }
}
