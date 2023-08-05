

// Define all the rules
// TODO:
// "Rules used to reject plains"
// "Implemented specific functions"
// see https://hashcat.net/wiki/doku.php?id=rule_based_attack

#[derive(Debug)]
pub enum Rule{
    Nothing,
    Lowercase(),
    Uppercase(),
    Capitalize(),
    InvertCapitalize(),
    ToggleCase(),
    ToggleAt(usize),
    Reverse(),
    Duplicate(),
    DuplicateN(usize),
    Reflect(),
    RotateLeft(),
    RotateRight(),
    Append(char),
    Prepend(char),
    TruncateLeft(),
    TruncateRight(),
    DeleteAt(usize),
    ExtractRange(usize, usize),
    OmitRange(usize, usize),
    InsertAt(usize, char),
    OverwriteAt(usize, char),
    TruncateAt(usize),
    Replace(char, char),
    Purge(char),
    DuplicateFirstN(usize),
    DuplicateLastN(usize),
    DuplicateAll(),
    // ExtractMemory(usize, usize, usize),
    // AppendMemory(),
    // PrependMemory(),
    // Memorize()

    Invalid(&'static str)
}


