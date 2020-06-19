pub static DEFAULT_ALPHABET: &str = "mn6j2c4rv8bpygw95z7hsdaetxuk3fq";
pub static DEFAULT_BLOCK_SIZE: usize = 24;
pub static MIN_LENGTH: usize = 5;

pub struct UrlEncoder {
    alphabet: Vec<char>,
    mask: usize,
    mapping: Vec<usize>,
}

impl UrlEncoder {
    pub fn new(alp: String, block: usize) -> Self {
        Self {
            alphabet: alp.chars().collect(),
            mask: (1 << block) - 1,
            mapping: (0..block).map(usize::from).collect(),
        }
    }

    pub fn encode_url(&self, n: usize, min_length: usize) -> String {
        return self.enbase(self.encode(n), min_length);
    }

    pub fn decode_url(&self, x: String) -> usize {
        self.decode(self.debase(x))
    }
    fn encode(&self, n: usize) -> usize {
        n & !self.mask | self._encode(n & self.mask)
    }

    fn _encode(&self, n: usize) -> usize {
        let mut result = 0;
        for (i, b) in self.mapping.iter().rev().enumerate() {
            if n & (1 << i) != 0 {
                result |= 1 << b;
            } else {
                continue;
            }
        }
        result
    }

    fn decode(&self, n: usize) -> usize {
        (n & !self.mask) | self._decode(n & self.mask)
    }

    fn _decode(&self, n: usize) -> usize {
        let mut result = 0;
        for (i, b) in self.mapping.iter().rev().enumerate() {
            if n & (1 << b) != 0 {
                result |= 1 << i;
            }
        }
        result
    }

    fn enbase(&self, x: usize, min_length: usize) -> String {
        let result = self._enbase(x);

        let pad_num = if min_length < result.chars().count() {
            0
        } else {
            min_length - result.chars().count()
        };

        let padding = self.alphabet[0].to_string().repeat(pad_num);

        format!("{}{}", padding, result)
    }

    fn _enbase(&self, x: usize) -> String {
        let n = self.alphabet.len();
        if x < n as usize {
            return vec![self.alphabet[x as usize]].into_iter().collect();
        }

        let mut rt: Vec<char> = vec![];
        let mut xx = x;
        loop {
            if xx < n {
                rt.insert(0, self.alphabet[xx]);
                break;
            }
            rt.insert(0, self.alphabet[xx % n]);
            xx /= n;
        }

        rt.into_iter().collect()
    }

    fn debase(&self, x: String) -> usize {
        let n = self.alphabet.len();
        let mut result = 0;
        for (i, c) in x.chars().into_iter().rev().enumerate() {
            result += self.alphabet.iter().position(|&x| x == c).unwrap() * (n.pow(i as u32))
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::UrlEncoder;

    #[test]
    fn encode_url() {
        let e = UrlEncoder::new(
            "mn6j2c4rv8bpygw95z7hsdaetxuk3fq".into(),
            super::DEFAULT_BLOCK_SIZE,
        );

        assert_eq!(String::from("867nv"), e.encode_url(1, 5));
    }

    #[test]
    fn decode_url() {
        let e = UrlEncoder::new(
            "mn6j2c4rv8bpygw95z7hsdaetxuk3fq".into(),
            super::DEFAULT_BLOCK_SIZE,
        );

        assert_eq!(1, e.decode_url("867nv".into()));
    }
}
