use std::str::FromStr;

use bigdecimal::BigDecimal;
use colored::Colorize;
use litrs::{
    ByteLit, ByteStringLit, CharLit, FloatLit, FloatType, IntegerLit, IntegerType, StringLit,
};
use mown::Mown;
use oxidate_macros::{data, key};

use crate::Span;

use super::UnspannedLiteral;

pub trait SuffixTrait {
    fn as_string(&self) -> &str;

    fn span(&self) -> Span {
        proc_macro2::Span::call_site().into()
    }

    fn matches_string(&self, string: &str) -> bool {
        let self_string = self.as_string();
        self_string == string
    }

    fn into_generic(self) -> AnyLiteralSuffix;
    fn to_generic(&self) -> AnyLiteralSuffix;
}

pub trait HighLevelSuffix: SuffixTrait {
    type Special: std::fmt::Debug;

    fn general(general: GeneralSuffix) -> Self;

    fn special_without_span(special: Self::Special) -> Self
    where
        Self: Sized;

    fn as_special(&self) -> Option<&Self::Special>;
}

#[data(copy)]
pub struct IntegerSuffix {
    inner: IntegerType,
    span: Span,
}

impl IntegerSuffix {
    pub fn without_span(integer: IntegerType) -> IntegerSuffix {
        IntegerSuffix {
            inner: integer,
            span: proc_macro2::Span::call_site().into(),
        }
    }
}

impl SuffixTrait for IntegerSuffix {
    fn as_string(&self) -> &str {
        match self.inner {
            IntegerType::U8 => "u8",
            IntegerType::U16 => "u16",
            IntegerType::U32 => "u32",
            IntegerType::U64 => "u64",
            IntegerType::U128 => "u128",
            IntegerType::Usize => "usize",
            IntegerType::I8 => "i8",
            IntegerType::I16 => "i16",
            IntegerType::I32 => "i32",
            IntegerType::I64 => "i64",
            IntegerType::I128 => "i128",
            IntegerType::Isize => "isize",
        }
    }

    fn span(&self) -> Span {
        self.span
    }

    fn into_generic(self) -> AnyLiteralSuffix {
        AnyLiteralSuffix::Numeric(NumericSuffix::Integer(self))
    }

    fn to_generic(&self) -> AnyLiteralSuffix {
        AnyLiteralSuffix::Numeric(NumericSuffix::Integer(*self))
    }
}

#[data(copy)]
pub struct FloatSuffix {
    inner: FloatType,
    span: Span,
}

impl SuffixTrait for FloatSuffix {
    fn as_string(&self) -> &str {
        match self.inner {
            FloatType::F32 => "f32",
            FloatType::F64 => "f64",
        }
    }

    fn span(&self) -> Span {
        self.span
    }

    fn into_generic(self) -> AnyLiteralSuffix {
        AnyLiteralSuffix::Numeric(NumericSuffix::Float(self))
    }

    fn to_generic(&self) -> AnyLiteralSuffix {
        AnyLiteralSuffix::Numeric(NumericSuffix::Float(*self))
    }
}

impl FloatSuffix {
    pub fn without_span(integer: FloatType) -> FloatSuffix {
        FloatSuffix {
            inner: integer,
            span: proc_macro2::Span::call_site().into(),
        }
    }
}

#[key]
pub struct NoSuffix;

#[data]
pub struct IdentifierSuffix {
    inner: String,
    span: Span,
}

impl IdentifierSuffix {
    pub fn without_span(identifier: impl Into<String>) -> IdentifierSuffix {
        IdentifierSuffix {
            inner: identifier.into(),
            span: proc_macro2::Span::call_site().into(),
        }
    }
}

impl SuffixTrait for IdentifierSuffix {
    fn as_string(&self) -> &str {
        &self.inner
    }

    fn span(&self) -> Span {
        self.span
    }

    fn into_generic(self) -> AnyLiteralSuffix {
        AnyLiteralSuffix::General(GeneralSuffix::Identifier(self))
    }

    fn to_generic(&self) -> AnyLiteralSuffix {
        AnyLiteralSuffix::General(GeneralSuffix::Identifier(self.clone()))
    }
}

/// The most general suffix that is legal after all literals.
#[data]
pub enum GeneralSuffix {
    #[allow(unused)]
    Identifier(IdentifierSuffix),
    None,
}

impl SuffixTrait for GeneralSuffix {
    fn as_string(&self) -> &str {
        match self {
            GeneralSuffix::Identifier(id) => id.as_string(),
            GeneralSuffix::None => "",
        }
    }

    fn into_generic(self) -> AnyLiteralSuffix {
        AnyLiteralSuffix::General(self)
    }

