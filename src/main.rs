use std::io;

fn main() {
    println!("Fibonacci");
    println!("*********");
    println!("");
    println!("Welches Folgenglied der Fibinacci-Folge soll berchnet werden?");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Fehler beim Lesen der Zeile");
    let n: i32 = n.trim().parse().expect("Bitte gib die Zahl ein!");


    println!("{}-te Fibonacci-Zahl: {}",n,fibonacci(n));
}

fn fibonacci(n: i32) -> i32 {
    let mut a = 0;
    let mut b: i32 = 1;
    let mut c: i32 = 1;
    
    if n == 0{
        c=a
    } else {
        for i in 2..(n+1) {
            a = b;
            b = c;
            c = a+b;
        }   
    }
    c
}