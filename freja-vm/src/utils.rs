use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum Pointer<T> {
    Heap(Rc<T>),
    Stack(T),
    Ref(*const Rc<T>),
}

impl<T> Pointer<T> {
    pub fn into_shared(self) -> Rc<T> {
        match self {
            Pointer::Heap(h) => h,
            Pointer::Stack(s) => Rc::new(s),
            Pointer::Ref(r) => unsafe { (&*r).clone() },
        }
    }
}

impl<T: Default + fmt::Debug> Pointer<T> {
    pub fn promote(&mut self) -> &mut Pointer<T> {
        println!("PROMOTE {:?}", self);
        let this = std::mem::replace(self, Pointer::Stack(T::default()));
        match this {
            Pointer::Heap(h) => {
                *self = Pointer::Heap(h);
            }
            Pointer::Stack(s) => {
                *self = Pointer::Heap(Rc::new(s));
            }
            Pointer::Ref(r) => *self = unsafe { Pointer::Heap((&*r).clone()) },
        }
        self
    }

    pub fn as_ptr(&mut self) -> Pointer<T> {
        match self {
            Pointer::Heap(h) => Pointer::Ref(h as *const Rc<T>),
            Pointer::Stack(s) => {
                self.promote();
                self.as_ptr()
            }
            Pointer::Ref(r) => unsafe { Pointer::Ref(*r) },
        }
    }
}

impl<T: fmt::Display> fmt::Display for Pointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <T as fmt::Display>::fmt(self.as_ref(), f)
    }
}

impl<T> std::ops::Deref for Pointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> AsRef<T> for Pointer<T> {
    fn as_ref(&self) -> &T {
        match self {
            Pointer::Heap(h) => h.as_ref(),
            Pointer::Ref(r) => unsafe { &**r },
            Pointer::Stack(s) => &s,
        }
    }
}

impl<T: Clone> Clone for Pointer<T> {
    fn clone(&self) -> Self {
        match self {
            Pointer::Heap(h) => Pointer::Heap(h.clone()),
            Pointer::Stack(s) => Pointer::Stack(s.clone()),
            Pointer::Ref(r) => unsafe { Pointer::Ref(*r) },
        }
    }
}

impl<T: PartialEq> PartialEq for Pointer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}
