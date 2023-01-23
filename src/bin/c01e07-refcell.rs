#[allow(unused)]
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::{
        atomic::{Ordering::*, *},
        *,
    },
    thread::{self, Thread},
};

fn f(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1); // We can modify the `Vec` directly
}

fn main() {
    let v = RefCell::new(vec![1, 2, 3]);
    f(&v);
    assert_eq!(v.into_inner(), vec![1, 2, 3, 1]);
}
