#![allow(dead_code)]
#![allow(raw_pointer_derive)] 
// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// FIXME: talk about offset, copy_memory, copy_nonoverlapping_memory

//! Operations on unsafe pointers, `*const T`, and `*mut T`.
//!
//! Working with unsafe pointers in Rust is uncommon,
//! typically limited to a few patterns.
//!
//! Use the `null` function to create null pointers, and the `is_null` method
//! of the `*const T` type  to check for null. The `*const T` type also defines
//! the `offset` method, for pointer math.
//!
//! # Common ways to create unsafe pointers
//!
//! ## 1. Coerce a reference (`&T`) or mutable reference (`&mut T`).
//!
//! ```
//! let my_num: i32 = 10;
//! let my_num_ptr: *const i32 = &my_num;
//! let mut my_speed: i32 = 88;
//! let my_speed_ptr: *mut i32 = &mut my_speed;
//! ```
//!
//! To get a pointer to a boxed value, dereference the box:
//!
//! ```
//! let my_num: Box<i32> = Box::new(10);
//! let my_num_ptr: *const i32 = &*my_num;
//! let mut my_speed: Box<i32> = Box::new(88);
//! let my_speed_ptr: *mut i32 = &mut *my_speed;
//! ```
//!
//! This does not take ownership of the original allocation
//! and requires no resource management later,
//! but you must not use the pointer after its lifetime.
//!
//! ## 2. Consume a box (`Box<T>`).
//!
//! The `into_raw` function consumes a box and returns
//! the raw pointer. It doesn't destroy `T` or deallocate any memory.
//!
//! ```
//! # #![feature(alloc)]
//! use std::boxed;
//!
//! unsafe {
//!     let my_speed: Box<i32> = Box::new(88);
//!     let my_speed: *mut i32 = boxed::into_raw(my_speed);
//!
//!     // By taking ownership of the original `Box<T>` though
//!     // we are obligated to put it together later to be destroyed.
//!     drop(Box::from_raw(my_speed));
//! }
//! ```
//!
//! Note that here the call to `drop` is for clarity - it indicates
//! that we are done with the given value and it should be destroyed.
//!
//! ## 3. Get it from C.
//!
//! ```
//! # #![feature(libc)]
//! extern crate libc;
//!
//! use std::mem;
//!
//! fn main() {
//!     unsafe {
//!         let my_num: *mut i32 = libc::malloc(mem::size_of::<i32>() as libc::size_t) as *mut i32;
//!         if my_num.is_null() {
//!             panic!("failed to allocate memory");
//!         }
//!         libc::free(my_num as *mut libc::c_void);
//!     }
//! }
//! ```
//!
//! Usually you wouldn't literally use `malloc` and `free` from Rust,
//! but C APIs hand out a lot of pointers generally, so are a common source
//! of unsafe pointers in Rust.


use std::marker::{PhantomData, Send, Sized, Sync};



/// A wrapper around a raw `*mut T` that indicates that the possessor
/// of this wrapper owns the referent. This in turn implies that the
/// `Unique<T>` is `Send`/`Sync` if `T` is `Send`/`Sync`, unlike a raw
/// `*mut T` (which conveys no particular ownership semantics).  It
/// also implies that the referent of the pointer should not be
/// modified without a unique path to the `Unique` reference. Useful
/// for building abstractions like `Vec<T>` or `Box<T>`, which
/// internally use raw pointers to manage the memory that they own.
#[derive(Debug)]
pub struct Unique<T: Sized> {
    pointer: *mut T,
    _marker: PhantomData<T>,
}

/// `Unique` pointers are `Send` if `T` is `Send` because the data they
/// reference is unaliased. Note that this aliasing invariant is
/// unenforced by the type system; the abstraction using the
/// `Unique` must enforce it.
unsafe impl<T: Send + Sized> Send for Unique<T> { }

/// `Unique` pointers are `Sync` if `T` is `Sync` because the data they
/// reference is unaliased. Note that this aliasing invariant is
/// unenforced by the type system; the abstraction using the
/// `Unique` must enforce it.
unsafe impl<T: Sync + Sized> Sync for Unique<T> { }

impl<T: Sized> Unique<T> {
    /// Creates a new `Unique`.
    pub unsafe fn new(ptr: *mut T) -> Unique<T> {
        Unique { pointer: ptr, _marker: PhantomData }
    }

    /// Dereferences the content.
    pub unsafe fn get(&self) -> &T {
        &*self.pointer
    }

    /// Mutably dereferences the content.
    pub unsafe fn get_mut(&mut self) -> &mut T {
        //&mut ***self
        &mut *self.pointer
    }
}

/*
impl<T:Sized> Deref for Unique<T> {
    type Target = *mut T;

    #[inline]
    fn deref<'a>(&'a self) -> &'a *mut T {
        unsafe { mem::transmute(&*self.pointer) }
    }
}*/
