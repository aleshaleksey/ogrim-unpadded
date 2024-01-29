/// This is a set of shared functions that is used by all crates.

/// Is this a valid name,
pub fn is_name(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    is_name_start_char(first) && chars.all(is_name_char)
}

/// Is this a valid character for the first character of a name.
pub fn is_name_start_char(c: char) -> bool {
    matches!(c,
        ':'
        | 'A'..='Z'
        | '_'
        | 'a'..='z'
        | '\u{C0}'..='\u{D6}'
        | '\u{D8}'..='\u{F6}'
        | '\u{F8}'..='\u{2FF}'
        | '\u{370}'..='\u{37D}'
        | '\u{37F}'..='\u{1FFF}'
        | '\u{200C}'..='\u{200D}'
        | '\u{2070}'..='\u{218F}'
        | '\u{2C00}'..='\u{2FEF}'
        | '\u{3001}'..='\u{D7FF}'
        | '\u{F900}'..='\u{FDCF}'
        | '\u{FDF0}'..='\u{FFFD}'
        | '\u{10000}'..='\u{EFFFF}'
    )
}

/// Is this a valid character for other parts of a name.
pub fn is_name_char(c: char) -> bool {
    is_name_start_char(c) || matches!(c,
        '-'
        | '.'
        | '0'..='9'
        | '\u{B7}'
        | '\u{0300}'..='\u{036F}'
        | '\u{203F}'..='\u{2040}'
    )
}
