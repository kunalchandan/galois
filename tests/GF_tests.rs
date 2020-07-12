#![allow(non_snake_case)]
extern crate Galois;
use Galois::linear::GF::*;

#[test]
fn test_GF_add() {
    let x = GF{value: true};
    let y = GF{value: false};
    assert_eq!(x + y, true);
}
#[test]
fn test_GF_add11() {
    let x = GF{value: true};
    let y = GF{value: true};
    assert_eq!(x + y, false);
}
#[test]
fn test_GF_mul10() {
    let x = GF{value: true};
    let y = GF{value: false};
    assert_eq!(x * y, false);
}
#[test]
fn test_GF_mul11() {
    let x = GF2{value: true};
    let y = GF2{value: true};
    assert_eq!(x * y, true);
}
