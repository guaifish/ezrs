use ezrs::*;

#[test]
fn test_counter() {
    let char_counts: Counter<_> = "barefoot".chars().collect();
    assert_eq!(char_counts[&'a'], 1);
    assert_eq!(char_counts[&'o'], 2);
    assert_eq!(char_counts[&'c'], 0);

    let counts: Counter<&str> = "able babble table babble rabble table able fable scrabble"
        .split_whitespace()
        .collect();
    assert_eq!(counts[&"able"], 2);
    assert_eq!(counts[&"rabble"], 1);
    assert_eq!(counts[&"a"], 0);

    let by_common = "eaddbbccc"
        .chars()
        .collect::<Counter<_>>()
        .most_common_ordered();
    let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
    assert_eq!(by_common, expected);
}
