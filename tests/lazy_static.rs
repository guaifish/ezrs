use ezrs::*;
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    static ref NUMBER: u32 = times_two(21);
}

fn times_two(n: u32) -> u32 {
    n * 2
}

#[test]
fn test_lazy_static() {
    assert_eq!(*COUNT, 3);
    assert_eq!(*NUMBER, 42);
    assert_eq!(HASHMAP[&0], "foo");
    assert_eq!(HASHMAP[&1], "bar");
    assert_eq!(HASHMAP[&2], "baz");
}
