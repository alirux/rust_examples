fn main() {
    println!("Hello, world!");
    println!("Sono qui!");

    let test = String::from("ciao!");
    println!("La stringa ha contenuto: {}", test);
    println!("Funzione: {}", test_function());
    funzione_parametri(String::from("stringa testo"), 234);
    funzione_parametri("Stringa testo2".to_string(), 234);
    funzione_condizionale(0);
    funzione_condizionale(-30);
    funzione_condizionale(20);
    let risultato_loop : i32 = funzione_loop();
    println!("risultato loop {}", risultato_loop);
}

fn test_function() -> u32 {
    5
}

fn funzione_parametri(p1: String, p2: u32) {
    println!("Risultato: {} numero: {}", p1, p2)
}

fn funzione_condizionale(p: i32) {
    if p == 0 {
        println!("p zero {}", p);
    } else if p > 0 {
        println!("p maggiore di zero {}", p);
    } else {
        println!("p minore di zero {}", p);
    }
}

// conta fino a 10, moltiplica per due e restituisce 20
fn funzione_loop() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // Ultima espressione diventa risultato della funzione
    result
}
