#![allow(clippy::float_cmp, clippy::non_ascii_literal, non_snake_case)]

use crate::{AstParse, AstSpan, Lexer, Literal as OxidateLiteral, Literal, Spanned, ToTokenTree};
use colored::Colorize;
use pad::PadStr;
use proc_macro2::{TokenStream, TokenTree};
use std::{
    any::Any,
    borrow::Borrow,
    error::Error,
    fmt::{self, Formatter},
    str::FromStr,
};

use super::literal_types::{
    ByteLiteral, ByteStringLiteral, CharLiteral, FloatLiteral, IntegerLiteral, LiteralTrait,
    StringLiteral,
};

fn align(left: &str, right: &str) -> (String, String) {
    let max = left.len().max(right.len());

    (
        left.pad_to_width_with_alignment(max + 2, pad::Alignment::Right),
        right.pad_to_width_with_alignment(max + 2, pad::Alignment::Right),
    )
}

macro_rules! assert_matches {

    ($action_desc:ident $($rest:tt)*) => {{
        assert_matches!({ stringify!($action_desc) } $($rest)*);
    }};

    (
        $action_desc:tt { $input:expr } $((style: $input_style:tt))? => $output:expr;

        expected $((style: $pattern_style:tt))? = $pattern:expr;
        $result_desc:ident $((style: $result_desc_style:tt))? = $actual:expr;

        assert { $assertion:expr } or $problem:expr;
    ) => {{
        let (pattern_desc, result_desc) = align("expected", stringify!($result_desc));

        assert!(
            $assertion,
            concat!($problem, ":\n\n{} {}\n{}{}\n\n{} => {}\n\n   {} {}\n   {} {}\n\n"),
            "while".white().dimmed(),
            $action_desc.bright_blue(),
            "-".repeat("while ".len()).white().dimmed(),
            "-".repeat($action_desc.len()).bright_blue(),
            format!(assert_matches!(@style $($input_style)?), $input).black().on_white().bold(),
            format!("{}", $output).on_cyan(),
            result_desc.cyan(),
            format!(assert_matches!(@style $($result_desc_style)?), $actual).on_cyan(),
            pattern_desc.magenta(),
            format!(assert_matches!(@style $($pattern_style)?), $pattern).on_magenta()
        );
    }};

    (@style) => {
        "{}"
    };

    (@style display) => {
        "{}"
    };

    (@style debug) => {
        "{:?}"
    };

    (@style $pattern:tt) => {
        $pattern
    }
}

