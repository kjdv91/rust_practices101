fn main() {
    println!("Hello, world!");
    test();
}

fn test(){
    println!("Test has called");
   let result: u64 =  add_numbers(20, 50);
   println!("The add is {}", result);

}

fn add_numbers(x: u64, y : u64) -> u64{
    return  x + y;
}