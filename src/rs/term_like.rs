use crate::rs::term::Term;

pub trait TermLike {
    fn value(&self) -> &str;

    fn as_term(self) -> Term;
    fn to_term(&self) -> Term;
}
