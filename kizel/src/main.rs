use std::io::stdin;
fn main() {
    println!("escribe la operacion a realizar\nejemplo:\nsuma\nmultiplicacion");
    let mut nu: String = String::new(); /*println!("no of bytes read , {}", b1);*/
    stdin().read_line(&mut nu).unwrap();
    let xd = String::from("multiplicacion\n");
    let suma = String::from("suma\n");
    if nu == xd {
        /*println!("ahora escribe que numeros quieres multiplicar");
        let mut num1 = String::new();
        let mut num2 = String::new();
        stdin().read_line(&mut num1).unwrap();
        stdin().read_line(&mut num2).unwrap();
        let x: u32 = num1.trim().parse().unwrap();
        let y: u32 = num1.trim().parse().unwrap();*/
        multi();
    } else if nu == suma {
        farvarshchuk()
    } else {
        println!("wtf escribe multiplicacion por favor {}", xd);
    }
}
fn farvarshchuk() {
    println!("ahora escribe que numeros quieres sumar");
    let mut num1 = String::new();
    let mut num2 = String::new();
    stdin().read_line(&mut num1).unwrap();
    stdin().read_line(&mut num2).unwrap();
    let x: u32 = num1.trim().parse().unwrap();
    let y: u32 = num2.trim().parse().unwrap();
    let r: u32 = x + y;
    println!("esta es el resultadode la suma: {}", r);
    return;
}
fn multi() {
    println!("ahora escribe que numeros quieres multiplicar");
    let mut num1 = String::new();
    let mut num2 = String::new();
    stdin().read_line(&mut num1).unwrap();
    stdin().read_line(&mut num2).unwrap();
    let x: u32 = num1.trim().parse().unwrap();
    let y: u32 = num2.trim().parse().unwrap();
    let r = x * y;
    println!("este es el resultado de tu multiplicacion = {}", r);
    return;
}
