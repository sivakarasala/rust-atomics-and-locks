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
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    thread::spawn(move || dbg!(x)).join().unwrap();
    thread::spawn(move || dbg!(x)).join().unwrap();
    let a = Rc::new([1, 2, 3]);
    let b = Rc::clone(&a);
    let c = &b;
    let d = &a;

    println!("{:p}", b);
    println!("{:p}", a);
    println!("{:p}", c);
    println!("{:p}", d);
    assert_eq!(a.as_ptr(), b.as_ptr());

    let x = Arc::new([11, 22, 33]);
    let y = Arc::clone(&x);
    thread::spawn(move || dbg!(x)).join().unwrap();
    thread::spawn(move || dbg!(y)).join().unwrap();

    let a = Arc::new([1, 2, 3]);
    thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    })
    .join()
    .unwrap();
    dbg!(a);
}
