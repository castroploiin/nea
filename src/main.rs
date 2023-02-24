fn main() {
    println!("{:?}", gen_fibonacci_list(11))
}

fn gen_fibonacci_list(limit: u32) -> Vec<u32> {
   let mut list: Vec<u32> = Vec::new();
   for index in 0..limit+1 {
        list.push(gen_fibonacci_number(index))
   }

   list
}

fn gen_fibonacci_number(index: u32) -> u32 {
    match index {
        0 => return 1,
        1 => return 1,
        _ => gen_fibonacci_number(index - 1) + gen_fibonacci_number(index - 2)
    }
}