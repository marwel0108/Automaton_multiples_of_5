use std::io;

#[derive(PartialEq, Eq)]
enum Q {
    Q0,
    Q1,
    Q2,
    Q3,
    Q4,
}

enum Alphabet {
    Zero,
    One
}

fn transition(state: Q, letter: Alphabet) -> Q {
    match state {
        Q::Q0 => match letter {
            Alphabet::Zero => Q::Q0,
            Alphabet::One => Q::Q1
        },
        Q::Q1 => match letter {
            Alphabet::Zero => Q::Q2, 
            Alphabet::One => Q::Q3
        },
        Q::Q2 => match letter {
            Alphabet::Zero => Q::Q4,
            Alphabet::One => Q::Q0
        }
        Q::Q3 => match letter {
            Alphabet::Zero => Q::Q1,
            Alphabet::One => Q::Q2
        }
        Q::Q4 => match letter {
            Alphabet::Zero => Q::Q3,
            Alphabet::One => Q::Q4
        }
    }
}

fn main() {
    
    let mut w = String::new();

    let mut state: Q = Q::Q0;

    io::stdin().read_line(&mut w).expect("Error reading the input.");

    for a in w.trim().chars() {
        
        match a {
            '0' => state = transition(state, Alphabet::Zero),
            '1' => state = transition(state, Alphabet::One),
            unknown => {
                println!("Uknown character: {}.", unknown);
                state = Q::Q1;
            }
        }
    }

    if state == Q::Q0 {
        println!("The string was accepted.");
    } else {
        println!("The string was rejected.");
    }
}
