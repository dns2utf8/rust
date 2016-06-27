// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(unboxed_closures)]

trait Three<A,B,C> { fn dummy(&self) -> (A,B,C); }

fn foo(_: &Three())
//~^ ERROR wrong number of generic type arguments
//~| ERROR associated type `Output` not found
{}

fn main() { }
