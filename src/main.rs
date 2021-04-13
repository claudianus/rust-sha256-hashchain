fn main() {
    let hashchain_seed = "aaaaaaaaaaaa";
    let hashchain_length = 10000000;
    let mut previous_hash:String = hashchain_seed.to_string();

    for _ in 0..hashchain_length {
        previous_hash = sha256::digest(previous_hash);
        // print!("{}\n", previous_hash)
    }

    print!("done")
}