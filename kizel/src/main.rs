use std::io::stdin;
fn main(){
    println!("escribe multiplicacion: ");
    let mut nu:String=String::new(); /*
    println!("no of bytes read , {}", b1);*/
    let _b2 =stdin().read_line(&mut nu).unwrap();
    let xd:bool=nu.eq("multiplicacion");

    if xd == true{

     println!("ahora escribe que numeros quieres multiplicar");
    let mut num1= String::new();
    let mut num2= String::new();

    let _b3= stdin().read_line(&mut num1).unwrap();
    let _b4 =stdin().read_line(&mut num2).unwrap();
    let x:u32 = num1.trim().parse().unwrap();
    let y:u32 = num1.trim().parse().unwrap();
        multi(x,y);
    } else {    
            println!("wtf escribe multiplicacion por favor {}",xd);
        }
 }
 fn multi(x:u32,y:u32){
    let r=x*y;
    println!("este es el resultado de tu multiplicacion{}",r);
    return;
 }