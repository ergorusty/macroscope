use oxidate_macros::data;

use crate::{AstParse, Lexer, ParseOutcome};

// This macro should only be used in this file. This means we don't need to
// defensively namespace everything via $crate.
macro_rules! def_token_enum {
    (@parse($lexer:expr) punctuation $peeked:ident [$chars:tt] => $variant:ident) => {
        if $peeked.is_punctuation(stringify!($chars)) {
            return $peeked.commit(RustOperator::$variant);
        }
    };

    (@parse($lexer:expr) word $peeked:ident [$chars:tt] => $variant:ident) => {
        if $peeked.is_word(stringify!($chars)) {
            return $peeked.commit(RustKeyword::$variant);
        }
    };

    (@peek punctuation $peeked:ident [$chars:tt] => $variant:ident) => {
        if let Some(_) = $peeked.as_punctuation(stringify!($chars).chars().nth(0).unwrap()) {
            return true;
        }
    };

    (@peek word $peeked:ident [$chars:tt] => $variant:ident) => {
        if let Some(_) = $peeked.as_word(stringify!($chars)) {
            return true;
        }
    };

    (@variant { [$nest:tt] => $variant:ident $( { $($nested:tt)* } )? }) => {
        $(
            $variant,
        )*

        $(
            ,
            def_token_enum!(@variant $($nested)*)
        )?
    };


    (RustKeyword($unspanned:ident) $nested:tt) => {
        def_token_enum! {
            word RustKeyword($unspanned) $nested
        }
    };

    (RustOperator($unspanned:ident) $nested:tt) => {
        def_token_enum! {
            punctuation RustOperator($unspanned) $nested
        }
    };

    (
        $kind:ident $enum_name:ident($unspanned_name:ident) {
            $(
                [$token:tt] => $name:ident $(
                    {
                        $(
                            [$nest1:tt] => $nest1_variant:ident
                            $(
                                {
                                    $(
                                        [$nest2:tt] => $nest2_variant:ident
                                    )*
                                }
                            )?
                        )*
                    }
                )?
            )*
        }
    ) => {
        #[data(copy)]
        #[derive(Eq, PartialEq)]
        pub enum $unspanned_name {
            $(
                $name,

                $($(
                    $nest1_variant,
                    $($(
                        $nest2_variant,
                    )*)?
                )*)?
            )*
        }

        impl $enum_name {
            fn next_from(lexer: &mut Lexer) -> ParseOutcome<$enum_name> {
                let mut peeked = lexer.begin();

                $(
                    def_token_enum!(@parse(lexer) $kind peeked [$token] => $name);
                )*

                ParseOutcome::LookaheadMismatch
            }

            fn peek_from(lexer: &mut Lexer) -> bool {
                let peeked = lexer.lookahead();

                $(
                    def_token_enum!(@peek $kind peeked [$token] => $name);
                )*

                false

            }

            fn as_str(&self) -> &'static str {
                match self {
                    $(
                        $enum_name::$name => stringify!($token),

                        $(
                            $(
                                $enum_name::$nest1_variant => concat!(stringify!($token), stringify!($nest1)),

                                $(
                                    $(
                                        $enum_name::$nest2_variant => concat!(stringify!($token), stringify!($nest1), stringify!($nest3)),
                                    )*
                                )?
                            )*
                        )?
                    )*
                }
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        // impl AstPeek for RustLeafToken {
        //     fn peek(&self, lexer: &mut Lexer) -> bool {
        //         let peeked = lexer.lookahead();
        //         $(
        //             def_token_enum!(@peek $kind peeked [$token] => $name);

        //             $(
        //                 $(
        //                     def_token_enum!(@peek $kind peeked [$nest1] => $nest1_variant);

        //                     $(
        //                         $(
        //                             def_token_enum!(@peek $kind peeked [$nest2] => $nest2_variant);
        //                         )*
        //                     )?
        //                 )*
        //             )?
        //         )*

        //         false
        //     }
        // }


        #[macro_export]
        macro_rules! $enum_name {
            $(
                ($token) => {
                    $crate::Token::$name
                };
            )*
        }

    }
}
pub enum RawDecompose {
    Double(RustOperator, RustOperator),
    Triple(RustOperator, RustOperator, RustOperator),
}

impl AstParse for RustKeyword {
    fn try_parse(lexer: &mut Lexer) -> crate::ParseOutcome<Self> {
        RustKeyword::next_from(lexer)
    }

    fn check(lexer: &mut Lexer) -> bool {
        RustKeyword::peek_from(lexer)
    }
}

impl AstParse for RustOperator {
    fn try_parse(lexer: &mut Lexer) -> ParseOutcome<Self> {
        RustOperator::next_from(lexer)
    }

    fn check(lexer: &mut Lexer) -> bool {
        RustOperator::peek_from(lexer)
    }
}

def_token_enum! {
    RustKeyword(RustKeyword) {
        [abstract] => Abstract
        [as] => As
        [async] => Async
        [auto] => Auto
        [await] => Await
        [become] => Become
        [box] => Box
        [break] => Break
        [const] => Const
        [continue] => Continue
        [crate] => Crate
        [default] => Default
        [do] => Do
        [dyn] => Dyn
        [else] => Else
        [enum] => Enum
        [extern] => Extern
        [final] => Final
        [fn] => Fn
        [for] => For
        [if] => If
        [impl] => Impl
        [in] => In
        [let] => Let
        [loop] => Loop
        [macro] => Macro
        [match] => Match
        [mod] => Mod
        [move] => Move
        [mut] => Mut
        [override] => Override
        [priv] => Priv
        [pub] => Pub
        [ref] => Ref
        [return] => Return
        [Self] => SelfType
        [self] => SelfVariable
        [static] => Static
        [struct] => Struct
        [super] => Super
        [trait] => Trait
        [try] => Try
        [type] => Type
        [typeof] => Typeof
        [union] => Union
        [unsafe] => Unsafe
        [unsized] => Unsized
        [use] => Use
        [virtual] => Virtual
        [where] => Where
        [while] => While
        [yield] => Yield
    }
}

def_token_enum! {
    RustOperator(RustOperator) {
        [+] => Plus {
            [=] => PlusEquals
        }
        [&] => And {
            [&] => AndAnd
            [=] => AndEquals
        }
        [@] => At
        [!] => Bang {
            [=] => NotEquals
        }
        [^] => Hat {
            [=] => HatEquals
        }
        [:] => Colon {
            [:] => ColonColon
        }
        [,] => Comma
        [/] => Divide {
            [=] => DivideEquals
        }
        [$] => Dollar
        [.] => Dot {
            [.] => DotDot {
                [.] => DotDotDot
                [=] => DotDotEquals
            }
        }
        [=] => Equals {
            [=] => EqualsEquals
            [>] => ThickArrow
        }
        [>] => Gt {
            [=] => GtEquals
            [>] => GtGt {
                [=] => GtGtEquals
            }
        }
        [<] => Lt {
            [=] => LtEquals
            [-] => ThinArrowLeft
            [<] => LtLt {
                [=] => LtLtEquals
            }
        }
        [*] => Star {
            [=] => StarEquals
        }
        [|] => Pipe {
            [=] => PipeEquals
            [|] => PipePipe
        }
        [-] => Minus {
            [>] => ThinArrowRight
            [=] => MinusEquals
        }
        [#] => Hash
        [?] => Question
        [%] => Percent {
            [=] => PercentEquals
        }
        [;] => Semicolon
        [~] => Tilde
        [_] => Underscore
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use quote::quote;
}