macro_rules! test_literal {
    ($source:tt => $literal_ty:ident($expected:expr) $suffix:tt) => {{
        fn test_literal<$literal_ty: LiteralTrait + AstParse>(
            source: &str,
            expected: &$literal_ty::Output,
        ) -> TestResult {
            let actual_literal = lit(source);
            let expected_suffix = $suffix;

            assert_matches! {
                parsing { source } (style: debug) => stringify!($literal_ty);

                expected(style: debug) = expected;
                parsed(style: display) = actual_literal.unspanned();

                assert { $literal_ty::matches_value(&actual_literal.unspanned(), expected) } or "Literal parse failure";
            };

            assert_matches! {
                "parsing suffix" { source } (style: debug) => stringify!($literal_ty);

                expected(style: debug) = expected_suffix;
                parsed(style: debug) = actual_literal.unspanned().suffix().to_string();

                assert { actual_literal.matches_suffix($suffix) } or "Literal parse failure";
            }

            let source_round_trip = actual_literal.to_tt().to_string();

            if source_round_trip != source {
                test_literal::<$literal_ty>(&source_round_trip, expected)?;
            }

            let mut lexer = Lexer::source($source)?;

            assert_matches! {
                "peeking" { source } (style: debug) => stringify!(Literal);

                expected(style: debug) = expected;
                parsed(style: display) = { lexer.lookahead() };

                assert { lexer.lookahead_is::<Literal>() } or "Failed to peek as a literal";
            };

            assert_matches! {
                "peeking" { source } (style: debug) => stringify!($literal_ty);

                expected(style: debug) = expected;
                parsed(style: display) = { lexer.lookahead() };

                assert { lexer.lookahead_is::<$literal_ty>() } or "Failed to peek as a literal";
            };

            match lexer.try_parse::<Literal>().ok() {
                None => {
                    assert_matches! {
                        "lexing" { source } (style: debug) => stringify!(Literal);

                        expected(style: debug) = Some(expected);
                        parsed(style: debug) = None::<Literal>;

                        assert { false } or "Literal lex failure";
                    };
                }

                Some(token) => {
                    assert_matches! {
                        "lexing" { source } (style: debug) => stringify!(Literal);

                        expected(style: debug) = expected;
                        parsed(style: debug) = token;

                        assert { &token == actual_literal.unspanned() } or "Literal lex failure";
                    };
                }
            }

            Ok(())
        }

        let panic = std::panic::catch_unwind(|| {
            let expected = $expected;
            let borrow = expected.borrow();
            test_literal::<$literal_ty>($source, borrow).unwrap()
        });

        match panic {
            Ok(_) => {}
            Err(err) => panic!("{}", PanicMessage(&err)),
        }
    }};

    ($source:tt => $literal_ty:ident($expected:expr)) => {{
        fn test_literal<$literal_ty: LiteralTrait>(
            source: &str,
            expected: &$literal_ty::Output,
        ) -> TestResult {
            let actual_literal = lit(source);

            assert!(
                $literal_ty::matches_value(&actual_literal.unspanned(), expected),
                "Literal parse failure:\n - expected={:?}\n - parsed={}",
                expected,
                actual_literal.unspanned()
            );

            // assert!(literal.unspanned().is_string(expected));
            let source_round_trip = actual_literal.to_tt().to_string();

            if source_round_trip != source {
                test_literal::<$literal_ty>(&source_round_trip, expected)?;
            }

            Ok(())
        }

        let panic = std::panic::catch_unwind(|| {
            test_literal::<$literal_ty>($source, $expected.borrow()).unwrap()
        });

        match panic {
            Ok(_) => {}
            Err(err) => panic!("{}", PanicMessage(&err)),
        }
    }};
}

struct PanicMessage<'a>(&'a (dyn Any + Send + 'static));

impl<'a> std::fmt::Display for PanicMessage<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if let Some(str_slice) = self.0.downcast_ref::<&'static str>() {
            fmt.pad(str_slice)?;
        } else if let Some(string) = self.0.downcast_ref::<String>() {
            fmt.pad(string)?;
        } else {
            fmt.pad("Box<Any>")?;
        }

        Ok(())
    }
}

fn lit(s: &str) -> Spanned<OxidateLiteral> {
    match TokenStream::from_str(s)
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
    {
        TokenTree::Literal(lit) => lit.into(),
        other => panic!(
            "{}: Expected literal, got {:?}",
            "PARSE ERROR".red().reversed(),
            other
        ),
    }
}

type TestResult = Result<(), Box<dyn Error>>;

macro_rules! test {
    ($literal:ident $block:block) => {
        paste::paste! {
            #[test]
            fn [<$literal:snake>]() -> TestResult {
                macro_rules! parses {
                    ($source:tt => $expected:expr) => {
                        test_literal!($source => $literal($expected))
                    };

                    ($source:tt => $expected:tt $suffix:tt) => {
                        test_literal!($source => $literal($expected) $suffix)
                    };

                    ($source:tt => $expected_macro:ident ! $expected:tt $suffix:tt) => {
                        test_literal!($source => $literal($expected_macro! $expected) $suffix)
                    };
                }
                $block

                Ok(())
            }
        }
    };
}

macro_rules! unimplemented_suffixes {
    ($($tokens:tt)*) => {};
}

macro_rules! scenario {
    ($name:ident { $($tokens:tt)* }) => {
        $($tokens)*
    };

    ($name:tt { $($tokens:tt)* }) => {
        $($tokens)*
    };


}

