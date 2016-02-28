#![no_std]
#![feature(libc, lang_items, alloc, collections, slice_concat_ext, macro_reexport)]

extern crate alloc;
#[macro_reexport(format, vec)]
extern crate collections;
extern crate libc;

pub mod memory;
pub mod controller;
pub mod system;
pub mod warp;
pub mod flags;
pub mod stages;
pub mod items;
pub mod link;
pub mod inventory;
pub mod console;

pub type Addr = libc::size_t;

pub mod std {
    pub use core::{any, cell, char, clone, cmp, convert, default, f32, f64, hash, i16, i32, i64,
                   i8, isize, iter, marker, mem, num, ops, option, ptr, result, sync, u16, u32,
                   u64, u8, usize};
    pub use alloc::{arc, rc};
    pub use collections::{borrow, boxed, fmt, slice, str, string, vec};

    pub mod collections {
        pub use collections::{binary_heap, btree_map, btree_set, linked_list, vec_deque,
                              BinaryHeap, LinkedList, VecDeque, String, Vec, BTreeMap, BTreeSet};
    }
}

pub use std::boxed;
pub use std::fmt;

pub mod prelude {
    pub use std::marker::{Copy, Send, Sized, Sync};
    pub use std::ops::{Drop, Fn, FnMut, FnOnce};
    pub use std::mem::drop;
    pub use std::boxed::Box;
    pub use std::borrow::ToOwned;
    pub use std::clone::Clone;
    pub use std::cmp::{PartialEq, PartialOrd, Eq, Ord};
    pub use std::convert::{AsRef, AsMut, Into, From};
    pub use std::default::Default;
    pub use std::iter::{Iterator, Extend, IntoIterator};
    pub use std::iter::{DoubleEndedIterator, ExactSizeIterator};
    pub use std::option::Option::{self, Some, None};
    pub use std::result::Result::{self, Ok, Err};
    pub use std::slice::SliceConcatExt;
    pub use std::string::{String, ToString};
    pub use std::vec::Vec;
}
