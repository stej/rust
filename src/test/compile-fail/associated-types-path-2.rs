// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test type checking of uses of associated types via sugary paths.

pub trait Foo {
    type A;
}

impl Foo for int {
    type A = uint;
}

pub fn f1<T: Foo>(a: T, x: T::A) {}
pub fn f2<T: Foo>(a: T) -> T::A {
    panic!();
}

pub fn f1_int_int() {
    f1(2i, 4i);
    //~^ ERROR expected uint, found int
}

pub fn f1_int_uint() {
    f1(2i, 4u);
}

pub fn f1_uint_uint() {
    f1(2u, 4u);
    //~^ ERROR the trait `Foo` is not implemented
    //~| ERROR the trait `Foo` is not implemented
}

pub fn f1_uint_int() {
    f1(2u, 4i);
    //~^ ERROR the trait `Foo` is not implemented
    //~| ERROR the trait `Foo` is not implemented
}

pub fn f2_int() {
    let _: int = f2(2i);
    //~^ ERROR expected `int`, found `uint`
}

pub fn main() { }