test!(StringLiteral {
    parses!("\"a\"" => "a");
    parses!("\"\\n\"" => "\n");
    parses!("\"\\r\"" => "\r");
    parses!("\"\\t\"" => "\t");
    parses!("\"🐕\"" => "🐕"); // NOTE: This is an emoji
    parses!("\"\\\"\"" => "\"");
    parses!("\"'\"" => "'");
    parses!("\"\"" => "");
    parses!("\"\\u{1F415}\"" => "\u{1F415}");
    parses!("\"\\u{1_2__3_}\"" => "\u{123}");
    parses!(
        "\"contains\nnewlines\\\nescaped newlines\"" =>
        "contains\nnewlinesescaped newlines"
    );
    parses!("r\"raw\nstring\\\nhere\"" => "raw\nstring\\\nhere");

    unimplemented_suffixes! {
        parses("\"...\"q", "...");
        parses("r\"...\"q", "...");
        parses("r##\"...\"##q", "...");
    }
});

test!(ByteStringLiteral {
    parses!("b\"a\"" => b"a");
    parses!("b\"\\n\"" => b"\n");
    parses!("b\"\\r\"" => b"\r");
    parses!("b\"\\t\"" => b"\t");
    parses!("b\"\\\"\"" => b"\"");
    parses!("b\"'\"" => b"'");
    parses!("b\"\"" => b"");
    parses!(
        "b\"contains\nnewlines\\\nescaped newlines\"" =>
        b"contains\nnewlinesescaped newlines"
    );
    parses!("br\"raw\nstring\\\nhere\"" => b"raw\nstring\\\nhere");

    unimplemented_suffixes! {
        parses!("b\"...\"q" => b"...");
        parses!("br\"...\"q" => b"...");
        parses!("br##\"...\"##q" => b"...");
    }
});

test!(ByteLiteral {
    parses!("b'a'" => &b'a');
    parses!("b'\\n'" => &b'\n');
    parses!("b'\\r'" => &b'\r');
    parses!("b'\\t'" => &b'\t');
    parses!("b'\\''" => &b'\'');
    parses!("b'\"'" => &b'"');

    unimplemented_suffixes! {
        parses!("b'a'q" => &b'a');
    }

});

test!(CharLiteral {
    parses!("'a'" => 'a');
    parses!("'\\n'" => '\n');
    parses!("'\\r'" => '\r');
    parses!("'\\t'" => '\t');
    parses!("'🐕'" => '🐕'); // NOTE: This is an emoji
    parses!("'\\''" => '\'');
    parses!("'\"'" => '"');
    parses!("'\\u{1F415}'" => '\u{1F415}');

    unimplemented_suffixes! {
        parses!("'a'q" => 'a');
    }
});

test!(IntegerLiteral {
    parses!("5" => 5 "");
    parses!("5u32" => 5 "u32");
    parses!("5_0" => 50 "");
    parses!("5_____0_____" => 50  "");
    parses!("0x7f" => 127 "");
    parses!("0x7F" => 127 "");
    parses!("0b1001" => 9 "");
    parses!("0o73" => 59 "");
    parses!("0x7Fu8" => 127 "u8");
    parses!("0b1001i8" => 9 "i8");
    parses!("0o73u32" => 59 "u32");
    parses!("0x__7___f_" => 127 "");
    parses!("0x__7___F_" => 127 "");
    parses!("0b_1_0__01" => 9 "");
    parses!("0o_7__3" => 59 "");
    parses!("0x_7F__u8" => 127 "u8");
    parses!("0b__10__0_1i8" => 9 "i8");
    parses!("0o__7__________________3u32" => 59 "u32");

    scenario!(negative {
        // Negative numbers aren't literals in Rust. They're a unary operator
        // applied to an integer.
        //
        // Negative numbers are a higher level production in oxidate.

        //     assert_eq!("-1", LitInt::new("-1", span).to_string());
        //     assert_eq!("-1i8", LitInt::new("-1i8", span).to_string());
        //     assert_eq!("-1i16", LitInt::new("-1i16", span).to_string());
        //     assert_eq!("-1i32", LitInt::new("-1i32", span).to_string());
        //     assert_eq!("-1i64", LitInt::new("-1i64", span).to_string());
        //     assert_eq!("-1.5", LitFloat::new("-1.5", span).to_string());
        //     assert_eq!("-1.5f32", LitFloat::new("-1.5f32", span).to_string());
        //     assert_eq!("-1.5f64", LitFloat::new("-1.5f64", span).to_string());

    });

    unimplemented_suffixes! {
        parses!("0ECMA" => 0 "ECMA");
        parses!("0E" => 0 "E");
        parses!("0o0A" => 0 "A");
        parses!("0e1\u{5c5}" => 0 "e1\u{5c5}");
    }
});