    fn to_generic(&self) -> AnyLiteralSuffix {
        match self {
            GeneralSuffix::Identifier(id) => GeneralSuffix::Identifier(id.clone()).into_generic(),
            GeneralSuffix::None => AnyLiteralSuffix::General(GeneralSuffix::None),
        }
    }
}

impl GeneralSuffix {
    #[allow(unused)]
    pub fn without_span(string: impl Into<String>) -> GeneralSuffix {
        GeneralSuffix::Identifier(IdentifierSuffix::without_span(string))
    }
}

#[data(copy)]
pub enum NumericSuffix {
    #[allow(unused)]
    Integer(IntegerSuffix),
    #[allow(unused)]
    Float(FloatSuffix),
}

impl SuffixTrait for NumericSuffix {
    fn as_string(&self) -> &str {
        match self {
            NumericSuffix::Integer(integer) => integer.as_string(),
            NumericSuffix::Float(float) => float.as_string(),
        }
    }

    fn into_generic(self) -> AnyLiteralSuffix {
        AnyLiteralSuffix::Numeric(self)
    }

    fn to_generic(&self) -> AnyLiteralSuffix {
        AnyLiteralSuffix::Numeric(*self)
    }
}

#[data]
pub enum AnyLiteralSuffix {
    Numeric(NumericSuffix),
    General(GeneralSuffix),
}

impl std::fmt::Display for AnyLiteralSuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyLiteralSuffix::Numeric(n) => write!(f, "{}", n.as_string()),
            AnyLiteralSuffix::General(g) => write!(f, "{}", g.as_string()),
        }
    }
}

impl SuffixTrait for AnyLiteralSuffix {
    fn as_string(&self) -> &str {
        match self {
            AnyLiteralSuffix::Numeric(numeric) => numeric.as_string(),
            AnyLiteralSuffix::General(general) => general.as_string(),
        }
    }

    fn into_generic(self) -> AnyLiteralSuffix {
        self
    }

    fn to_generic(&self) -> AnyLiteralSuffix {
        self.clone()
    }
}

impl HighLevelSuffix for AnyLiteralSuffix {
    type Special = NumericSuffix;

    fn general(general: GeneralSuffix) -> Self
    where
        Self: Sized,
    {
        AnyLiteralSuffix::General(general)
    }

    fn special_without_span(special: Self::Special) -> Self
    where
        Self: Sized,
    {
        AnyLiteralSuffix::Numeric(special)
    }

    fn as_special(&self) -> Option<&Self::Special> {
        match self {
            AnyLiteralSuffix::Numeric(numeric) => Some(numeric),
            AnyLiteralSuffix::General(_) => None,
        }
    }
}

#[data]
pub enum AnyIntegerSuffix {
    Integer(IntegerSuffix),
    General(GeneralSuffix),
}

impl AnyIntegerSuffix {
    pub fn special_without_span(integer: IntegerType) -> AnyIntegerSuffix {
        AnyIntegerSuffix::Integer(IntegerSuffix::without_span(integer))
    }
}

impl SuffixTrait for AnyIntegerSuffix {
    fn as_string(&self) -> &str {
        match self {
            AnyIntegerSuffix::Integer(integer) => integer.as_string(),
            AnyIntegerSuffix::General(general) => general.as_string(),
        }
    }

    fn into_generic(self) -> AnyLiteralSuffix {
        match self {
            AnyIntegerSuffix::Integer(integer) => {
                AnyLiteralSuffix::Numeric(NumericSuffix::Integer(integer))
            }
            AnyIntegerSuffix::General(general) => AnyLiteralSuffix::General(general),
        }
    }

    fn to_generic(&self) -> AnyLiteralSuffix {
        self.clone().into_generic()
    }
}

impl HighLevelSuffix for AnyIntegerSuffix {
    type Special = IntegerType;

    fn general(general: GeneralSuffix) -> Self
    where
        Self: Sized,
    {
        AnyIntegerSuffix::General(general)
    }

    fn special_without_span(special: Self::Special) -> Self
    where
        Self: Sized,
    {
        AnyIntegerSuffix::Integer(IntegerSuffix::without_span(special))
    }

    fn as_special(&self) -> Option<&Self::Special> {
        match self {
            AnyIntegerSuffix::Integer(integer) => Some(&integer.inner),
            AnyIntegerSuffix::General(_) => None,
        }
    }
}

#[data]
pub enum AnyFloatSuffix {
    Float(FloatSuffix),
    General(GeneralSuffix),
}

