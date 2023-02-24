#[derive(Debug)]
enum Object {
    string(String),
    integer(u32),

}

#[derive(Debug)]
struct Token{
    t_type: TokenType,
    lexeme: String,
    literal: Object,
    line: u32,
}

impl Token{
    fn new(t_type: TokenType, lexeme: String, literal: Object, line: u32) -> Self {
        let t = Token {
            t_type: t_type,
            lexeme: lexeme,
            literal: literal,
            line: line,
            
        };
        return t;
    }
}
fn main() {
    println!("yuh");
    let t: Token = Token::new(TokenType::BANG, String::from("h"), Object::integer(3), 3);

    dbg!(t);
}

#[derive(Debug)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // ONE-TWO CHARACTER TOKENS
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // LITERALS
    IDENTIFIER,
    STRING,
    NUMBER,
    //
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF
}