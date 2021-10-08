use std::borrow::Cow;

use mown::Mown;
use regex::Regex;

pub trait StringPattern {
    fn matches(&self, actual: &str) -> bool;
}

macro_rules! string {
    ($ty:ty) => {
        impl StringPattern for $ty {
            fn matches(&self, actual: &str) -> bool {
                let str: &str = self.as_ref();
                str == actual
            }
        }

        impl StringPattern for &$ty {
            fn matches(&self, actual: &str) -> bool {
                let str: &str = self.as_ref();
                str == actual
            }
        }

        impl StringPattern for &mut $ty {
            fn matches(&self, actual: &str) -> bool {
                let str: &str = self.as_ref();
                str == actual
            }
        }
    };
}

#[macro_export]
macro_rules! impl_for_str_patterns {
    ($trait_ty:ty {
        $($impl:tt)*
    }) => {
        impl $trait_ty for str {
            $($impl)*
        }

        impl $trait_ty for String {
            $($impl)*
        }

        impl $trait_ty for Cow<'_, str> {
            $($impl)*
        }

        impl $trait_ty for Mown<'_, str> {
            $($impl)*
        }
    };
}

string!(str);
string!(String);
string!(Cow<'_, str>);
string!(Mown<'_, str>);

impl StringPattern for Regex {
    fn matches(&self, actual: &str) -> bool {
        self.is_match(actual)
    }
}
