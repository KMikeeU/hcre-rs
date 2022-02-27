

// Define all the rules
// TODO:
// Toggle Case
// Toggle @
// Reflect
// Rotate Left
// Rotate Right
// Truncate Left
// Truncate Right
// 
#[derive(Debug)]
pub enum Rule{
    Nothing,
    Append(char),
    Prepend(char),
    Substitute(char, char),
    Lowercase(),
    Uppercase(),
    Capitalize(),
    InvertCapitalize(),
    Reverse(),
    Duplicate(),
    Purge(char),
    RotateLeft(),
    RotateRight()
}


