#![allow(dead_code)]

use ezrs::*;

#[derive(Constructor, Add, Not, Neg, Mul, Sum, PartialEq)]
struct MyInts(i32, i32);

#[derive(Constructor, Add, Not, Neg, Mul, AddAssign, MulAssign)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Index, IndexMut)]
struct MyVec(Vec<i32>);

#[derive(Index, IndexMut)]
struct Numbers {
    #[index]
    #[index_mut]
    numbers: Vec<i32>,
    useless: bool,
}

#[derive(Deref, DerefMut)]
struct Num {
    num: i32,
}

#[derive(Deref, DerefMut)]
#[deref(forward)]
#[deref_mut(forward)]
struct MyBoxedInt(Box<i32>);

#[derive(Deref, DerefMut)]
struct CoolVec {
    cool: bool,
    #[deref]
    #[deref_mut]
    vec: Vec<i32>,
}

#[test]
fn test_derive_more_constructor() {
    let a1 = MyInts::new(1, 2);
    let a2 = MyInts(1, 2);
    assert_eq!(a1.0, a2.0);
    assert_eq!(a1.1, a2.1);

    let p1 = Point2D::new(100, -100);
    let p2 = Point2D { x: 100, y: -100 };
    assert_eq!(p1.x, p2.x);
    assert_eq!(p1.y, p2.y);
}

#[test]
fn test_derive_more_index_and_indexmut() {
    let mut myvec = MyVec(vec![5, 8]);
    assert_eq!(5, myvec[0]);
    myvec[0] = 50;
    assert_eq!(50, myvec[0]);

    let mut numbers = Numbers {
        numbers: vec![100, 200],
        useless: false,
    };
    assert_eq!(200, numbers[1]);
    numbers[1] = 400;
    assert_eq!(400, numbers[1]);
}

#[test]
fn test_derive_more_deref_and_derefmut() {
    let mut num = Num { num: 123 };
    assert_eq!(123, *num);
    *num += 123;
    assert_eq!(246, *num);

    let mut boxed = MyBoxedInt(Box::new(123));
    assert_eq!(123, *boxed);
    *boxed += 1000;
    assert_eq!(1123, *boxed);

    let mut cool_vec = CoolVec {
        cool: true,
        vec: vec![123],
    };
    assert_eq!(vec![123], *cool_vec);
    cool_vec.push(456);
    assert_eq!(vec![123, 456], *cool_vec);
}

#[test]
fn test_derive_more_not_and_neg() {
    let a1 = !MyInts(-2, 3);
    let a2 = MyInts(1, -4);
    assert_eq!(a1.0, a2.0);
    assert_eq!(a1.1, a2.1);

    let a3 = -MyInts(3, 0);
    let a4 = MyInts(-3, 0);
    assert_eq!(a3.0, a4.0);
    assert_eq!(a3.1, a4.1);
}

#[test]
fn test_derive_more_add_and_addassign() {
    let p1 = Point2D::new(1, 2);
    let p2 = Point2D::new(3, 4);
    let p3 = p1 + p2;
    assert_eq!(p3.x, 4);
    assert_eq!(p3.y, 6);

    let p4 = Point2D::new(1, 2);
    let mut p5 = Point2D::new(3, 4);
    p5 += p4;
    assert_eq!(p3.x, p5.x);
    assert_eq!(p3.y, p5.y);
}

#[test]
fn test_derive_more_mul_and_mulassign() {
    let p1 = Point2D::new(1, 2);
    let p2 = Point2D::new(3, 6);
    let p3 = p1 * 3;
    assert_eq!(p2.x, p3.x);
    assert_eq!(p2.y, p3.y);

    let mut p4 = Point2D::new(1, 2);
    p4 *= 3;
    assert_eq!(p3.x, p4.x);
    assert_eq!(p3.y, p4.y);
}

#[test]
fn test_derive_more_sum() {
    let int_vec = vec![MyInts(2, 3), MyInts(4, 5), MyInts(6, 7)];
    assert!(MyInts(12, 15) == int_vec.into_iter().sum())
}
