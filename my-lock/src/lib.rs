
use std::cell::RefCell;
use std::fmt;

pub struct Lock<T> {
    locked: RefCell<bool>,
    data: RefCell<T>,
}

impl<T: fmt::Debug> fmt::Debug for Lock<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}


unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    pub fn new(data: T) -> Self { 
        Self {
            data: RefCell::new(data),
            locked: RefCell::new(false),
        }
    }

    pub fn lock(&self, op: impl FnOnce(&mut T)) {
        // if have not get lock , will all spin
        while dbg!(*self.locked.borrow()) != false {} // **1

        // get lock, add lock 
        *self.locked.borrow_mut() = true; // **2

        // call op 
        op(&mut self.data.borrow_mut()); // **3

        // unlock 
        *self.locked.borrow_mut() = false; // **4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    
    #[test]
    fn test_my_lock_is_work() {
        let data = Arc::new(Lock::new(0));
        
        let mut threads = vec![];
        for _index in 0..12 {
            let inner_data = dbg!(data.clone());
            let thread = thread::spawn(move || {
                inner_data.lock(|v| *v += 10);
            });
            threads.push(thread);
        }
        
        for t in threads {
            let _r = t.join();
        }

        println!("data lock : {:?}", dbg!(data));

        // 出现多个线程同时将locked 修改成了true
        //         running 1 test
        // [my-lock/src/lib.rs:54] data.clone() = Lock<0>
        // [my-lock/src/lib.rs:54] data.clone() = Lock<0>
        // [my-lock/src/lib.rs:54] data.clone() = Lock<0>
        // [my-lock/src/lib.rs:54] data.clone() = Lock<0>
        // [my-lock/src/lib.rs:54] data.clone() = Lock<0>
        // [my-lock/src/lib.rs:54] data.clone() = Lock<0>
        // [my-lock/src/lib.rs:29] *self.locked.borrow() = false
        // [thread 'my-lock/src/lib.rs<unnamed>:' panicked at '29already borrowed: BorrowMutError] ', *self.locked.borrow()my-lock/src/lib.rs = :false32
        // :22
        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        // thread '<unnamed>' panicked at 'already borrowed: BorrowMutError', my-lock/src/lib.rs:32:22
        // [my-lock/src/lib.rs:29] *self.locked.borrow() = false
        // thread '<unnamed>' panicked at 'already borrowed: BorrowMutError', my-lock/src/lib.rs:32:22
        // [my-lock/src/lib.rs:54] data.clone() = Lock<0>
        // [my-lock/src/lib.rs:29] *self.locked.borrow() = false
        // thread '<unnamed>' panicked at '[my-lock/src/lib.rs:54] data.clone() = Lock<0>
        // already borrowed: BorrowMutError', my-lock/src/lib.rs:32:22
        // [my-lock/src/lib.rs:54] data.clone() = Lock<0>
        // [my-lock/src/lib.rs:29] *self.locked.borrow() = false
        // thread '<unnamed>' panicked at 'already borrowed: BorrowMutError', my-lock/src/lib.rs:32:22
        // [my-lock/src/lib.rs:29] *self.locked.borrow() = false
        // thread '<unnamed>' panicked at 'already borrowed: BorrowMutError', my-lock/src/lib.rs:32:22
        // [my-lock/src/lib.rs:29] *self.locked.borrow() = false
        // thread '<unnamed>' panicked at 'already borrowed: BorrowMutError', my-lock/src/lib.rs:32:22
        // [my-lock/src/lib.rs:54] data.clone() = Lock<0>
        // [my-lock/src/lib.rs:29] *self.locked.borrow() = false
        // thread '<unnamed>' panicked at 'already borrowed: BorrowMutError', my-lock/src/lib.rs:32:22
        // [my-lock/src/lib.rs:29] *self.locked.borrow() = false
        // [my-lock/src/lib.rs:54] data.clone() = Lock<10>
        // [my-lock/src/lib.rs:29] *self.locked.borrow() = false
        // [my-lock/src/lib.rs:54] data.clone() = Lock<20>
        // [my-lock/src/lib.rs:29] *self.locked.borrow() = false
        // [my-lock/src/lib.rs:29] *self.locked.borrow() = false
        // [my-lock/src/lib.rs:65] data = Lock<40>
        // data lock : Lock<40>
        // test tests::test_my_lock_is_work ... ok
    }
}