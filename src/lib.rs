// The MIT License (MIT)

// Copyright (c) 2014 Y. T. CHUNG

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Ini parser for Rust
//!
//! ```no_run
//! use ini::Ini;
//!
//! let mut conf = Ini::new();
//! conf.begin_section("User")
//!    .set("name", "Raspberry树莓")
//!    .set("value", "Pi")
//!    .end_section();
//! conf.begin_section("Library")
//!     .set("name", "Sun Yat-sen U")
//!    .set("location", "Guangzhou=world")
//!    .end_section();
//! conf.write_to_file("conf.ini").unwrap();
//!
//!
//! let i = Ini::load_from_file("conf.ini").unwrap();
//! for (sec, prop) in i.iter() {
//!    println!("Section: {}", *sec);
//!    for (k, v) in prop.iter() {
//!        println!("{}:{}", *k, *v);
//!    }
//! }
//! ```

#![allow(unstable)]
#[macro_use] extern crate log;

pub use ini::Ini;
pub mod ini;
