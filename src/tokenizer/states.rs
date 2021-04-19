pub use self::State::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
pub enum AttrValueKind {
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
pub enum State {
    Data,
    Plaintext,
    TagOpen,
    EndTagOpen,
    TagName,
    SelfClosingStartTag,
    Doctype,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    AttributeValue,
    BeforeAttributeValue,
}