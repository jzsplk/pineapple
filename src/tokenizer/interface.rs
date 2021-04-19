use std::borrow::Cow;

pub use self::TagKind::{StartTag,EndTag};

#[derive(Debug,Clone, Eq, PartialEq)]
pub enum TagKind {
    StartTag,
    EndTag
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Tag {
    pub kind: TagKind,
    pub name: String,
    pub self_closing: bool,
    pub atts: Vec<Attribute>
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Attribute {
    /// The name of the attribute (e.g. the `class` in `<div class="test">`)
    pub name: String,
    /// The value of the attribute (e.g. the `"test"` in `<div class="test">`)
    pub value: String,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    TagToken(Tag),
    CommentToken(String),
    CharacterTokens(String),
    NullCharacterToken,
    EOFToken,
    ParseError(Cow<'static, str>),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum TokenSinkResult<Handle> {
    Continue,
    Script(Handle),
    Plaintext,
    RawData,
}

// Types which can receive tokens from the tokenizer.
pub trait TokenSink {
    type Handle;

    /// Process a token.
    fn process_token(&mut self, token: Token, line_number: u64) -> TokenSinkResult<Self::Handle>;

    // Signal sink that tokenization reached the end.
    fn end(&mut self) {}
}