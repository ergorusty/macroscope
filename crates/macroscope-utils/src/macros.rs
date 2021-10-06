#[macro_export]
macro_rules! try_parse {
    (parse $input:expr => $output:ty) => {{
        match $crate::tools::syn::parse2::<$output>($input) {
            Ok(v) => v,
            Err(err) => return TokenStream::from(err.to_compile_error()),
        }
    }};

    (parse $input:expr) => {{
        match $crate::tools::syn::parse2($input) {
            Ok(v) => v,
            Err(err) => return TokenStream::from(err.to_compile_error()),
        }
    }};

    (darling $input:expr) => {{
        match $input {
            Ok(v) => v,
            Err(err) => return TokenStream::from(err.write_errors()),
        }
    }};

    ($input:expr) => {{
        match $input {
            Ok(v) => v,
            Err(err) => return TokenStream::from(err.to_compile_error()),
        }
    }};
}

#[macro_export]
macro_rules! expr {
    ($($tokens:tt)*) => {
        $crate::tools::proc_macro2::TokenStream::from($crate::tools::quote::quote! {{ $($tokens)* }})
    }
}

#[macro_export]
macro_rules! ty {
    ($($tokens:tt)*) => {
        $crate::tools::proc_macro2::TokenStream::from($crate::tools::quote::quote![$($tokens)*])
    }
}

#[macro_export]
macro_rules! snippet {
    ($($tokens:tt)*) => {
        $crate::tools::proc_macro2::TokenStream::from($crate::tools::quote::quote![$($tokens)*])
    }
}
