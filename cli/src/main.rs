use some_lib::{get_msg, get_random_u32};

fn main() {
    let msg = get_msg();
    let rando = get_random_u32();
    println!("msg: {}", msg);
    println!("rando: {}", rando);
}
