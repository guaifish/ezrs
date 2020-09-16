use ezrs::rand::seq::SliceRandom;
use ezrs::rand::thread_rng;
use ezrs::*;

#[test]
fn test_rand_random() {
    let _: bool = random();
    let _: i32 = random();
    let _: f64 = random();
    let _: (char, u8) = random();
    let _: [i32; 3] = random();
}

#[test]
fn test_rand_shuffle() {
    let mut rng = thread_rng();
    let mut y = [1, 2, 3, 4, 5];
    let expect = [1, 2, 3, 4, 5];
    y.shuffle(&mut rng);
    if y != expect {
        y.sort();
        assert_eq!(y, expect);
    }
}

#[test]
fn test_rand_choose() {
    let choices = [1, 2, 4, 8, 16, 32];
    let mut rng = thread_rng();
    for _ in 0..10 {
        let x = choices.choose(&mut rng).unwrap();
        assert!(choices.contains(x));
    }
    assert_eq!(choices[..0].choose(&mut rng), None);
}
