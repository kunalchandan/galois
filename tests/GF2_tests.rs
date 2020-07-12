#![allow(non_snake_case)]
extern crate Galois;
use Galois::linear::GF2::*;

#[test]
fn test_GF2_add10() {
    let x = GF2{value: true};
    let y = GF2{value: false};
    assert_eq!(x + y, true);
}
#[test]
fn test_GF2_add11() {
    let x = GF2{value: true};
    let y = GF2{value: true};
    assert_eq!(x + y, false);
}
#[test]
fn test_GF2_mul10() {
    let x = GF2{value: true};
    let y = GF2{value: false};
    assert_eq!(x * y, false);
}
#[test]
fn test_GF2_mul11() {
    let x = GF2{value: true};
    let y = GF2{value: true};
    assert_eq!(x * y, true);
}
