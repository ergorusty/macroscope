use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    token::Token,
};

pub trait Validate {
    fn validate(stream: &ParseStream) -> bool;
}

impl<T> Validate for T
where
    T: Token,
{
    fn validate(stream: &ParseStream) -> bool {
        T::peek(stream.cursor())
    }
}

pub trait SpeculativeValidate: Parse {
    fn speculative_validate(stream: &ParseStream) -> Option<Self> {
        let fork = stream.fork();

        match Self::parse(&fork) {
            Ok(parsed) => {
                stream.advance_to(&fork);
                Some(parsed)
            }
            Err(_) => None,
        }
    }
}

impl<T> SpeculativeValidate for T where T: Parse {}

#[macro_export]
macro_rules! try_parse {
    ($input:expr => $ty:ty) => {{
        use syn::parse::discouraged::Speculative;

        let fork = $input.fork();

        match <$ty>::parse(&fork) {
            Ok(parsed) => {
                $input.advance_to(&fork);
                Some(parsed)
            }
            Err(_) => None,
        }
    }};
}
