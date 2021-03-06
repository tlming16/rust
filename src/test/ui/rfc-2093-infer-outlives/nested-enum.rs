// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustc_attrs)]
#![feature(infer_outlives_requirements)]


#[rustc_outlives]
enum Foo<'a, T> { //~ ERROR 16:1: 19:2: rustc_outlives

    One(Bar<'a, T>)
}

struct Bar<'b, U> {
    field2: &'b U
}

fn main() {}