macro_rules! float {
    ($float:tt) => {{
        bigdecimal::BigDecimal::from_str(stringify!($float)).unwrap()
    }};
}

test!(FloatLiteral {
    parses!("5.5" => float!(5.5) "");
    parses!("5.5E12" => float!(5.5e12) "");
    parses!("5.5e12" => float!(5.5e12) "");
    // parses!("1.0__3e-12" => float!(1.03e-12) "");
    parses!("1.03e+12" => float!(1.03e12) "");

    unimplemented_suffixes! {
        parses!("9e99e99" => float!(9e99) "e99");
        parses!("0.0ECMA" => float!(0.0) "ECMA");
    }

    parses!("1e_0" => float!(1.0) "");

});

// #[test]
// fn negative_overflow() {
//     assert!(syn::parse_str::<LitFloat>("-1.0e99f64").is_ok());
//     assert!(syn::parse_str::<LitFloat>("-1.0e999f64").is_err());
// }

// #[test]
// fn suffix() {
//     fn get_suffix(token: &str) -> String {
//         let lit = syn::parse_str::<Lit>(token).unwrap();
//         match lit {
//             Lit::Str(lit) => lit.suffix().to_owned(),
//             Lit::ByteStr(lit) => lit.suffix().to_owned(),
//             Lit::Byte(lit) => lit.suffix().to_owned(),
//             Lit::Char(lit) => lit.suffix().to_owned(),
//             Lit::Int(lit) => lit.suffix().to_owned(),
//             Lit::Float(lit) => lit.suffix().to_owned(),
//             _ => unimplemented!(),
//         }
//     }

//     assert_eq!(get_suffix("\"\"s"), "s");
//     assert_eq!(get_suffix("r\"\"r"), "r");
//     assert_eq!(get_suffix("b\"\"b"), "b");
//     assert_eq!(get_suffix("br\"\"br"), "br");
//     assert_eq!(get_suffix("r#\"\"#r"), "r");
//     assert_eq!(get_suffix("'c'c"), "c");
//     assert_eq!(get_suffix("b'b'b"), "b");
//     assert_eq!(get_suffix("1i32"), "i32");
//     assert_eq!(get_suffix("1_i32"), "i32");
//     assert_eq!(get_suffix("1.0f32"), "f32");
//     assert_eq!(get_suffix("1.0_f32"), "f32");
// }

// #[test]
// fn test_deep_group_empty() {
//     let tokens = TokenStream::from_iter(vec![TokenTree::Group(Group::new(
//         Delimiter::None,
//         TokenStream::from_iter(vec![TokenTree::Group(Group::new(
//             Delimiter::None,
//             TokenStream::from_iter(vec![TokenTree::Literal(Literal::string("hi"))]),
//         ))]),
//     ))]);

//     snapshot!(tokens as Lit, @r#""hi""# );
// }

// #[test]
// fn test_error() {
//     let err = syn::parse_str::<LitStr>("...").unwrap_err();
//     assert_eq!("expected string literal", err.to_string());

//     let err = syn::parse_str::<LitStr>("5").unwrap_err();
//     assert_eq!("expected string literal", err.to_string());
// }
