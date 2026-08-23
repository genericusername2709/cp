#[derive(Default)]
pub struct Reader {
    buffer: Vec<String>,
}

#[allow(dead_code)]
impl Reader {
    // Reads next token from stdin - either from the current line - that is present in the #buffer
    // or from reading new a line.
    //
    // It panics if the expected type of token is not present.
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed to parse!");
            }
            self.fill_buff();
        }
    }

    pub fn read_chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }

    // Reads and stores elems into a array, until first non-empty line ends.
    pub fn read_arr<T: std::str::FromStr>(&mut self) -> Vec<T> {
        let mut len = self.buffer.len();
        while len == 0 {
            self.fill_buff();
            len = self.buffer.len();
        }
        self.buffer
            .drain(..)
            .map(|e| e.parse().ok().expect("Failed to parse!"))
            .rev()
            .collect()
    }

    // Reads line, and puts it into the buffer
    fn fill_buff(&mut self) {
        let input = self.read_line();
        self.buffer = input.split_whitespace().rev().map(String::from).collect();
    }

    pub fn read_line(&self) -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input
    }
}
