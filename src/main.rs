use sha3::{Digest, Sha3_256};

fn main() {
    // Ingresar la contraseña
    let password = "Tucontraseña";

    // Crear un objeto llaado hasher de SHA-3 de 256 bits
    let mut hasher = Sha3_256::new();

    // Calcular el hash de la contraseña
    hasher.update(password);

    // Obtener el hash
    let result = hasher.finalize();

    // Convertir el hash a una cadena hexadecimal
    let hash_hex = format!("{:x}", result);

    // Imprimir el hash generado
    println!("Contraseña generada: {}", hash_hex);
}
