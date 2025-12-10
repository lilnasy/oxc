use super::{Kind, Lexer, Token, TokenStore};

impl<'a, Store: TokenStore<'a>> Lexer<'a, Store> {
    /// Re-tokenize '<<' or '<=' or '<<=' to '<'
    pub(crate) fn re_lex_as_typescript_l_angle(&mut self, offset: u32) -> Token {
        self.token.set_start(self.offset() - offset);
        self.source.back(offset as usize - 1);
        self.finish_next(Kind::LAngle)
    }

    /// Re-tokenize '>>' and '>>>' to '>'
    pub(crate) fn re_lex_as_typescript_r_angle(&mut self, offset: u32) -> Token {
        self.token.set_start(self.offset() - offset);
        self.source.back(offset as usize - 1);
        self.finish_next(Kind::RAngle)
    }
}
