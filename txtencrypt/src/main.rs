use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::io::{ErrorKind};

use aes_gcm::Aes256Gcm;
use aead::{Aead, NewAead, generic_array::GenericArray};

extern crate rand;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {

    let mut f = match File::open("encryptme.txt") {
        Ok(file) => {
            file
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => { panic!("permision denied to open encryptme.txt, please");}
                ErrorKind::NotFound => { 
                    println!("Creating file encryptme.txt...");
                    match File::create("encryptme.txt") {
                        Ok(file) => {
                            println!("Go ahead stop this program if you want to put text inside of encryptme.txt to encrypt");
                            file
                        }
                        Err(e) => {
                            panic!("failed to make the file encryptme.txt (hint: you could manually make it). Error code: {}", e);
                        }
                    }
                }
                _ => {
                    panic!("failed to open the file encryptme.txt");
                }
            
            }
        }
    };

    let key = rand_key();
    let nonce = rand_nonce();
    let mut encryptme = String::new();
    f.read_to_string(&mut encryptme).expect("Could not read the file encryptme.txt to a string. Are you root?");  //dumps the contents of encryptme.txt into the string encryptme

    encrypt(key.clone(), nonce.clone(), encryptme.clone());

    println!("Remember to write down the key and nonce somewhere safe, or you won't be able to decrypt the file encrypted.txt!!!");
    //println!("Your keys and nonces have been written to keys.txt. Don't lose them.");

    //println!("Do you want to decrypt a file?")

    decrypt(key.clone(), nonce.clone());
}


fn encrypt(key: String, nonce: String, plaintext: String) {
    let key = GenericArray::clone_from_slice(key.as_bytes());
    let aead = Aes256Gcm::new(key);

    let snonce = GenericArray::from_slice(nonce.as_bytes());

    let encryptthis: Vec<u8> = aead.encrypt(snonce, plaintext.as_bytes().as_ref()).expect("failed to encrypt text");
    
    let mut f = File::create("encrypted.txt").expect("failed to create the file encrypted.txt");

    f.write_all(&encryptthis[..]).expect("failed to write to the file encrypted.txt");

}


#[allow(unused_mut,unused_variables)]
fn rand_nonce() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .collect();

    println!("your nonce is {}", rand_string);

    //writes the key to keys.txt
    let mut f = match File::open("keys.txt") {
        Ok(file) => {
            file
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => { panic!("permision denied to open keys.txt");}
                ErrorKind::NotFound => { 
                    println!("Creating file keys.txt...");
                    match File::create("keys.txt") {
                        Ok(file) => {
                            file
                        }
                        Err(e) => {
                            panic!("failed to make the file encryptme.txt (hint: you could manually make it). Error code: {}", e);
                        }
                    }
                }
                _ => {
                    panic!("failed to open the file encryptme.txt");
                }  
            }
        }
    };

    //f.write(&format!("Your nonce is {}", rand_string).as_bytes()[..]).expect("could not write nonce to keys.txt");

    
    rand_string
}


#[allow(unused_mut,unused_variables)]
fn rand_key() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect();

    println!("your key is {}", rand_string);

    let mut f = match File::open("keys.txt") {
        Ok(file) => {
            file
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => { panic!("permision denied to open keys.txt");}
                ErrorKind::NotFound => { 
                    println!("Creating file keys.txt...");
                    match File::create("keys.txt") {
                        Ok(file) => {
                            file
                        }
                        Err(e) => {
                            panic!("failed to make the file encryptme.txt (hint: you could manually make it). Error code: {}", e);
                        }
                    }
                }
                _ => {
                    panic!("failed to open the file encryptme.txt: {}", e);
                }  
            }
        }
    };

    //f.write(&format!("your key is {}", rand_string).as_bytes()[..]).expect("failed to write your key to keys.txt");


    rand_string
}




#[allow(dead_code)]
#[allow(unused_variables, unused_mut)]
fn decrypt(
    key: String,
    nonce: String,
) {
    let thekey = GenericArray::clone_from_slice(key.as_bytes());
    let thenonce = GenericArray::from_slice(nonce.as_bytes().as_ref());
    let mut cyphertext = String::new();
    let aead = Aes256Gcm::new(thekey);
    let mut f = match File::open("encrypted.txt") {
        Ok(file) => {
            file
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => {
                    panic!("Permission denied for opening encrypted.txt.");
                }
                ErrorKind::NotFound => {
                    panic!("encrypted.txt not found!!");
                }
                _ => {
                    panic!("failed to open encrypted.txt");
                }
            }
        }
    };

    f.read_to_string(&mut cyphertext).expect("failed to read encrypted.txt to a string");


    let mut f2 = match File::open("decrypted.txt") {
        Ok(file) => {
            file
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    println!("The file decrypted.txt was not found!");
                    println!("Treating decrypted.txt...");
                    match File::create("decrypted.txt") {
                        Ok(file) => {
                            file
                        }
                        Err(e) => {
                            panic!("failed to create decrypted.txt");
                        }
                    }
                }
                ErrorKind::PermissionDenied => {
                    panic!("Permission denied to open decrypted.txt");
                }
                _ => {
                    panic!("Failed to open decrypted.txt");
                }
            }
        }
    };

    let decryptedtext: Vec<u8> = aead.decrypt(thenonce, cyphertext.as_bytes().as_ref()).expect("Couldn't decrypt file!");

    f2.write_all(&decryptedtext[..]).expect("couldn't write the decrypted text to decrypted.txt");

}