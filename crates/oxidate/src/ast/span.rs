use oxidate_macros::data;

#[data(copy)]
pub struct Span {
    inner: proc_macro2::Span,
}

impl Into<proc_macro2::Span> for Span {
    fn into(self) -> proc_macro2::Span {
        self.inner
    }
}

impl From<proc_macro2::Span> for Span {
    fn from(span: proc_macro2::Span) -> Self {
        Span { inner: span }
    }
}
