use std::collections::HashSet;

use mio::Token;

/// Allocates and manages MIO tokens, to ensure unique tokens for its users.
pub struct TokenManager {
    used_tokens: HashSet<Token>,
    free_tokens: Vec<Token>,
    next: usize,
}

impl TokenManager {
    pub fn new() -> TokenManager {
        TokenManager {
            used_tokens: HashSet::new(),
            free_tokens: Vec::new(),
            next: 0,
        }
    }

    /// Allocate an unused token.
    pub fn allocate_token(&mut self) -> Token {
        if let Some(token) = self.free_tokens.pop() {
            self.used_tokens.insert(token);
            token
        } else {
            let token = Token(self.next);
            self.next += 1;
            self.used_tokens.insert(token);
            token
        }
    }

    /// Release a token.
    pub fn free_token(&mut self, token: Token) {
        if self.used_tokens.remove(&token) {
            self.free_tokens.push(token);
        }
    }
}
