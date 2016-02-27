use nom::IResult::*;
named!(is_not_end_or_new_key<&str, &str>, is_not_s!("\n\r"));
named!(get_one_val<&str, &str>,
       chain!(
           tag_s!("") ~
           val: is_not_end_or_new_key,

           || {val}
));

#[derive(Debug)]
pub enum ParsedKey<'a> {
    Key(&'a str),
    SentenceStarter(&'a str)
}
#[derive(Debug)]
pub enum ParserError {
    KeyMatchFailed,
    NoValsMatched
}
named!(get_key<&str, ParsedKey>,
       chain!(
           tag_s!("") ~
               ss: tag_s!("")? ~
               val: is_not_end_or_new_key,

           || {
               if ss.is_some() {
                   ParsedKey::SentenceStarter(val)
               } else {
                   ParsedKey::Key(val)
               }
           }
));
pub fn parseln(str: &str) -> Result<(ParsedKey, Vec<&str>), ParserError> {
    match get_key(str) {
        Done(ks, pk) => {
            let mut key_str = ks;
            let mut key_vec: Vec<&str> = Vec::new();
            while let Done(new_key_str, key) = get_one_val(key_str) {
                key_vec.push(key);
                key_str = new_key_str;
            }
            if key_vec.len() < 1 {
                Err(ParserError::NoValsMatched)
            }
            else {
                Ok((pk, key_vec))
            }
        }
        _ => Err(ParserError::KeyMatchFailed)
    }
}
