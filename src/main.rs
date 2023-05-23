/*fn main() {

    const PI: f32 = 3.1416;
    let x = 3;
    let c = 'a';
    println!("El valor de x es: {}", x);

    //let mult = PI * 5;

   
    println!("El valor de Y es: {}", c);
    println!("El valor de PI es: {}", PI);

    let a = [1.5, 2.7, 3.6, 4.3, 5.7];

    let primero = a[0];
    let tercero = a[2];

    print!("El primer elemento de la lista es: {}\n", primero);
    print!("El tercer elemento de la lista es: {}", tercero);
}

use std::io;
fn main(){
    let a = [1, 2, 3, 4, 5];

    println!("Por favor ingresa un arreglo index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Fallo en leer la linea");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index ingresado no tiene un numero");

    let element = a[index];

    println!("El valor del elemento en index {index} is: {element}")
}*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Por favor ingresa dos números enteros como argumentos.");
        std::process::exit(1);
    }
    let num1: i32 = args[1].parse().unwrap();
    let num2: i32 = args[2].parse().unwrap();
    let sum = num1 + num2;
    println!("La suma es: {}", sum);
}
