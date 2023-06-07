pub fn generate(prompt: &String, n: u8, size: &String, dir: &String) {
    println!("Executing 'open-ai dall-e generate --n {1} --size {2}x{2} --dir {3} \"{0}\"'", prompt, n, size, dir);
}
