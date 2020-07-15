use num_traits::{Num, NumCast};

pub mod err;

type Result<T> = std::result::Result<T, err::ShortError>;

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
            mapping: (0..block).map(std::convert::From::from).collect(),
        }
    }

    pub fn encode_url<N: Num + NumCast>(&self, n: N, min_length: usize) -> Result<String> {
        return Ok(self.enbase(
            self.encode(NumCast::from(n).ok_or_else(|| err::ShortError::InvalidNumber)?),
            min_length,
        ));
    }

    pub fn decode_url(&self, x: String) -> Result<usize> {
        Ok(self.decode(self.debase(x)?))
    }

    fn encode(&self, n: usize) -> usize {
        n & !self.mask | self._encode(n & self.mask)
    }

    fn _encode(&self, n: usize) -> usize {
        let mut result = 0;

        self.mapping.iter().rev().enumerate().for_each(|(i, b)| {
            if n & (1 << i) != 0 {
                result |= 1 << b;
            }
        });

        result
    }

    fn decode(&self, n: usize) -> usize {
        (n & !self.mask) | self._decode(n & self.mask)
    }

    fn _decode(&self, n: usize) -> usize {
        let mut result = 0;

        self.mapping.iter().rev().enumerate().for_each(|(i, b)| {
            if n & (1 << b) != 0 {
                result |= 1 << i;
            }
        });

        result
    }

    fn enbase(&self, x: usize, min_length: usize) -> String {
        let result = self._enbase(x);

        let pad_num = match min_length < result.chars().count() {
            true => 0,
            false => min_length - result.chars().count(),
        };

        let padding = self.alphabet[0].to_string().repeat(pad_num);

        format!("{}{}", padding, result)
    }

    fn _enbase(&self, x: usize) -> String {
        let n = self.alphabet.len();
        if x < n {
            return self.alphabet[x].to_string();
        }

        let mut rt: Vec<char> = Vec::new();
        let mut xx = x;

        while !(xx < n) {
            rt.insert(0, self.alphabet[xx % n]);
            xx /= n;
        }

        rt.insert(0, self.alphabet[xx]);

        rt.into_iter().collect()
    }

    fn debase(&self, x: String) -> Result<usize> {
        let n = self.alphabet.len();
        let mut result = 0;

        for (i, c) in x.chars().into_iter().rev().enumerate() {
            result += self
                .alphabet
                .iter()
                .position(|&x| x == c)
                .ok_or_else(|| err::ShortError::InvalidString)?
                * (n.pow(i as u32))
        }

        Ok(result)
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

        assert_eq!(String::from("867nv"), e.encode_url(1, 5).unwrap());
    }

    #[test]
    fn decode_url() {
        let e = UrlEncoder::new(
            "mn6j2c4rv8bpygw95z7hsdaetxuk3fq".into(),
            super::DEFAULT_BLOCK_SIZE,
        );

        assert_eq!(1, e.decode_url("867nv".into()).unwrap());
    }
}
