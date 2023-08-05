

pub struct StringReader {
    string: Vec<char>,
    index: usize
}
impl StringReader {
    pub fn from_string(s: &str) -> StringReader {
        StringReader::new(s.chars().collect())
    }

    pub fn new(string: Vec<char>) -> StringReader {
        StringReader {
            string,
            index: 0
        }
    }

    pub fn read(&mut self) -> Result<char, &str> {
        if self.index >= self.string.len() {
            return Err("Reached end of string");
        }
        let c = self.string[self.index];
        self.index += 1;
        Ok(c)
    }

    pub fn read_usize(&mut self) -> Result<usize, &str> {
        let c = self.read()?
            .to_digit(35)
            .ok_or("Failed to parse digit")?;
        usize::try_from(c).map_err(|_| "Failed to convert digit to usize")
    }

    pub fn skip_line(&mut self) -> Result<(), &str> {
        loop {
            match self.read() {
                Ok(c) => {
                    if c == '\n' {
                        break;
                    }
                },
                Err(_) => {
                    break;
                }
            }
        }
        Ok(())
    }

    // Unneeded code, might be required later on
    //
    // fn peek(&mut self) -> Result<char, &str> {
    //     Ok(self.string[self.index + 1])
    // }

    // fn readWord(&mut self) -> Result<String, &str> {
    //     let mut out = String::new();
    //     loop {
    //         match self.peek() {
    //             Ok(c) => {
    //                 match c {
    //                     ' ' => {
    //                         if out.len() != 0 {
    //                             break;
    //                         }
    //                     },
    //                     _ => {
    //                         out.push(self.read().unwrap());
    //                     }
    //                 }
    //             },
    //             Err(_) => {
    //                 break;
    //             }
    //         }
    //     }
    //     Ok(out)
    // }

    //    fn readTo(&mut self, to: char) -> Result<String, &str> {
    //        let mut out = String::new();
    //        loop {
    //            match self.read() {
    //                Ok(c) => {
    //                    out.push(c);
    //                    if c == to {
    //                        break;
    //                    }
    //                },
    //                Err(_) => { break; }
    //            }
    //        }
    //        Ok(out)
    //    }
}
