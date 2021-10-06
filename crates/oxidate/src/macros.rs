#[macro_export]
macro_rules! tokens {
    ({ $($tokens:tt)* } spanned $span:expr) => {{
        let span = $span.into();
        ($crate::tools::quote::quote_spanned! { span => $($tokens)* }).into()
    }};

    ($($tokens:tt)*) => {{
        $crate::tools::quote::quote! { $($tokens)* }
    }};
}

#[macro_export]
macro_rules! macro_error {
    ($token:tt) => {{
        let error = format!("{}", $token);
        ($crate::tools::quote::quote!(::std::compile_error!(#error))).into()
    }};

    (display $token:tt spanned $span:expr) => {{
        let span = $span.into();
        let error = format!("{}", $token);

        ($crate::tools::quote::quote_spanned!(span => ::std::compile_error!(#error))).into()
    }};

    (display($error:expr) spanned $span:expr) => {{
        let span = $span.into();
        let error = format!("{}", $error);
        ($crate::tools::quote::quote_spanned!(span => ::std::compile_error!(#error))).into()
    }};

    (format($($tokens:tt)*) spanned $span:expr) => {{
        let span = $span.into();
        let error = format!($($tokens)*);
        ($crate::tools::quote::quote_spanned!(span => ::std::compile_error!(#error))).into()
    }};

    (format($($tokens:tt)*)) => {{
        let error = format!($($tokens)*);
        $crate::tools::quote::quote!(::std::compile_error!(#error)).into()
    }};
}