impl SuffixTrait for AnyFloatSuffix {
    fn as_string(&self) -> &str {
        match self {
            AnyFloatSuffix::Float(float) => float.as_string(),
            AnyFloatSuffix::General(general) => general.as_string(),
        }
    }

    fn into_generic(self) -> AnyLiteralSuffix {
        match self {
            AnyFloatSuffix::Float(float) => AnyLiteralSuffix::Numeric(NumericSuffix::Float(float)),
            AnyFloatSuffix::General(general) => AnyLiteralSuffix::General(general),
        }
    }

    fn to_generic(&self) -> AnyLiteralSuffix {
        self.clone().into_generic()
    }
}

impl HighLevelSuffix for AnyFloatSuffix {
    type Special = FloatType;

    fn general(general: GeneralSuffix) -> Self
    where
        Self: Sized,
    {
        AnyFloatSuffix::General(general)
    }

    fn special_without_span(special: Self::Special) -> Self
    where
        Self: Sized,
    {
        AnyFloatSuffix::Float(FloatSuffix::without_span(special))
    }

    fn as_special(&self) -> Option<&Self::Special> {
        match self {
            AnyFloatSuffix::Float(float) => Some(&float.inner),
            AnyFloatSuffix::General(_) => None,
        }
    }
}

pub trait BaseType: std::fmt::Debug + Eq {}

impl<T> BaseType for T where T: std::fmt::Debug + Eq + ?Sized {}

pub trait LiteralTrait: BaseType + std::fmt::Display {
    type Output: ?Sized + mown::ToOwned + BaseType;
    type Suffix: std::fmt::Debug + SuffixTrait;

    fn value(&self) -> Mown<Self::Output>;
    fn as_str(&self) -> Mown<str>;

    fn check(literal: &UnspannedLiteral) -> Option<&Self>
    where
        Self: Sized;

    fn matches_value(literal: &UnspannedLiteral, output: &Self::Output) -> bool
    where
        Self: Sized,
    {
        match Self::check(literal) {
            None => false,
            Some(literal) => literal.value() == Mown::Borrowed(output),
        }
    }

    fn matches_suffix(&self, rhs: &str) -> bool {
        // TODO: Implement AsMown a la AsRef
        self.suffix().matches_string(rhs)
    }

    fn matches(&self, rhs: &UnspannedLiteral) -> bool
    where
        Self: Sized,
    {
        if let Some(this) = Self::check(rhs) {
            this == self
        } else {
            false
        }
    }

    fn suffix(&self) -> Mown<Self::Suffix>;

    fn generic_suffix(&self) -> AnyLiteralSuffix {
        self.suffix().to_generic()
    }
}

