use std::fmt;
use super::{type_name, Ty};

/// Like [`Any`](std::any::Any), but with [`Debug`], [`Send`], & [`Sync`].
pub trait AnyDebug: mopa::Any + fmt::Debug + Send + Sync {
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
    /// You're inevitably wanting to call this with something like `Box<dyn AnyDebug>`, but instead
    /// of returning the ty of the contents, it uselessly returns `Ty::of::<Box<dyn AnyDebug>>()`.
    /// How do you deal with this? Instead of `this.get_ty()`, call `(*this).get_ty()`.
    fn get_ty(&self) -> Ty;
}
mopafy!(AnyDebug);
impl<X: mopa::Any + fmt::Debug + Send + Sync> AnyDebug for X {
    fn get_ty(&self) -> Ty {
        Ty::of::<Self>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let f: &i32 = &0;
        let f: &dyn AnyDebug = f;
        let f = format!("{:?}", f);
        assert_eq!(f, "0");
    }

    #[test]
    fn get_ty_is_hard() {
        let ty = Ty::of::<i32>();
        assert_eq!(format!("{ty:?}"), "i32");
        let f: &i32 = &0;
        let f: &dyn AnyDebug = f;
        assert_eq!(ty, f.get_ty());
    }

    #[test]
    fn get_ty_is_still_hard() {
        let ty = Ty::of::<i32>();
        assert_eq!(format!("{ty:?}"), "i32");
        let f: Box<i32> = Box::new(0);
        let f: Box<dyn AnyDebug> = f;
        assert_eq!(ty, <dyn AnyDebug>::get_ty(&*f));
        assert_eq!(ty, (*f).get_ty());
    }

    #[test]
    fn what_about_std_just_outta_curiosity() {
        use std::any::{TypeId, Any};
        let f: Box<i32> = Box::new(0);
        let f: Box<dyn Any> = f;
        let ty = TypeId::of::<i32>();
        assert_eq!(ty, (*f).type_id());
    }
}
