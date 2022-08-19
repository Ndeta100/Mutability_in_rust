use std::cell::UnsafeCell;
pub struct Cell<T> {
    value: UnsafeCell<T>,
}
unsafe impl<T> Sync for Cell<T> {}
//Implied by unsafeCell
//impl !Sync for Cell<T>{}
impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }
    pub fn set(&self, value: T) {
        //SAFETY: We know no-one else is concurrently mutating self.value (because !Sync)
        //SAFETY: We know we're no invalidating any refernces, because we never give any out
        unsafe { *self.value.get() = value }
    }
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        //SAFETY: We know no-one else is modifying this value, since only this thread can mutate
        //(Because !Sync), and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use std::{sync::Arc, thread};

    use super::*;
    #[test]
    fn bad() {
        let x = Arc::new(Cell::new([0; 40240]));
        let x1 = Arc::clone(&x);
        let jh1 = thread::spawn(move || {
            x1.set([1; 40240]);
        });

        let x2 = Arc::clone(&x);
        let jh2 = thread::spawn(move || {
            x2.set([2; 40240]);
        });
        jh1.join().unwrap();
        jh2.join().unwrap();
        for &i in x.get().iter() {
            eprint!("{}", i);
        }
    }
    #[test]
    fn bad2() {
        let x = Cell::new(vec![42]);
        let first = &x.get()[0];
        x.set(vec![]);
        eprint!("{}", first);
    }
}
