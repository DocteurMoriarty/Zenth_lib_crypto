
mod zenth_cipher;
use crate::zenth_cipher::symmetric::aes::aes_gcm::Aes256GcmEncryption;
fn main() {
    println!("Hello, world!");
    // 1. Préparer les entrées
    let key: [u8; 32] = [...];             // Clé AES-256
    let nonce: [u8; 12] = [...];           // Nonce unique (12 octets standard)
    let aad: &[u8] = b"header-data";       // Données authentifiées mais non chiffrées
    let mut data: Vec<u8> = b"secret".to_vec(); // Données à chiffrer (modifiable)

    // 2. Créer un contexte de chiffrement
    let mut enc = Aes256GcmEncryption::new(&key, &nonce, aad)?;

    // 3. Chiffrer les données en place
    enc.encrypt(&mut data);

    // 4. Calculer le tag d’authentification
    let tag = enc.compute_tag();

}
