#[macro_use]

use nom::{is_alphanumeric};

// named!(parens, delimited!(char!('('), is_not!(")"), char!(')')));
// named!(alpha, take_while!( is_alphanumeric ) );

named!(parens_num, delimited!(char!('('), take_while!(is_alphanumeric), char!(')')));
