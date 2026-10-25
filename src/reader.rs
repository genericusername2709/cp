use std::io::{stdin, BufRead, BufReader, Stdin};

#[derive(Default)]
pub struct Reader<R> {
    reader: R,
    buffer: Vec<u8>,
    cursor: usize,
}

impl Reader<BufReader<Stdin>> {
    pub fn new() -> Self {
        Self {
            reader: BufReader::with_capacity(1 << 20, stdin()), // 1MB buffer
            buffer: Vec::new(),
            cursor: 0,
        }
    }
}

#[allow(unused)]
impl<R: BufRead> Reader<R> {
    #[inline]
    fn next_token_slice(&mut self) -> Option<(usize, usize)> {
        loop {
            while self.cursor < self.buffer.len() && self.buffer[self.cursor] <= b' ' {
                self.cursor += 1;
            }

            if self.cursor < self.buffer.len() {
                let start = self.cursor;
                while self.cursor < self.buffer.len() && self.buffer[self.cursor] > b' ' {
                    self.cursor += 1;
                }
                return Some((start, self.cursor));
            }

            self.buffer.clear();
            self.cursor = 0;
            if self.reader.read_until(b'\n', &mut self.buffer).unwrap() == 0 {
                return None;
            }
        }
    }

    /// Reads the next single token
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        let (start, end) = self.next_token_slice().expect("Unexpected end of input");
        let token_str = std::str::from_utf8(&self.buffer[start..end]).unwrap();
        token_str.parse().ok().expect("Failed to parse!")
    }

    pub fn read_chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }

    /// Reads the next integer token
    pub fn next_int<T: FastParsable>(&mut self) -> T {
        let (start, end) = self.next_token_slice().expect("Unexpected end of input");
        FastParsable::parse_bytes(&self.buffer[start..end]).expect("Failed to parse integer!")
    }

    #[inline]
    fn fill_line_if_empty(&mut self) -> bool {
        while self.cursor < self.buffer.len() && self.buffer[self.cursor] <= b' ' {
            self.cursor += 1;
        }

        if self.cursor >= self.buffer.len() {
            self.buffer.clear();
            self.cursor = 0;
            if self.reader.read_until(b'\n', &mut self.buffer).unwrap() == 0 {
                return false; // EOF
            }

            while self.cursor < self.buffer.len() && self.buffer[self.cursor] <= b' ' {
                self.cursor += 1;
            }
        }
        self.cursor < self.buffer.len()
    }

    #[inline]
    fn next_line_token_slice(&mut self) -> Option<(usize, usize)> {
        // Skip whitespace on the current line
        while self.cursor < self.buffer.len() && self.buffer[self.cursor] <= b' ' {
            self.cursor += 1;
        }

        if self.cursor >= self.buffer.len() {
            return None;
        }

        let start = self.cursor;
        while self.cursor < self.buffer.len() && self.buffer[self.cursor] > b' ' {
            self.cursor += 1;
        }

        Some((start, self.cursor))
    }

    /// Fast reading of a full array line
    pub fn read_arr<T: std::str::FromStr>(&mut self) -> Vec<T> {
        let mut arr = Vec::new();
        if !self.fill_line_if_empty() {
            return arr;
        }

        while let Some((start, end)) = self.next_line_token_slice() {
            let token_str = std::str::from_utf8(&self.buffer[start..end]).unwrap();
            arr.push(token_str.parse().ok().expect("Failed to parse!"));
        }

        arr
    }

    /// Fast reading of a full array line
    pub fn read_vec<T: std::str::FromStr>(&mut self, len: usize) -> Vec<T> {
        let mut arr = Vec::with_capacity(len);
        if !self.fill_line_if_empty() {
            return arr;
        }

        while let Some((start, end)) = self.next_line_token_slice() {
            let token_str = std::str::from_utf8(&self.buffer[start..end]).unwrap();
            arr.push(token_str.parse().ok().expect("Failed to parse!"));
        }

        arr
    }

    /// Fast reading of a full array line
    pub fn read_arr_int<T: FastParsable>(&mut self) -> Vec<T> {
        let mut arr = Vec::new();
        if !self.fill_line_if_empty() {
            return arr;
        }

        while let Some((start, end)) = self.next_line_token_slice() {
            let val = FastParsable::parse_bytes(&self.buffer[start..end])
                .expect("Failed to parse integer!");
            arr.push(val);
        }

        arr
    }

    /// Fast reading of a full array line
    pub fn read_vec_int<T: FastParsable>(&mut self, len: usize) -> Vec<T> {
        let mut arr = Vec::with_capacity(len);
        if !self.fill_line_if_empty() {
            return arr;
        }

        while let Some((start, end)) = self.next_line_token_slice() {
            let val = FastParsable::parse_bytes(&self.buffer[start..end])
                .expect("Failed to parse integer!");
            arr.push(val);
        }

        arr
    }
}

pub trait FastParsable: Sized {
    fn parse_bytes(bytes: &[u8]) -> Option<Self>;
}

// Single macro generating the byte-parsing loop for any unsigned type
macro_rules! impl_fast_parse_unsigned {
    ($($t:ty),*) => {
        $(
            impl FastParsable for $t {
                #[inline]
                fn parse_bytes(bytes: &[u8]) -> Option<Self> {
                    if bytes.is_empty() { return None; }
                    let mut val: $t = 0;
                    for &b in bytes {
                        if b < b'0' || b > b'9' { return None; }
                        val = val * 10 + (b - b'0') as $t;
                    }
                    Some(val)
                }
            }
        )*
    };
}

// Single macro generating the byte-parsing loop for any signed type
macro_rules! impl_fast_parse_signed {
    ($($t:ty),*) => {
        $(
            impl FastParsable for $t {
                #[inline]
                fn parse_bytes(bytes: &[u8]) -> Option<Self> {
                    if bytes.is_empty() { return None; }
                    let (neg, rest) = if bytes[0] == b'-' {
                        (true, &bytes[1..])
                    } else {
                        (false, bytes)
                    };
                    let mut val: $t = 0;
                    for &b in rest {
                        if b < b'0' || b > b'9' { return None; }
                        val = val * 10 + (b - b'0') as $t;
                    }
                    Some(if neg { -val } else { val })
                }
            }
        )*
    };
}

// Apply the implementation to all desired types in a single line each
impl_fast_parse_unsigned!(usize, u8, u16, u32, u64, u128);
impl_fast_parse_signed!(isize, i8, i16, i32, i64, i128);

#[macro_export]
macro_rules! parr {
    ($writer: expr, $arr: expr) => {
        for i in 0..$arr.len() {
            write!(
                $writer,
                "{}{}",
                $arr[i],
                if i == $arr.len() { " " } else { "\n" }
            );
        }
    };
}

#[macro_export]
macro_rules! pen {
    ($writer: expr, $elem: expr) => {
        write!($writer, "{}\n", $elem);
    };
}

#[macro_export]
macro_rules! pes {
    ($writer: expr, $elem: expr) => {
        write!($writer, "{} ", $elem);
    };
}
