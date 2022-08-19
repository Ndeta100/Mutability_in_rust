use crate::Cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;
struct RcInner<T> {
    value: *const T,
    refcount: Cell<usize>,
}
pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}
impl<T> Rc<T> {
    fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });
        Rc {
            //SAFETY: Box does not give us a null pointer
            inner: NonNull::new_uncheched(Box::into_raw(inner)),
            _marker: PhantomData,
        }
    }
}
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { &*self.inner };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}
impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        //SAFETY:
        //self.inner is a Box that is only deallocated when the last Rc goes away
        //we have an Rc, therefore the box has not been deallocated, so deref is fine
        &unsafe { self.inner.as_ref() }.value
    }
}
impl Drop for Rc<T> {
    fn drop(&self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            //SAFETY: we are the only Rc left, and we are being dropped.
            //Therefore, after us, there will be no Rc's and no references to T.
            drop(inner);
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            //There are other Rc's, so do not drop the Box
            inner.refcount.set(c - 1);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn bad() {
        let (y, _x);
        x = String::from("foo");
        y = Rc::new(&x);
    }
}
