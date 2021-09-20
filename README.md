# short_url-rs
short_url is a crate that is ported from [https://github.com/Alir3z4/python-short_url](python-short_url) into Rust.

# Quickstart
```rust
use short_url::UrlEncoder;

let encoder = UrlEncoder::new("mn6j2c4rv8bpygw95z7hsdaetxuk3fq", 24);

// Encode a url with id 1, and a min length of 5.
assert_eq!(String::from("867nv"), e.encode_url(1, 5));

// Decode a url into a usize.
assert_eq!(1, e.decode_url("867nv").unwrap());
```

# License
Licensed under MIT as inherited from the orginal work at [https://github.com/Alir3z4/python-short_url](python-short_url).