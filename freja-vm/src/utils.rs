use std::fmt;

#[derive(Debug)]
pub enum Pointer<T> {
    Stack(T),
    Ref(*const T),
}

impl<T: Default + fmt::Debug> Pointer<T> {
    pub fn as_ptr(&self) -> Pointer<T> {
        match self {
            Pointer::Stack(s) => Pointer::Ref(s as *const T),
            Pointer::Ref(r) => Pointer::Ref(*r),
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            Pointer::Ref(_) => true,
            _ => false,
        }
    }
}

impl<T: Clone> Pointer<T> {
    pub fn into_inner(self) -> T {
        match self {
            Pointer::Ref(r) => unsafe { (&*r).clone() },
            Pointer::Stack(s) => s,
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
            Pointer::Ref(r) => unsafe { &**r },
            Pointer::Stack(s) => &s,
        }
    }
}

impl<T: Clone> Clone for Pointer<T> {
    fn clone(&self) -> Self {
        match self {
            Pointer::Stack(s) => Pointer::Stack(s.clone()),
            Pointer::Ref(r) => unsafe { Pointer::Stack((&**r).clone()) },
        }
    }
}

impl<T: PartialEq> PartialEq for Pointer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}
