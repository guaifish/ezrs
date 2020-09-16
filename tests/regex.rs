use ezrs::*;

#[test]
fn test_regex_is_match() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("2014-01-01"));
}

#[test]
fn test_regex_replace_all() {
    let re = Regex::new(
        r"(?x)
    (?P<y>\d{4}) # the year
    -
    (?P<m>\d{2}) # the month
    -
    (?P<d>\d{2}) # the day
    ",
    )
    .unwrap();
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    let after = re.replace_all(before, "$m/$d/$y");
    assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");
}
