//! Zero-fuss Any-tools.
//!
//! There's only two types exposed here that you're going to be interested in:
//! * [`Ty`], a nicer `TypeId`
//! * [`AnyDebug`], a nicer `Any`

#[cfg(feature = "any_debug")]
#[macro_use]
extern crate mopa;

use std::fmt;
use std::any::TypeId as StdTypeId;
use std::hash;
use std::cmp::Ordering;
#[cfg(feature = "layout")]
use std::alloc::Layout;

/// A nicer `TypeId`.
///
/// 1. Shorter name.
/// 2. Improved [`Debug`](fmt::Debug) impl.
/// 3. Option for non-`'static` types.
/// 4. Knows its [layout](std::alloc::Layout) (if you enable the `layout` feature).
#[derive(Copy, Clone, Eq)]
pub struct Ty {
    id: TypeId,
    // fn() is half the size of a &str
    name: fn() -> &'static str,
    #[cfg(feature = "layout")]
    layout: Layout,
}
impl Ty {
    pub fn name(&self) -> &'static str { (self.name)() }
    pub fn id(&self) -> TypeId { self.id }
    #[cfg(feature = "layout")]
    pub fn layout(&self) -> Layout { self.layout }
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
impl Ty {
    pub fn of<T: 'static>() -> Ty {
        Ty {
            id: TypeId::of::<T>(),
            name: type_name::<T>,
            #[cfg(feature = "layout")]
            layout: Layout::new::<T>(),
        }
    }
    pub fn of_every<T>() -> Ty {
        Ty {
            id: TypeId::of_every::<T>(),
            name: type_name::<T>,
            #[cfg(feature = "layout")]
            layout: Layout::new::<T>(),
        }
    }
}
impl fmt::Debug for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self.name)())
    }
}

/// Either a [`std:TypeId`](std::any::TypeId) or a non-`'static` [`ezty:TypeId`](TypeId).
///
/// Note that `Ty::of::<T>() != Ty::of_every::<T>()`.
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum TypeId {
    Std(StdTypeId),
    NonStatic(NonStaticTypeId),
}
impl TypeId {
    pub fn of<T: 'static>() -> Self {
        Self::Std(StdTypeId::of::<T>())
    }
    pub fn of_every<T>() -> Self {
        Self::NonStatic(NonStaticTypeId::of::<T>())
    }
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
    pub fn of<T>() -> Self {
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
        assert_eq!(a, "Vec<i32>");
    }
}
