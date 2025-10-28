use std::cell::{Cell, UnsafeCell};

pub struct MyRefCell<T> {
    value: UnsafeCell<T>,
    borrow: Cell<isize>, // 0 = free, >0 = immutable count, -1 = mut borrow
}

// ---------- Basic implementation ----------
impl<T> MyRefCell<T> {
    pub fn new(value: T) -> Self {
        MyRefCell {
            value: UnsafeCell::new(value),
            borrow: Cell::new(0),
        }
    }

    pub fn borrow(&self) -> MyRef<'_, T> {
        let count = self.borrow.get();
        if count == -1 {
            panic!("already mutably borrowed!");
        }
        self.borrow.set(count + 1);
        MyRef { refcell: self }
    }

    pub fn borrow_mut(&self) -> MyRefMut<'_, T> {
        let count = self.borrow.get();
        if count != 0 {
            panic!("already immutably borrowed!");
        }
        self.borrow.set(-1);
        MyRefMut { refcell: self }
    }

    // internal unsafe accessors
    fn get_ref(&self) -> &T {
        unsafe { &*self.value.get() }
    }

    fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }
}

// ---------- Immutable reference wrapper ----------
pub struct MyRef<'a, T> {
    refcell: &'a MyRefCell<T>,
}

impl<'a, T> std::ops::Deref for MyRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.refcell.get_ref()
    }
}

impl<'a, T> Drop for MyRef<'a, T> {
    fn drop(&mut self) {
        let count = self.refcell.borrow.get();
        self.refcell.borrow.set(count - 1);
    }
}

// ---------- Mutable reference wrapper ----------
pub struct MyRefMut<'a, T> {
    refcell: &'a MyRefCell<T>,
}

impl<'a, T> std::ops::Deref for MyRefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.refcell.get_mut()
    }
}

impl<'a, T> std::ops::DerefMut for MyRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.refcell.get_mut()
    }
}

impl<'a, T> Drop for MyRefMut<'a, T> {
    fn drop(&mut self) {
        self.refcell.borrow.set(0);
    }
}

// ---------- Demonstration ----------
fn main() {
    let data = MyRefCell::new(100);

    {
        let r1 = data.borrow();
        let r2 = data.borrow();
        println!("Two immutable borrows: {}, {}", *r1, *r2);
    }

    {
        let mut r3 = data.borrow_mut();
        *r3 += 1;
        println!("Mutable borrow: {}", *r3);
    }

   
    // let _r1 = data.borrow();
    // let _r2 = data.borrow_mut(); // panic: already immutably borrowed!
}