macro_rules! literal {

    // (@impl Literal $kind:ident -> $value_ty:ty) => {
    //     impl Literal for $kind {
    //         type Output = $value_ty;

    //         fn value(&self) -> $value_ty {
    //             self.value()
    //         }
    //     }
    // };

    (UnspannedLiteral::$variant:ident($kind:ident($wrapping:ident)) { Suffix = $suffix_enum:ident :: $suffix_variant:ident; $($tokens:tt)* }) => {
        literal!(@impl LiteralTrait { impl = { UnspannedLiteral::$variant($kind($wrapping)) { $($tokens)* } } suffix($suffix_enum) = |l| {
            match l.inner.type_suffix() {
                Some(suffix) => Mown::Owned($suffix_enum::special_without_span(suffix)),
                None => Mown::Owned($suffix_enum::general(GeneralSuffix::None))
            }
        } });
    };

    (UnspannedLiteral::$variant:ident($kind:ident($wrapping:ident)) { $($tokens:tt)* }) => {
        literal!(@impl LiteralTrait {
            impl = {
                UnspannedLiteral::$variant($kind($wrapping)) {
                    $($tokens)*
                }
            }
            suffix(GeneralSuffix) = |_| { Mown::Owned(GeneralSuffix::None) }
        });
    };


    (@impl LiteralTrait {
        impl = {
            UnspannedLiteral::$variant:ident($kind:ident($wrapping:ident)) {
                value($literal:tt) -> &$value_ty:ty { $($value_expr:tt)* }
            }
        }
        suffix($suffix_ty:ident) = |$suffix_ident:tt| { $($suffix_expr:tt)* }
    }) => {
        literal!(@impl Concrete $kind($wrapping) { value($literal) -> &$value_ty { $($value_expr)* } });

        impl LiteralTrait for $kind {
            type Output = $value_ty;
            type Suffix = $suffix_ty;

            fn value(&self) -> Mown<$value_ty> {
                Mown::Borrowed(self.to_value())
            }

            fn as_str(&self) -> Mown<str> {
                Mown::Owned(self.inner.to_string())
            }

            fn check(rhs: &UnspannedLiteral) -> Option<&Self>
            where
                Self: Sized,
            {
                match rhs {
                    UnspannedLiteral::$variant(rhs) => Some(rhs),
                    _ => None
                }
            }

            fn suffix(&self) -> Mown<Self::Suffix> {
                let $suffix_ident = self;
                Mown::from($($suffix_expr)*)
            }
        }
    };

    (@impl LiteralTrait {
        impl = {
            UnspannedLiteral::$variant:ident($kind:ident($wrapping:ident)) { value($literal:tt) -> $value_ty:ty { $($value_expr:tt)* } }
        }
        suffix($suffix_ty:ident) = |$suffix_ident:tt| { $($suffix_expr:tt)* }
    }) => {
        literal!(@impl Concrete $kind($wrapping) { value($literal) -> $value_ty { $($value_expr)* } });

        impl LiteralTrait for $kind {
            type Output = $value_ty;
            type Suffix = $suffix_ty;

            fn value(&self) -> Mown<$value_ty> {
                Mown::Owned(self.to_value())
            }

            fn as_str(&self) -> Mown<str> {
                Mown::Owned(self.inner.to_string())
            }

            fn check(rhs: &UnspannedLiteral) -> Option<&Self>
            where
                Self: Sized
            {
                match rhs {
                    UnspannedLiteral::$variant(rhs) => Some(rhs),
                    _ => None
                }
            }

            fn suffix(&self) -> Mown<Self::Suffix> {
                let $suffix_ident = self;
                $($suffix_expr)*
            }
        }

    };


    (@impl Concrete $kind:ident($wrapping:ident) { value($literal:tt) -> $value_ty:ty { $($value_expr:tt)* } }) => {
        #[data]
        #[derive(Eq, PartialEq)]
        pub struct $kind
        {
            inner: $wrapping<String>,
        }


        impl $kind {
            pub fn parse(source: impl Into<String>) -> Result<Self, litrs::ParseError> {
                let inner = $wrapping::parse(source.into())?;
                Ok(Self { inner })
            }

        }

        impl $kind  {
            pub fn to_value(&self) -> $value_ty {
                let $literal = &self.inner;
                $($value_expr)*
            }
        }

        impl From<$wrapping<String>> for $kind {
            fn from(litrs: $wrapping<String>) -> $kind {
                $kind { inner: litrs }
            }
        }

        impl Into<$wrapping<String>> for $kind {
            fn into(self) -> $wrapping<String> {
                self.inner
            }
        }

        impl Into<litrs::Literal<String>> for $kind
        {
            fn into(self) -> litrs::Literal<String> {
                self.inner.into()
            }
        }

        impl std::fmt::Display for $kind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.inner)
            }
        }
    };
}

literal!(UnspannedLiteral::String(StringLiteral(StringLit)) {
    value(string_lit) -> &str { string_lit.value() }
});

literal!(UnspannedLiteral::ByteString(ByteStringLiteral(ByteStringLit)) {
    value(byte_string_lit) -> &[u8] { byte_string_lit.value() }
});

literal!(UnspannedLiteral::Byte(ByteLiteral(ByteLit)) {
    value(byte_lit) -> u8 { byte_lit.value() }
});

literal!(UnspannedLiteral::Char(CharLiteral(CharLit)) {
    value(char_lit) -> char {
        char_lit.value()
    }
});

literal!(UnspannedLiteral::Integer(IntegerLiteral(IntegerLit)) {
    Suffix = AnyIntegerSuffix::Integer;
    value(integer_lit) -> i128 {
        integer_lit.value::<i128>().expect("UNEXPECTED: litrs::IntegerLit's value method should always support i128")
    }
});

literal!(UnspannedLiteral::Float(FloatLiteral(FloatLit)) {
    Suffix = AnyFloatSuffix::Float;
    value(float_lit) -> BigDecimal {
        let string = float_lit.number_part().replace("_", "-");
        BigDecimal::from_str(&string).unwrap_or_else(|err|
            panic!(
                "\
                    Parse Error\n\n{}\n\n\
                    {} {} {}\n\n\
                    {}{} {}\n\n{}\
                ",
                "litrs::FloatLit::number_part should be parseable by BigDecimal, after `_`s are removed.".white().dimmed(),
                " ATTEMPTED ".yellow().reverse(),
                format!("to parse this output of `number_part`:").yellow(),
                format!("{:?}", string).white().dimmed().reverse(),
                "    ",
                " ERROR ".on_red(), err.to_string().red(),
                "If you're seeing this, please file a bug (TODO: URL)".green())
            )
    }
});
