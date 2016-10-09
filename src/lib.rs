#![no_std]
#![feature(oom,
           sip_hash_13,
           heap_api,
           unique,
           dropck_parametricity,
           core_intrinsics,
           float_extras,
           libc,
           lang_items,
           alloc,
           collections,
           slice_concat_ext,
           macro_reexport,
           allow_internal_unstable,
           shared,
           prelude_import,
           question_mark,
           const_fn,
           try_from,
           try_borrow,
           reflect_marker,
           int_error_internals,
           unicode,
           fused,
           stmt_expr_attributes,
           zero_one,
           fnbox,
           optin_builtin_traits,
           box_syntax,
           cfg_target_thread_local,
           drop_types_in_const)]

extern crate alloc;
#[macro_reexport(format, vec)]
#[macro_use]
extern crate collections;
extern crate libc;
extern crate rustc_unicode;

pub mod game;
pub mod system;
pub mod warping;
pub mod link;

pub type Addr = libc::size_t;
pub use link::Link;

#[prelude_import]
#[allow(unused)]
use prelude::*;

pub mod std {
    pub use core::{any, cell, char, clone, cmp, convert, default, hash, i16, i32, i64, i8, isize,
                   iter, marker, mem, ops, option, ptr, result, u16, u32, u64, u8, usize,
                   intrinsics};
    pub use alloc::{arc, rc};
    pub use collections::{borrow, boxed, fmt, slice, str, string, vec};
    pub use system::{error, io, fs, ascii, time, num, thread, sync, ffi, path};
    #[path = "../system/num/f32.rs"]    pub mod f32;
    #[path = "../system/num/f64.rs"]    pub mod f64;

    pub mod collections {
        pub use collections::{binary_heap, btree_map, btree_set, linked_list, vec_deque,
                              BinaryHeap, LinkedList, VecDeque, String, Vec, BTreeMap, BTreeSet};
        pub use system::hash::map::HashMap;
        pub use system::hash::set::HashSet;
        pub use system::hash::map as hash_map;
        pub use system::hash::set as hash_set;
    }
}

pub mod prelude {
    pub use Coord;

    pub use std;
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
    pub use std::fmt::Write;
}

pub use std::{fmt, boxed, vec};

#[derive(Clone)]
#[repr(C, packed)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}, {:.2}, {:.2}", self.x, self.y, self.z)
    }
}
