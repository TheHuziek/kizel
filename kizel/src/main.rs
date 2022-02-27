use std::io::stdin;
fn main() {
    println!("escribe la operacion a realizar\nejemplo:\nsuma\nmultiplica");
    let mut nu: String = String::new(); /*println!("no of bytes read , {}", b1);*/
    stdin().read_line(&mut nu).unwrap();
    let xd = String::from("multiplicacion\n");
    let suma= String::from("suma\n");
    if nu == xd {
        /*println!("ahora escribe que numeros quieres multiplicar");
        let mut num1 = String::new();
        let mut num2 = String::new();
        stdin().read_line(&mut num1).unwrap();
        stdin().read_line(&mut num2).unwrap();
        let x: u32 = num1.trim().parse().unwrap();
        let y: u32 = num1.trim().parse().unwrap();*/
        multi();
    }else if nu==suma{
        suman()
    }else {
        println!("wtf escribe multiplicacion por favor {}", xd);
    }
}
fn suman(){

    println!("ahora escribe que numeros quieres sumar");
    let mut num1 = String::new();
    let mut num2 = String::new();
    stdin().read_line(&mut num1).unwrap();
    stdin().read_line(&mut num2).unwrap();
    let x: u32 = num1.trim().parse().unwrap();
    let y: u32 = num1.trim().parse().unwrap();
    let r= x+y;
    println!("esta es el resultadode la suma: {}",r)
}
fn multi() {

    println!("ahora escribe que numeros quieres multiplicar");
    let mut num1 = String::new();
    let mut num2 = String::new();
    stdin().read_line(&mut num1).unwrap();
    stdin().read_line(&mut num2).unwrap();
    let x: u32 = num1.trim().parse().unwrap();
    let y: u32 = num1.trim().parse().unwrap();
    let r = x * y;
    println!("este es el resultado de tu multiplicacion = {}", r);
    return;
}
