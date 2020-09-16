use core::str::FromStr;
use ezrs::*;

#[test]
fn test_rust_decimal() {
    assert_eq!(dec!(2.1) + dec!(4.2), dec!(6.3));
    assert_ne!(2.1 + 4.2, 6.3);

    let d1 = dec!(2.02);
    let d2 = Decimal::new(202, 2);
    assert_eq!(d1, d2);

    let d3 = Decimal::from_str("2").unwrap();
    let d4: Decimal = 2i32.into();
    assert_eq!(d3, d4);
}
