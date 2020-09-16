use ezrs::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[test]
fn test_hashmap() {
    let m1 = hashmap! {
        'a' => 1,
        'b' => 2,
    };
    let mut m2 = HashMap::new();
    m2.insert('a', 1);
    m2.insert('b', 2);
    assert_eq!(m1, m2);
}

#[test]
fn test_btreemap() {
    let m1 = btreemap! {
        'a' => 1,
        'b' => 2,
    };
    let mut m2 = BTreeMap::new();
    m2.insert('a', 1);
    m2.insert('b', 2);
    assert_eq!(m1, m2);
}

#[test]
fn test_hashset() {
    let s1 = hashset! { 'a', 'b' };
    let mut s2 = HashSet::new();
    s2.insert('a');
    s2.insert('b');
    assert_eq!(s1, s2);
}

#[test]
fn test_btreeset() {
    let s1 = btreeset! { 'a', 'b' };
    let mut s2 = BTreeSet::new();
    s2.insert('a');
    s2.insert('b');
    assert_eq!(s1, s2);
}
