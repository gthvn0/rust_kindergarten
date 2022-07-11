use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct Resistance {
    kind: RType,
    value: f32,
}

#[derive(Debug, PartialEq)]
enum RType {
    Serie,
    Parallel,
}

fn push_serie(s: &mut Vec<Resistance>) {
    s.push(Resistance {
        kind: RType::Serie,
        value: 0.0,
    });
}

fn push_parallel(s: &mut Vec<Resistance>) {
    s.push(Resistance {
        kind: RType::Parallel,
        value: 0.0,
    });
}

fn pop_resistance(s: &mut Vec<Resistance>) {
    match s.pop() {
        None => panic!("Something goes wrong you should have a least one serie in progress"),
        Some(v) => {
            // On doit basculer la valeur dans l'element du dessous
            //
            match s.pop() {
                // Si on est le dernier item on va inverser dans le cas des resistances
                // en paralleles.
                None => {
                    let new_value = if v.kind == RType::Parallel {
                        1.0 / v.value
                    } else {
                        v.value
                    };
                    s.push(Resistance {
                        kind: v.kind,
                        value: new_value,
                    })
                }
                Some(down) => {
                    let new_value = if down.kind != v.kind {
                        down.value + (1.0 / v.value)
                    } else {
                        down.value + v.value
                    };
                    s.push(Resistance {
                        kind: down.kind,
                        value: new_value,
                    });
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn evaluate(vt: Vec<Token>, rv: &HashMap<&str, f32>) -> f32 {
    let mut stack: Vec<Resistance> = Vec::new();

    for tok in vt {
        //eprintln!("Token {:?}", tok);
        match tok {
            Token::Po => push_serie(&mut stack),
            Token::Bo => push_parallel(&mut stack),
            Token::Pc => pop_resistance(&mut stack),
            Token::Bc => pop_resistance(&mut stack),
            Token::Id(id) => {
                // Get the resistance of the value
                match rv.get(id) {
                    Some(v) => {
                        eprintln!(" > Found Res {}", v);
                        let elmt = stack.pop().unwrap();
                        let new_val = if elmt.kind == RType::Serie {
                            elmt.value + v
                        } else {
                            elmt.value + (1.0 / v)
                        };
                        eprintln!("   > Pushing {}", new_val);
                        stack.push(Resistance {
                            kind: elmt.kind,
                            value: new_val,
                        });
                    }
                    None => panic!("{} not found", id),
                }
            }
        }
    }

    // Result should be the only item remaining in the stack
    assert!(stack.len() == 1);
    match stack.pop() {
        Some(v) => v.value,
        _ => panic!("Stack shoulnd't be empty"),
    }
}

#[derive(Debug)]
enum Token<'a> {
    Po,
    Pc,
    Bo,
    Bc,
    Id(&'a str),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut v: Vec<Token> = Vec::new();

    for item in input.split(' ') {
        match item {
            "(" => v.push(Token::Po),
            ")" => v.push(Token::Pc),
            "[" => v.push(Token::Bo),
            "]" => v.push(Token::Bc),
            s => v.push(Token::Id(s)),
        }
    }

    v
}

#[test]
fn test_serie_easy() {
    let mut resistance_value: HashMap<&str, f32> = HashMap::new();

    resistance_value.insert(&"A", 24.0);
    resistance_value.insert(&"B", 8.0);

    assert_eq!(32.0, evaluate(tokenize(&"( A B )"), &resistance_value));
}

#[test]
fn test_parallel_easy() {
    let mut resistance_value: HashMap<&str, f32> = HashMap::new();

    resistance_value.insert(&"A", 24.0);
    resistance_value.insert(&"B", 8.0);

    assert_eq!(6.0, evaluate(tokenize(&"[ A B ]"), &resistance_value));
}

#[test]
fn test_serie_in_parallel() {
    let mut resistance_value: HashMap<&str, f32> = HashMap::new();

    resistance_value.insert(&"A", 24.0);
    resistance_value.insert(&"B", 8.0);
    resistance_value.insert(&"C", 48.0);

    assert_eq!(
        10.666667,
        evaluate(tokenize(&"[ ( A B ) [ C A ] ]"), &resistance_value)
    );
}

#[test]
fn complex_alpha() {
    let mut resistance_value: HashMap<&str, f32> = HashMap::new();

    /*
     * This complex one is not working: Trouvé : 2.0 Attendu : 2.4
     * circuit: ( Alfa [ Charlie Delta ( Bravo [ Echo ( Foxtrot Golf ) ] ) ] )
     * name: Alfa, r: 1
     * name: Bravo, r: 1
     * name: Charlie, r: 12
     * name: Delta, r: 4
     * name: Echo, r: 2
     * name: Foxtrot, r: 10
     * name: Golf, r: 8
     */
    resistance_value.insert(&"Alfa", 1.0);
    resistance_value.insert(&"Bravo", 1.0);
    resistance_value.insert(&"Charlie", 12.0);
    resistance_value.insert(&"Delta", 4.0);
    resistance_value.insert(&"Echo", 2.0);
    resistance_value.insert(&"Foxtrot", 10.0);
    resistance_value.insert(&"Golf", 8.0);

    assert_eq!(
        2.4,
        evaluate(
            tokenize(&"( Alfa [ Charlie Delta ( Bravo [ Echo ( Foxtrot Golf ) ] ) ] )"),
            &resistance_value
        )
    );
}

#[test]
fn complex_alef() {
    /* This one not working
     * name: Alef, r: 30
     * name: Bet, r: 20
     * name: Vet, r: 10
     * circuit: ( Alef [ ( Bet Bet Bet ) ( Vet [ ( Vet Vet ) ( Vet [ Bet Bet ] ) ] ) ] )
     *
     * Trouvé : 30.1
     * Attendu : 45.0
     */
    let mut resistance_value: HashMap<&str, f32> = HashMap::new();

    resistance_value.insert(&"Alef", 1.0);
    resistance_value.insert(&"Bet", 1.0);
    resistance_value.insert(&"Vet", 12.0);
    assert_eq!(
        45.0,
        evaluate(
            tokenize(&"( Alef [ ( Bet Bet Bet ) ( Vet [ ( Vet Vet ) ( Vet [ Bet Bet ] ) ] ) ] )"),
            &resistance_value
        )
    );
}

#[test]
fn complex_star() {
    let mut resistance_value: HashMap<&str, f32> = HashMap::new();

    /* And also this one
     * name: Star, r: 78
     * circuit: [ ( [ Star ( Star Star ) ] [ Star ( Star Star ) ] Star ) ( [ Star ( Star Star ) ] [ Star ( Star Star ) ] Star ) ]
     *
     * Trouvé : 39.0
     * Attendu : 91.0
     */
    resistance_value.insert(&"Star", 78.0);
    assert_eq!(
        91.0,
        evaluate(tokenize(&"[ ( [ Star ( Star Star ) ] [ Star ( Star Star ) ] Star ) ( [ Star ( Star Star ) ] [ Star ( Star Star ) ] Star ) ]"),
        &resistance_value));
}

fn main() {
    println!("Hello");
}
