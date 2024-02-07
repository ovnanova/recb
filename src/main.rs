use openssl::hash::{hash, MessageDigest};
use openssl::symm::{encrypt, Cipher};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::Command;
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        std::process::exit(1);
    }

    let image_path = &args[1];
    let image_file_name = Path::new(image_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;
    let ppm_path = "image.ppm";
    let ecb_ppm_output_path = "image.ecb.ppm";
    let ecb_output_path = format!("ecb_{}.png", image_file_name);

    // Convert the input image to PPM format using ImageMagick
    Command::new("magick")
        .args(["convert", image_path, ppm_path])
        .status()?;

    let ppm_file = File::open(ppm_path)?;
    let mut reader = BufReader::new(ppm_file);
    let mut header = String::new();

    // Parse the PPM header
    for _ in 0..3 {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        header.push_str(&line);
    }

    let mut body = Vec::new();
    reader.read_to_end(&mut body)?;

    // Hash the passphrase
    // Change the passphrase to whatever you like
    // Different passphrases will produce different results
    let passphrase = "ANNA";
    let hashed = hash(MessageDigest::sha256(), passphrase.as_bytes()).expect("Failed to hash passphrase");
    // AES-128 requires a 16-byte key, so we will use the first 16 bytes of the hash as the key
    let key = &hashed[0..16];

    // Encrypt the body with AES-128-ECB
    let cipher = Cipher::aes_128_ecb();
    let encrypted_data = encrypt(cipher, key, None, &body).expect("Encryption failed");

    // Write the encrypted PPM file
    // Write the PPM header first, then the encrypted body
    let mut ecb_ppm_output = File::create(ecb_ppm_output_path)?;
    write!(ecb_ppm_output, "{}", header)?;
    ecb_ppm_output.write_all(&encrypted_data)?;

    // Convert the encrypted PPM file back to PNG format using ImageMagick
    Command::new("magick")
        .args(["convert", &ecb_ppm_output_path, &ecb_output_path])
        .status()?;

    // Clean up the temporary PPM files
    fs::remove_file(ppm_path)?;
    fs::remove_file(ecb_ppm_output_path)?;

    println!("Your image has been ECBed!");
    Ok(())
}