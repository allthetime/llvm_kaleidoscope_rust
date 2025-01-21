#[cfg(test)]
mod tests {
    use lng::lexer::get_tokens;
    use lng::lexer::Token;

    #[test]
    fn lexer_get_tokens_test() {
        let input = "def foo(x) extern 42";
        let input_chars: Vec<char> = input.chars().collect();
        let tokens = get_tokens(&input_chars);

        dbg!(&tokens);

        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("foo".to_string()));
        assert_eq!(tokens[2], Token::Unknown('('));
        assert_eq!(tokens[3], Token::Identifier("x".to_string()));
        assert_eq!(tokens[4], Token::Unknown(')'));
        assert_eq!(tokens[5], Token::Extern);
        assert_eq!(tokens[6], Token::Number(42.0));
        assert_eq!(tokens[7], Token::Eof);
    }
}
