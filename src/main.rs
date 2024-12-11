use openssl::sha::Sha256;

fn main() {
    let mut hasher = Sha256::new();

    hasher.update(b"European Burmese");

    let hash = hasher.finish();
    println!("{}", hex::encode(hash));
}
