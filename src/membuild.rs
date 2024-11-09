use std::cell::RefCell;
use std::rc::Rc;

pub fn build_ref<T>(var: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(var))
}

pub fn build_ref_mut<T>(var: T) -> RefCell<T> {
    RefCell::new(var)
}