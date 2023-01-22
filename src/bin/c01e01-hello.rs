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

fn main() {
    thread::spawn(f);
    thread::spawn(f);

    println!("Aum Namah Sivaya!!! Main thread");
}

fn f() {
    println!("Hara Hara Mahadev!!! Child thread");

    let id = thread::current().id();
    println!("Siva Shambho! Thread id: {id:?}");
}
