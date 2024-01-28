use std::io;
use rand::Rng;

fn main() {
    println!(" Добро пожаловать в генератор названий китайских автомобилей! ");
    let mut enter = String::new();
    loop {
        match io::stdin().read_line(&mut enter) {
            Ok(_) => {},
            Err(e) => println!("Error... - {}", e)  
        }
        println!("{} {} {}", one(), two(), three());
    }
}

fn one() -> &'static str {
    let name: [&str; 12] = ["Bao", "Zen", "Lou", "Seng", "Chan", "Ri", "Po", "Shai", "Gao", "Keng", "Ang", "Fei"];
    let mut rng = rand::thread_rng();
    let num: usize = rng.gen_range(0..12);
    name[num]
}
fn two() -> &'static str {
    let name: [&str; 12] = ["Zan", "Ren", "Zhang", "Gan", "Nan", "Shang", "Tang", "Fang", "Bang", "Ong", "Kai", "He"];
    let mut rng = rand::thread_rng();
    let mut num: usize = rng.gen_range(0..12);
    name[num]
}
fn three() -> &'static str {
    let name: [&str; 12] = ["GT-750Ti", "Sport", "Luxuri", "Elite", "Luxury", "001", "T600", "Nomad", "Sauvana", "Midi", "Super RTX", "Emgrand"];
    let mut rng = rand::thread_rng();
    let num: usize = rng.gen_range(0..12);
    name[num]
}