#[macro_export]
macro_rules! parse_tokens {
    ($input:expr) => {{
        let input = $input;

        match $crate::tools::syn::parse2(input) {
            Ok(tokens) => Ok(tokens),
            Err(error) => Err($crate::error::MacroError::SynError(error)),
        }
    }};
}

#[macro_export]
macro_rules! tokens {
    ({ $($tokens:tt)* } spanned $span:expr) => {{
        let span = $span;
        $crate::tools::quote::quote_spanned! { span => $($tokens)* }
    }};

    ($($tokens:tt)*) => {{
        $crate::tools::quote::quote! { $($tokens)* }
    }};

}

#[macro_export]
macro_rules! macro_error {
    ($token:tt) => {{
        let error = format!("{}", $token);
        $crate::tools::quote::quote!(::std::compile_error!(#error))
    }};

    (display $token:tt spanned $span:expr) => {{
        let span = $span;
        let error = format!("{}", $token);

        $crate::tools::quote::quote_spanned!(span => ::std::compile_error!(#error))
    }};

    (display($error:expr) spanned $span:expr) => {{
        let span = $span;
        let error = format!("{}", $error);
        $crate::tools::quote::quote_spanned!(span => ::std::compile_error!(#error))
    }};

    (format($($tokens:tt)*) spanned $span:expr) => {{
        let span = $span;
        let error = format!($($tokens)*);
        $crate::tools::quote::quote_spanned!(span => ::std::compile_error!(#error))
    }};

    (format($($tokens:tt)*)) => {{
        let error = format!($($tokens)*);
        $crate::tools::quote::quote!(::std::compile_error!(#error))
    }};
}

#[macro_export]
macro_rules! entry_point_parse {
    ($expr:expr) => {
        match $expr {
            Ok(expr) => expr,
            Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
        }
    };
}
