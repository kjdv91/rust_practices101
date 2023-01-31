
use std::io;
fn main() {
    let nombre: &str = "Kevin";
    let mut edad: u8 = 30;
    edad = edad + 1;
    let tup: (i32, bool, char) = (10,false, 'k');
    //constantes
    let MY_CONSTANT: u8 = 4;
    
    println!("Hello, soy {} y tengo {} años {:#?}, my constants {} \n", nombre, edad, tup, MY_CONSTANT);
    let mut inp = String::new();
   
    io::stdin().read_line(&mut inp).expect("Failed to read line");
   
    println!("input the text {}", inp);
    data_types()
}


fn data_types(){
    //tuplas 
    let tup : (u8, bool, &str) = (1,true,"Hi");
    //arrays
    let aar1: [i32;5]   = [1,2,3,4,5];
    let mut inp = String::new();
   
    let mascotas:[&str;4] = ["Perro", "Gato", "Caballo", "Chancho"];

    //crea una matriz de 5 elemmentos numeros 0
    let num= [0; 5];

    //vector puedo agregar y borrar elementos
    let mut numbers :Vec<i32> = vec![2,4,15,98];
    
    

    println!("Las mascotas son,{:#?} " ,mascotas );
    println!("Los numeors son {:#?}", num);
    println!("La segunda mascota es {}", mascotas[1]);
    println!("El vector de numero es {:#?}",numbers );
    numbers.push(58);
    println!("El vector agrego un num es {:#?}", numbers);

    let mut mappng:HashMap<&str, &str>  = HashMap::new();
    mappng.insert("name", "Kevin");
    mappng.insert("age", "31");
    mappng.insert("sexo", "Maculino");
    println!("Mapa hash {:#?}", mappng);
    println!("Nombre de la persona {:#?}", mappng.get.name);


    println!("Is my tuple {:#?} my array {:#?} ", tup, aar1);

}