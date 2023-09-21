extern crate pest;
#[macro_use]
extern crate pest_derive;


#[derive(Parser)]
#[grammar = "iso8601.pest"]
pub struct ISO8601Parser;


#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    use rstest::rstest;


    #[rstest]
    #[case ("0000-01-01")]
    #[case ("1900-10-30")]
    #[case ("9999-12-31")]
    fn can_parse_iso8601_strict_date_extended(#[case] year: &str) {

        let pairs = ISO8601Parser::parse(
            Rule::iso8601_strict_date_extended, year)
            .unwrap_or_else(|e| panic!("{}", e));

        // Parsing succeeded; ensure at least 1 pair was returned
        assert!(pairs.len() > 0);
    }


    #[rstest]
    #[case ("000-01-01")]   // Year out of range
    #[case ("00000-01-01")]
    #[case ("01-01")]       // missing year part

    #[case ("0000-00-01")]  // Month out of range
    #[case ("0000-13-01")]
    #[case ("1999")]        // month segment missing
    #[case ("1999-")]

    #[case ("0000-01-00")]  // Day out of range
    #[case ("0000-01-32")]
    #[case ("1999-12")]     // day segment missing
    #[case ("1999-12-")]

    #[case ("000o-01-01")]  // Invalid chars
    #[case ("1999-0x-12")]
    #[case ("1999-12-0x")]

    #[should_panic(expected = "expected iso8601_")] // matches errors from multiple iso8601 rules
    fn verify_iso8601_strict_date_extended_error(#[case] bad_date: &str) {

        ISO8601Parser::parse(
            Rule::iso8601_strict_date_extended, bad_date)
            .unwrap_or_else(|e| panic!("{}", e));

        // should never reach this code since all cases should result in panic
        println!("Test case '{}' should fail to parse!", bad_date);
        assert!(false);
    }


    #[rstest]
    #[case ("00000101")]
    #[case ("19001030")]
    #[case ("99991231")]
    fn can_parse_iso8601_strict_date_standard(#[case] year: &str) {

        let pairs = ISO8601Parser::parse(
            Rule::iso8601_strict_date_standard, year)
            .unwrap_or_else(|e| panic!("{}", e));

        // Parsing succeeded; ensure at least 1 pair was returned
        assert!(pairs.len() > 0);
    }


    #[rstest]
    #[case ("0000101")]     // Year out of range
    #[case ("000000101")]

    #[case ("00000001")]    // Month out of range
    #[case ("00001301")]
    #[case ("0000101")]     // invalid: single digit month
    #[case ("1999")]        // month segment missing

    #[case ("00000100")]    // Day out of range
    #[case ("00000132")]
    #[case ("0000011")]     // invalid: single digit day
    #[case ("199912")]      // day segment missing

    #[case ("000o0101")]    // Invalid chars
    #[case ("19990x12")]
    #[case ("1999120x")]

    #[case ("0000-01-01")]  // Valid extended, but not standard
    #[case ("1900-10-30")]
    #[case ("9999-12-31")]

    #[should_panic(expected = "expected iso8601_")] // matches errors from multiple iso8601 rules
    fn verify_iso8601_strict_date_standard_error(#[case] bad_date: &str) {

        ISO8601Parser::parse(
            Rule::iso8601_strict_date_standard, bad_date)
            .unwrap_or_else(|e| panic!("{}", e));

        // should never reach this code since all cases should result in panic
        println!("Test case '{}' should fail to parse!", bad_date);
        assert!(false);
    }


    // TODO: cf comment in pest file
    // #[rstest]
    // #[case ("0000041")]      // single digit day
    // #[case ("00000101")]
    // #[case ("19001030")]
    // #[case ("99991231")]
    // fn can_parse_iso8601_lax_date_standard(#[case] year: &str) {

    //     let pairs = ISO8601Parser::parse(
    //         Rule::iso8601_lax_date_standard, year)
    //         .unwrap_or_else(|e| panic!("{}", e));

    //     // Parsing succeeded; ensure at least 1 pair was returned
    //     assert!(pairs.len() > 0);
    // }


    // TODO: cf comment in pest file
    // #[rstest]
    // #[case ("000000101")]   // Year out of range

    // #[case ("00000001")]    // Month out of range
    // // TODO: lax parsing stops at '30'? Interprets as 0000-01-30?
    // // #[case ("00001301")]
    // #[case ("1999")]        // month segment missing

    // #[case ("00000100")]    // Day out of range
    // // TODO: lax parsing stops at '3' rather than all of '32'?
    // // #[case ("00000132")]
    // #[case ("199912")]      // day segment missing

    // #[case ("000o0101")]    // Invalid chars
    // #[case ("19990x12")]
    // #[case ("1999120x")]

    // #[case ("0000-01-01")]  // Valid extended, but not standard
    // #[case ("1900-10-30")]
    // #[case ("9999-12-31")]

    // #[should_panic(expected = "expected iso8601_")] // matches errors from multiple iso8601 rules
    // fn verify_iso8601_lax_date_standard_error(#[case] bad_date: &str) {

    //     ISO8601Parser::parse(
    //         Rule::iso8601_lax_date_standard, bad_date)
    //         .unwrap_or_else(|e| panic!("{}", e));

    //     // should never reach this code since all cases should result in panic
    //     println!("Test case '{}' should fail to parse!", bad_date);
    //     assert!(false);
    // }

}
