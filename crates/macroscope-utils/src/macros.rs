#[macro_export]
macro_rules! try_parse {
    ($input:expr => $output:ty) => {{
        match $crate::hidden::syn::parse2::<$output>($input) {
            Ok(v) => v,
            Err(err) => return TokenStream::from(err.to_compile_error()),
        }
    }};

    ($input:expr) => {{
        match $crate::hidden::syn::parse2($input) {
            Ok(v) => v,
            Err(err) => return TokenStream::from(err.to_compile_error()),
        }
    }};
}

#[macro_export]
macro_rules! expr {
    ($($tokens:tt)*) => {
        $crate::hidden::proc_macro2::TokenStream::from($crate::hidden::quote::quote! {{ $($tokens)* }})
    }
}

#[macro_export]
macro_rules! ty {
    ($($tokens:tt)*) => {
        $crate::hidden::proc_macro2::TokenStream::from($crate::hidden::quote::quote![$($tokens)*])
    }
}

#[macro_export]
macro_rules! snippet {
    ($($tokens:tt)*) => {
        $crate::hidden::proc_macro2::TokenStream::from($crate::hidden::quote::quote![$($tokens)*])
    }
}
