//! Zero-fuss Any-tools.
//!
//! The most interesting things this crate provides are:
//! * [`Ty`], a nicer [`std:TypeId`](`StdTypeId`)
//! * [`AnyDebug`], a nicer [`Any`](`std::any::Any`)

#[cfg(feature = "any_debug")]
#[macro_use]
extern crate mopa;

use std::fmt;
use std::any::TypeId as StdTypeId;
use std::hash;
use std::cmp::Ordering;
use std::alloc::Layout;

/// A nicer [`std:TypeId`](`StdTypeId`).
///
/// 1. Shorter name.
/// 2. Improved [`Debug`](fmt::Debug) impl.
/// 3. Option for non-`'static` types.
#[derive(Copy, Clone, Eq)]
pub struct Ty {
    id: TypeId,
    // fn() is half the size of a &str
    name: fn() -> &'static str,
}
impl Ty {
    pub fn of<T: ?Sized + 'static>() -> Ty {
        Ty {
            id: TypeId::of::<T>(),
            name: type_name::<T>,
        }
    }
    pub fn of_every<T: ?Sized>() -> Ty {
        Ty {
            id: TypeId::of_every::<T>(),
            name: type_name::<T>,
        }
    }
}
impl Ty {
    pub fn name(&self) -> &'static str { (self.name)() }
    pub fn id(&self) -> TypeId { self.id }
}
impl hash::Hash for Ty {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        hash::Hash::hash(&self.id, state)
    }
}
impl PartialEq for Ty {
    fn eq(&self, other: &Ty) -> bool {
        self.id == other.id
    }
}
impl Ord for Ty {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Ty {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
impl fmt::Debug for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self.name)())
    }
}

/// Either a [`std:TypeId`](std::any::TypeId) or a non-`'static` [`ezty:NonStaticTypeId`](NonStaticTypeId).
///
/// Note that `Ty::of::<T>() != Ty::of_every::<T>()`.
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum TypeId {
    Std(StdTypeId),
    NonStatic(NonStaticTypeId),
}
impl TypeId {
    pub fn of<T: ?Sized + 'static>() -> Self {
        Self::Std(StdTypeId::of::<T>())
    }
    pub fn of_every<T: ?Sized>() -> Self {
        Self::NonStatic(NonStaticTypeId::of::<T>())
    }
}
impl TypeId {
    pub fn std(&self) -> Option<StdTypeId> {
        if let &Self::Std(t) = self {
            Some(t)
        } else {
            None
        }
    }
    pub fn non_static(&self) -> Option<NonStaticTypeId> {
        if let &Self::NonStatic(t) = self {
            Some(t)
        } else {
            None
        }
    }
}

/// A [`TypeId`](StdTypeId) for non-`'static` types.
///
/// (Disclaimer: satisfaction not guaranteed.)
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct NonStaticTypeId(usize);
impl NonStaticTypeId {
    pub fn of<T: ?Sized>() -> Self {
        Self(Self::of::<T> as usize)
    }
}


/// Returns the prettified name of a type, eg `"Vec<T>"` rather than `"alloc::vec::Vec<T>"`.
pub fn type_name<T: ?Sized>() -> &'static str {
    pretty(std::any::type_name::<T>())
}

mod pretty_impl;
pub use self::pretty_impl::pretty;



#[cfg(feature = "any_debug")]
mod any_debug;
#[cfg(not(feature = "any_debug"))]
mod any_debug {
    /// The `any_debug` feature must be enabled to use this trait.
    pub enum AnyDebug {}
}
pub use self::any_debug::AnyDebug;

/// Just like [`Ty`] but it also includes [`Layout`] information.
#[derive(Clone, Eq, PartialEq)]
pub struct LTy {
    ty: Ty,
    layout: Layout,
}
impl fmt::Debug for LTy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self.ty.name)())
    }
}
impl LTy {
    pub fn of<T: 'static>() -> LTy {
        LTy {
            ty: Ty::of::<T>(),
            layout: Layout::new::<T>(),
        }
    }
    pub fn of_every<T>() -> LTy {
        LTy {
            ty: Ty::of_every::<T>(),
            layout: Layout::new::<T>(),
        }
    }
}
impl LTy {
    pub fn ty(&self) -> Ty { self.ty }
    pub fn layout(&self) -> Layout { self.layout }
    pub fn name(&self) -> &'static str { (self.ty.name)() }
    pub fn id(&self) -> TypeId { self.ty.id() }
}

#[cfg(test)]
mod tests {
    use super::Ty;

    #[test]
    fn basics() {
        struct A;
        struct B;
        struct C;
        let a = Ty::of::<A>();
        let b = Ty::of::<B>();
        let c = Ty::of::<C>();
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
        dbg!(a);
    }

    #[test]
    fn pretty() {
        let a = Ty::of::<Vec<i32>>();
        let a = format!("{:?}", a);
        println!("{}", a);
        assert_eq!(a, "Vec<i32>");

        let a = Ty::of::<Vec<Ty>>();
        let a = format!("{:?}", a);
        println!("{}", a);
    }

    #[test]
    fn less_pretty() {
        // FIXME: pretty should return a String.
        let a = Ty::of::<Vec<Vec<u8>>>();
        let a = format!("{:?}", a);
        println!("{}", a);
        assert_eq!(a, "Vec<Vec<u8>>");
    }
}
