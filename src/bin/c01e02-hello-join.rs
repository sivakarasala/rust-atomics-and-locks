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
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Aum Namah Sivaya!!! Main thread");

    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hara Hara Mahadev!!! Child thread");

    let id = thread::current().id();
    println!("Siva Shambho! Thread id: {id:?}");
}
