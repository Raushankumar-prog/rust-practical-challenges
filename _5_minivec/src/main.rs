use std::alloc::{self, Layout};
use std::ptr;

struct MyVec<T> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec {
            ptr: std::ptr::null_mut(),
            capacity: 0,
            len: 0,
        }
    }

    fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            self.ptr.add(self.len).write(value);
        }

        self.len += 1;
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
        let new_layout = Layout::array::<T>(new_capacity).unwrap();

        unsafe {
            let new_ptr = if self.capacity == 0 {
                alloc::alloc(new_layout) as *mut T
            } else {
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::realloc(self.ptr as *mut u8, old_layout, new_layout.size()) as *mut T
            };
            
            if new_ptr.is_null() {
                std::alloc::handle_alloc_error(new_layout);
            }

            self.ptr = new_ptr;
            self.capacity = new_capacity;
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            unsafe { Some(&mut *self.ptr.add(index)) }
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        unsafe { Some(ptr::read(self.ptr.add(self.len))) }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            // Drop each element manually
            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.add(i));
            }

            // Free the memory if allocated
            if self.capacity != 0 {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

fn main() {
    let mut v = MyVec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    println!("v[1] = {:?}", v.get(1));

    println!("popped = {:?}", v.pop());
    println!("popped = {:?}", v.pop());
    println!("popped = {:?}", v.pop());
}
