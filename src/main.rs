use rand::distributions::Uniform;
use rand::{Rng, distributions::Alphanumeric};

fn gerar_senha(tamanho: usize) -> String {
    // Definindo os caracteres possíveis
    let maiusculas = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let minusculas = "abcdefghijklmnopqrstuvwxyz";
    let numeros = "0123456789";
    let simbolos = "!@#$%^&*()-_=+[]{}|;:,.<>?";

    let todos_caracteres = format!("{}{}{}{}", maiusculas, minusculas, numeros, simbolos);

    let mut rng = rand::thread_rng();
    let char_dist = Uniform::from(0..todos_caracteres.len());

    // Construindo a senha escolhendo caracteres aleatórios
    let senha: String = (0..tamanho)
        .map(|_| {
            let idx = rng.sample(char_dist);
            todos_caracteres.chars().nth(idx).unwrap()
        })
        .collect();

    senha
}

fn main() {
    let tamanho = 16;
    let senha = gerar_senha(tamanho);
    println!("Senha gerada: {}", senha);
}
