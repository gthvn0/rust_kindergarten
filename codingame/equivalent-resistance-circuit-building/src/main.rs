#[derive(Debug)]
enum Token {
    PO,
    PF,
    CO,
    CF,
    R(f32),
}

fn next_token(e: &[Token]) -> Option<Token> {
    if e.is_empty() {
        return None;
    }

    return match e[0] {
        Token::PO => Some(Token::PO),
        Token::PF => Some(Token::PF),
        Token::CO => Some(Token::CO),
        Token::CF => Some(Token::CF),
        Token::R(x) => Some(Token::R(x)),
    };
}

fn parallel(e: &[Token]) -> f32 {
    match next_token(e) {
        Some(Token::CF) => 0.0 + parse(&e[1..]),
        Some(Token::CO) => 1.0 / parallel(&e[1..]),
        Some(Token::PF) => 0.0 + parse(&e[1..]),
        Some(Token::PO) => serie(&e[1..]),
        Some(Token::R(x)) => (1.0 / x) + parallel(&e[1..]),
        _ => panic!("WTF"),
    }
}

fn serie(e: &[Token]) -> f32 {
    match next_token(e) {
        Some(Token::PF) => 0.0 + parse(&e[1..]),
        Some(Token::PO) => serie(&e[1..]),
        Some(Token::CF) => 0.0 + parse(&e[1..]),
        Some(Token::CO) => 1.0 / parallel(&e[1..]),
        Some(Token::R(x)) => x + serie(&e[1..]),
        _ => panic!("WTF"),
    }
}

fn parse(e: &[Token]) -> f32 {
    match next_token(e) {
        Some(Token::CO) => 1.0 / parallel(&e[1..]),
        Some(Token::PO) => serie(&e[1..]),
        Some(Token::CF) => 0.0 + parse(&e[1..]),
        Some(Token::PF) => 0.0 + parse(&e[1..]),
        None => 0.0,
        _ => {
            println!("Token {:?} not expected here", e);
            0.0
        }
    }
}

fn main() {
    //A 24
    //B 8
    //C 48
    // This will look something like this:
    //
    //        +---[C]---+
    //        |         |
    //     +--+         +--+
    //     |  |         |  |
    //     |  +---[A]---+  |
    //     |               |
    //     +---[A]---[B]---+
    //     |               |
    //     +---[Battery]---+
    //
    // [ ( A B ) [ C A ] ] => [ 24+8 1/(1/48+1/24) ] => [ 32 16 ] => 1/(1/32+1/16) => 32/3 => 10.666... => 10.7
    let e2: [Token; 10] = [
        Token::CO,
        Token::PO,
        Token::R(24.0),
        Token::R(8.0),
        Token::PF,
        Token::CO,
        Token::R(48.0),
        Token::R(24.0),
        Token::CF,
        Token::CF,
    ];

    println!("R: {}", parse(&e2[..]));
}
