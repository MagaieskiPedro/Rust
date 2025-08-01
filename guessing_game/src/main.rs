use std::io;

fn main() {
	println!("Descubra o numero:");	    
	let mut tentativa = String::new();

	io::stdin()
		.read_line(&mut tentativa)
		.expect("Erro ao ler dado");
	println!("Você chutou: {tentativa}");
	
	let x = 5;
	let y = 10;
	println!("x = {x} and y + 2 = {}",y + 2);
}
