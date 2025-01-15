#[cfg(test)]
mod tests {
    use lng::lexer::Token;

    #[test]
    fn token_has_expected_values() {
        assert_eq!(Token::Eof as i32, -1);
        assert_eq!(Token::Def as i32, -2);
        assert_eq!(Token::Extern as i32, -3);
        assert_eq!(Token::Identifier as i32, -4);
        assert_eq!(Token::Number as i32, -5);
    }
}
