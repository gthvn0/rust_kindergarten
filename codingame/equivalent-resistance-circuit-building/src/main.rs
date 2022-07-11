use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct Resistance {
    kind: RType,
    depth: i32,
    value: f32,
}

#[derive(Debug, PartialEq)]
enum RType {
    Serie,
    Parallel,
}

fn start_serie(s: &mut Vec<Resistance>) {
    match s.pop() {
        None => s.push(Resistance {
            kind: RType::Serie,
            depth: 1,
            value: 0.0,
        }),
        Some(v) => {
            if v.kind == RType::Serie {
                s.push(Resistance {
                    kind: RType::Serie,
                    depth: v.depth + 1,
                    value: v.value,
                });
            } else {
                s.push(v);
                s.push(Resistance {
                    kind: RType::Serie,
                    depth: 1,
                    value: 0.0,
                });
            }
        }
    }
}

fn start_parallel(s: &mut Vec<Resistance>) {
    match s.pop() {
        None => s.push(Resistance {
            kind: RType::Parallel,
            depth: 1,
            value: 0.0,
        }),
        Some(v) => {
            if v.kind == RType::Parallel {
                s.push(Resistance {
                    kind: RType::Parallel,
                    depth: v.depth + 1,
                    value: v.value,
                });
            } else {
                s.push(v);
                s.push(Resistance {
                    kind: RType::Parallel,
                    depth: 1,
                    value: 0.0,
                });
            }
        }
    }
}

fn end_serie(s: &mut Vec<Resistance>) {
    match s.pop() {
        None => panic!("Something goes wrong"),
        Some(v) => {
            eprintln!("v == {:?}", v);
            if v.kind == RType::Serie {
                if v.depth == 1 {
                    // On doit basculer la valeur dans l'element du dessous
                    match s.pop() {
                        None => s.push(Resistance {
                            kind: v.kind,
                            depth: 0,
                            value: v.value,
                        }),
                        Some(down) => {
                            let new_value: f32 = if down.kind == RType::Serie {
                                down.value + v.value
                            } else {
                                down.value + (1.0 / v.value)
                            };
                            s.push(Resistance {
                                kind: down.kind,
                                depth: down.depth,
                                value: new_value,
                            });
                        }
                    }
                } else if v.depth > 1 {
                    s.push(Resistance {
                        kind: RType::Serie,
                        depth: v.depth - 1,
                        value: v.value,
                    });
                } else {
                    panic!("v.depth shouldn't be < 1");
                }
            } else {
                panic!("Top element should be serie");
            }
        }
    }
}

fn end_parallel(s: &mut Vec<Resistance>) {
    match s.pop() {
        None => panic!("Something goes wrong"),
        Some(v) => {
            eprintln!("v == {:?}", v);
            if v.kind == RType::Parallel {
                if v.depth == 1 {
                    // On doit basculer la valeur dans l'element du dessous
                    match s.pop() {
                        None => s.push(Resistance {
                            kind: v.kind,
                            depth: 0,
                            value: 1.0 / v.value,
                        }),
                        Some(down) => {
                            let new_value: f32 = if down.kind == RType::Serie {
                                down.value + v.value
                            } else {
                                down.value + (1.0 / v.value)
                            };
                            s.push(Resistance {
                                kind: down.kind,
                                depth: down.depth,
                                value: new_value,
                            });
                        }
                    }
                } else if v.depth > 1 {
                    s.push(Resistance {
                        kind: RType::Parallel,
                        depth: v.depth - 1,
                        value: v.value,
                    });
                } else {
                    panic!("v.depth shouldn't be < 1");
                }
            } else {
                panic!("Top element should be parallel");
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
            Token::Po => start_serie(&mut stack),
            Token::Pc => end_serie(&mut stack),
            Token::Bo => start_parallel(&mut stack),
            Token::Bc => end_parallel(&mut stack),
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
                            depth: elmt.depth,
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

fn main() {
    let mut resistance_value: HashMap<&str, f32> = HashMap::new();

    resistance_value.insert(&"A", 24.0);
    resistance_value.insert(&"B", 8.0);
    resistance_value.insert(&"C", 48.0);
    resistance_value.insert(&"D", 20.0);
    resistance_value.insert(&"E", 25.0);

    println!(
        " >>>> {} \n\n",
        evaluate(tokenize(&"( A B )"), &resistance_value)
    );
    println!(
        " >>>> {} \n\n",
        evaluate(tokenize(&"[ D E ]"), &resistance_value)
    );
    println!(
        " >>>> {} \n\n",
        evaluate(tokenize(&"[ ( A B ) [ C A ] ]"), &resistance_value)
    );

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

    let vtoks: Vec<Token> =
        tokenize(&"( Alfa [ Charlie Delta ( Bravo [ Echo ( Foxtrot Golf ) ] ) ] )");
    println!(" >>>> {}\n\n", evaluate(vtoks, &resistance_value));

    /* This one not working
     * name: Alef, r: 30
     * name: Bet, r: 20
     * name: Vet, r: 10
     * circuit: ( Alef [ ( Bet Bet Bet ) ( Vet [ ( Vet Vet ) ( Vet [ Bet Bet ] ) ] ) ] )
     *
     * Trouvé : 30.1
     * Attendu : 45.0
     */
    resistance_value.insert(&"Alef", 1.0);
    resistance_value.insert(&"Bet", 1.0);
    resistance_value.insert(&"Vet", 12.0);
    println!(
        " >>>> {}\n\n",
        evaluate(
            tokenize(&"( Alef [ ( Bet Bet Bet ) ( Vet [ ( Vet Vet ) ( Vet [ Bet Bet ] ) ] ) ] )"),
            &resistance_value
        )
    );

    /* And also this one
     * name: Star, r: 78
     * circuit: [ ( [ Star ( Star Star ) ] [ Star ( Star Star ) ] Star ) ( [ Star ( Star Star ) ] [ Star ( Star Star ) ] Star ) ]
     *
     * Trouvé : 39.0
     * Attendu : 91.0
     */
    resistance_value.insert(&"Star", 78.0);
    println!(" >>>> {}\n\n", evaluate(tokenize(&"[ ( [ Star ( Star Star ) ] [ Star ( Star Star ) ] Star ) ( [ Star ( Star Star ) ] [ Star ( Star Star ) ] Star ) ]"), &resistance_value));
}
