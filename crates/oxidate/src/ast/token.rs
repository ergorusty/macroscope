use oxidate_macros::data;

use crate::{error::MacroResult, AstParse, AstSpan, ParseBuffer, Span};

// This macro should only be used in this file. This means we don't need to
// defensively namespace everything via $crate.
macro_rules! def_token_enum {
    (@peek punctuation $peeked:ident [$chars:tt] => $variant:ident) => {
        if let Some(lexed) = $peeked.as_punctuation(stringify!($chars)) {
            return Some(UnspannedToken::$variant.spanned(lexed.span()))
        }
    };

    (@peek word $peeked:ident [$chars:tt] => $variant:ident) => {
        if let Some(lexed) = $peeked.as_word(stringify!($chars)) {
            return Some(UnspannedToken::$variant.spanned(lexed.span()))
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

    (
        $(
            $kind:ident [$token:tt] => $name:ident $(
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
    ) => {
        #[data(copy)]
        pub enum UnspannedToken {
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

        impl Token {
            fn next_from(input: &mut ParseBuffer) -> Option<Token> {
                let peeked = input.peek();
                $(
                    def_token_enum!(@peek $kind peeked [$token] => $name);
                )*

                None
            }
        }


        // impl AstParse for Token {
        // }

        // impl AstPeek for Token {
        //     fn peek(input: &mut ParseBuffer) -> bool {
        //         $(
        //             def_token_enum!(@peek $kind input [$token] => )
        //             // $($(
        //             //     $nest1_variant,
        //             //     $($(
        //             //         $nest2_variant,
        //             //     )*)?
        //             // )*)?
        //         )*

        //         return false;
        //     }

        // }

        #[macro_export]
        macro_rules! Token {
            $(
                ($token) => {
                    $crate::Token::$name
                };
            )*
        }

    }
}

pub enum RawDecompose {
    Double(UnspannedToken, UnspannedToken),
    Triple(UnspannedToken, UnspannedToken, UnspannedToken),
}

#[data(copy)]
pub struct Token {
    span: Span,
    unspanned: UnspannedToken,
}

impl AstParse for Token {
    fn try_parse(input: &mut ParseBuffer) -> MacroResult<Option<Self>> {
        Ok(Token::next_from(input))
    }
}

impl AstSpan for Token {
    type Unspanned = UnspannedToken;

    fn span(&self) -> Span {
        self.span
    }

    fn unspanned(&self) -> &Self::Unspanned {
        &self.unspanned
    }
}

impl UnspannedToken {
    fn spanned(self, span: Span) -> Token {
        Token {
            span,
            unspanned: self,
        }
    }
}

def_token_enum! {
    word [abstract] => Abstract
    word [as] => As
    word [async] => Async
    word [auto] => Auto
    word [await] => Await
    word [become] => Become
    word [box] => Box
    word [break] => Break
    word [const] => Const
    word [continue] => Continue
    word [crate] => Crate
    word [default] => Default
    word [do] => Do
    word [dyn] => Dyn
    word [else] => Else
    word [enum] => Enum
    word [extern] => Extern
    word [final] => Final
    word [fn] => Fn
    word [for] => For
    word [if] => If
    word [impl] => Impl
    word [in] => In
    word [let] => Let
    word [loop] => Loop
    word [macro] => Macro
    word [match] => Match
    word [mod] => Mod
    word [move] => Move
    word [mut] => Mut
    word [override] => Override
    word [priv] => Priv
    word [pub] => Pub
    word [ref] => Ref
    word [return] => Return
    word [Self] => SelfType
    word [self] => SelfVariable
    word [static] => Static
    word [struct] => Struct
    word [super] => Super
    word [trait] => Trait
    word [try] => Try
    word [type] => Type
    word [typeof] => Typeof
    word [union] => Union
    word [unsafe] => Unsafe
    word [unsized] => Unsized
    word [use] => Use
    word [virtual] => Virtual
    word [where] => Where
    word [while] => While
    word [yield] => Yield
    punctuation [+] => Plus {
        [=] => PlusEquals
    }
    punctuation [&] => And {
        [&] => AndAnd
        [=] => AndEquals
    }
    punctuation [@] => At
    punctuation [!] => Bang {
        [=] => NotEquals
    }
    punctuation [^] => Hat {
        [=] => HatEquals
    }
    punctuation [:] => Colon {
        [:] => ColonColon
    }
    punctuation [,] => Comma
    punctuation [/] => Divide {
        [=] => DivideEquals
    }
    punctuation [$] => Dollar
    punctuation [.] => Dot {
        [.] => DotDot {
            [.] => DotDotDot
            [=] => DotDotEquals
        }
    }
    punctuation [=] => Equals {
        [=] => EqualsEquals
        [>] => ThickArrow
    }
    punctuation [>] => Gt {
        [=] => GtEquals
        [>] => GtGt {
            [=] => GtGtEquals
        }
    }
    punctuation [<] => Lt {
        [=] => LtEquals
        [-] => ThinArrowLeft
        [<] => LtLt {
            [=] => LtLtEquals
        }
    }
    punctuation [*] => Star {
        [=] => StarEquals
    }
    punctuation [|] => Pipe {
        [=] => PipeEquals
        [|] => PipePipe
    }
    punctuation [-] => Minus {
        [>] => ThinArrowRight
        [=] => MinusEquals
    }
    punctuation [#] => Hash
    punctuation [?] => Question
    punctuation [%] => Percent {
        [=] => PercentEquals
    }
    punctuation [;] => Semicolon
    punctuation [~] => Tilde
    punctuation [_] => Underscore
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use quote::quote;
}
