#[warn(unused_imports)]

use self::states::{Data,TagOpen,EndTagOpen,State};
use self::interface::{Token,TokenSink,TagKind,Attribute,TokenSinkResult};
// use self::states::{};
use log::debug;

pub mod states;
mod interface;

pub enum ProcessResult<Handle> {
    Continue,
    Suspend,
    Script(Handle),
}

#[must_use]
pub enum TokenizerResult<Handle> {
    Done,
    Script(Handle),
}

pub struct Tokenizer<Sink> {
    pub sink: Sink,
    state: states::State, 
    at_eof: bool,
    current_char: char,
    current_tag_kind: TagKind,
    // Current tag name.
    current_tag_name: String,

    // Current tag is self-closing?
    current_tag_self_closing: bool,

    // Current tag attributes.
    current_tag_attrs: Vec<Attribute>,
    
    // Current attribute name.
    current_attr_name: String,

    // Current attribute value.
    current_attr_value: String,
    
    // Track current line
    current_line: u64,
}

impl<Sink: TokenSink> Tokenizer<Sink> {
    pub fn new(sink: Sink) -> Tokenizer<Sink> {
        let state = states::Data;
        Tokenizer {
            sink,
            state,
            at_eof: false,
            current_char: '\0',
            current_tag_kind: TagKind::StartTag,
            current_tag_name: String::new(),
            current_line: 1,
            current_attr_name: String::new(),
            current_attr_value: String::new(),
            current_tag_attrs: vec![],
            current_tag_self_closing: false,
        }
    }
    
    // Feed the input string into the tokenizer
    pub fn feed(&mut self, input: &mut String) -> TokenizerResult<Sink::Handle> {
       if input.is_empty() {
            return TokenizerResult::Done;
       } 
       
       self.run(input)
    }
    
    fn process_token(&mut self, token: Token) -> TokenSinkResult<Sink::Handle> {
        self.sink.process_token(token, self.current_line)
    }

    fn run(&mut self, input: &mut String) -> TokenizerResult<Sink::Handle> {
        // loop {
        //     match self
        // }
        TokenizerResult::Done
    }
}
// END

// shorthand for common state machine behaviours.
macro_rules! shorthand {
    ($me:ident : emit $c:expr                  ) => ( $me.emit_char($c);                               );
    ($me:ident : create_tag $kind:ident $c:expr) => ( $me.create_tag($kind, $c);                       );
}

macro_rules! pop_except_from {
    ($me:expr, $input:expr, $set:expr) => {
        return unwrap_or_return!($me.pop_except_from($input, $set), ProcessResult::Suspend);
    };
}

macro_rules! get_char ( ($me:expr, $input:expr) => (
    unwrap_or_return!($me.get_char($input), ProcessResult::Suspend)
));

impl<Sink: TokenSink> Tokenizer<Sink> {
    #[allow(clippy::never_loop)]
    fn step(&mut self, input: &mut String) -> ProcessResult<Sink::Handle> {
        debug!("processing in state {:?}", self.state);

        ProcessResult::Continue
        // match self.state {
        //    states::Data => loop {
        //       matches pop_except_from!(self, input) 
        //    },
        //    _ => (()) 
        // }
    }
    
}
