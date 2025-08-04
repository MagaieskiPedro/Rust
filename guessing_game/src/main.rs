use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	let mut tentativa = String::new();
	let numero_secreto = ran::thread_rng().gen_range(1..=100)
	println!("Descubra o numero:");	    

	io::stdin()
		.read_line(&mut tentativa)
		.expect("Erro ao ler dado");
	println!("Você chutou: {tentativa}");

	match tentativa.cmp(&numero_secreto) {
		Ordering::Less => println!("Muito baixo"),
		Ordering::Greater => println!("Muito alto"),
		Ordering::Equal => println!("Acertou"),
	
//	let x = 5;
//	let y = 10;
//	println!("x = {x} and y + 2 = {}",y + 2);
}
