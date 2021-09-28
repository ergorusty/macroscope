pub trait TransparentWrapper {
    type Inner;
    fn get_inner(&self) -> &Self::Inner;
    fn into_inner(self) -> Self::Inner;
}

#[macro_export]
macro_rules! transparent_wrapper {
    ($ident:ident $(< $($ty:ident),* $(,)? >)? $(where { $($where_clause:tt)* })? => self.$path:ident as $path_ty:ty) => {
        impl $(< $($ty,)* >)? $crate::wrapper::TransparentWrapper for $ident $(< $($ty,)* >)? $(where $($where_clause)*)? {
            type Inner = $path_ty;

            fn into_inner(self) -> $path_ty {
                self.$path
            }

            fn get_inner(&self) -> &$path_ty {
                &self.$path
            }
        }

        impl $(< $($ty,)* >)? std::ops::Deref for $ident $(< $($ty,)* >)? $(where $($where_clause)*)? {
            type Target = $path_ty;

            fn deref(&self) -> &$path_ty {
                &self.$path
            }
        }
    };
}

pub trait HasParts<'a> {
    type IntoParts;
    type AsParts: 'a;

    fn into_parts(self) -> Self::IntoParts;
    fn as_parts(&'a self) -> Self::AsParts;
}
