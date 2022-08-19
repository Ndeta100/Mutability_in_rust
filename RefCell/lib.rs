use crate::Cell::Cell;
#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}
//Implied by unsafeCell
//impl !Sync for Cell<T>{}
impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }
    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        //SAFETY: No exclusive references have been given out, since they will be exclusive
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(unsafe { &*self.value.get() })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(unsafe { &*self.value.get() })
            }
            RefState::Exclusive => None,
        }
    }
    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            //SAFETY: No exclusive references have been given out, since they will be shared or exclusive

            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}
pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}
pub struct RefMut<'refcell, T> {}
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}
impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
    }
}
impl std::ops::Deref for Ref<'_, T> {
    type Target = T;
    //SAFETY:
    //a Ref is only created if no exclusive reference have been given out.
    //once it is given out. state is set to SHared, so no exclusive references are given out.
    //so dereferencing into a shared reference is fine.
    fn deref(&self) -> &Self::Target {
        Some(unsafe { &*self.refcell.value.get() })
    }
}
impl std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    //SAFETY:
    //a RefMut is only created if no exclusive reference have been given out.
    //once it is given out. state is set to Exclusive, so no exclusive references are given out.
    //so dereferencing into a shared reference is fine.
    fn deref(&self) -> &Self::Target {
        Some(unsafe { &*self.refcell.value.get() })
    }
}
impl std::ops::DerefMut for RefMut<'_, T> {
    type Target = T;
    //SAFETY:
    //a RefMut is only created if no exclusive reference have been given out.
    //once it is given out. state is set to Exclusive, so no exclusive references are given out.
    //so dereferencing into a shared reference is fine.
    fn deref_mut(&mut self) -> &mut Self::Target {
        Some(unsafe { &*self.refcell.value.get() })
    }
}
