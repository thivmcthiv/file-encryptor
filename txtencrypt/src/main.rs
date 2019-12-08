use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
use std::io;
mod pull;

use std::io::{ErrorKind};

use aes_gcm::Aes256Gcm;
use aead::{Aead, NewAead, generic_array::GenericArray};

extern crate rand;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


fn main() {
    println!("Do you want to decrypt or encrypt a file? (type d or e) "); 
    let mut e_or_d = String::new();
    io::stdin().read_line(&mut e_or_d) .expect("Failed to read line");
    let ed = pull::EncryptOrDecrypt::new(e_or_d.trim().to_string());
    match ed {
        pull::ed::e => {

            //asks what file you want to decrypt
            println!("What is the name of the file you want to encrypt?");
            let mut filename = String::new();
            io::stdin().read_line(&mut filename).expect("failed to read line");

            //asks what the output file should be
            println!("Select a name for the encrypted output file:");
            let mut outputfile = String::new();
            io::stdin().read_line(&mut outputfile).expect("failed to read line");

            println!("Do you want to generate a random key of type in your own? (type r for random, w to type it in yourself)");
            let mut randorwrite = String::new();
            io::stdin().read_line(&mut randorwrite).expect("failed to read line");
            let r_or_w: pull::rw = pull::RandomOrType::new(randorwrite.trim().to_string());

            let mut key = String::new();
            let mut nonce = String::new();
            match r_or_w {
                pull::rw::r => {
                    key = rand_key();
                }
                pull::rw::w => {
                    println!("Type in the key that you want to encrypt the file with.");
                    io::stdin().read_line(&mut key).expect("failed to read line");
                    key = pull::Key::new(key);
                }
            }

            println!("Do you want to generate a random nonce (recomended), or write one in yourself?");
            let mut randorwrite = String::new();
            io::stdin().read_line(&mut randorwrite).expect("failed to read line");
            let r_or_w = pull::RandomOrType::new(randorwrite.trim().to_string());
            match r_or_w {
                pull::rw::r => {
                    nonce = rand_nonce();
                }
                pull::rw::w => {
                    println!("Type in the nonce that you want to encrypt the file with.");
                    io::stdin().read_line(&mut nonce).expect("failed to read line");
                    nonce = pull::Nonce::new(nonce);
                }
            }

            //////////////////////////////////////////////////////////////////////
            //parsing the variables to the function encrypt()/////////////////////
            //////////////////////////////////////////////////////////////////////
            let mut f = match File::open(filename.trim()) {
                Ok(file) => {
                    file
                }
                Err(e) => {
                    match e.kind() {
                        ErrorKind::PermissionDenied => { panic!("permision denied to open {}", filename); }
                        ErrorKind::NotFound => { 
                            println!("Creating file {}...", filename);
                            match File::create(filename.trim()) {
                                Ok(file) => {
                                    println!("Go ahead stop this program if you want to put data inside of {} to encrypt", filename);
                                    file
                                }
                                Err(e) => {
                                    panic!("failed to make the file {} (hint: you could manually make it). Error code: {}",filename, e);
                                }
                            }
                        }
                        _ => {
                            panic!("failed to open the file {}", filename);
                        }
                    
                    }
                }
            };
        
            let mut encryptme: Vec<u8> = Vec::new();
            f.read_to_end(&mut encryptme).expect("Could not read the file encryptme.txt to a string. Are you root?");  //dumps the contents of encryptme.txt into the string encryptme
        
            encrypt(key.clone().trim().to_string(), nonce.trim().clone().to_string(), encryptme.clone(), outputfile.clone().trim().to_string());
            ///////////////////////////////////////////////////////////////////
            ///////////////////////////////////////////////////////////////////
        
            println!("Remember to write down the key and nonce somewhere safe, or you won't be able to decrypt the file encrypted.txt!!!");

        }
        pull::ed::d => {
            println!("Type in the key: ");
            let mut key = String::new();
            io::stdin().read_line(&mut key).expect("failed to read line");

            println!("Type in the nonce: ");
            let mut nonce = String::new();
            io::stdin().read_line(&mut nonce).expect("failed to read line");

            println!("Type in the name of the file you want to decrypt.");
            let mut filename = String::new();
            io::stdin().read_line(&mut filename).expect("failed to read line");

            println!("Type in the name of the file you want the decrypted bytes to be written to.  (choose the same file extension):");
            let mut outputfile = String::new();
            io::stdin().read_line(&mut outputfile).expect("failed to read line");

            decrypt(key.trim().to_string(), nonce.trim().to_string(), filename.trim().to_string(),outputfile.trim().to_string());
        }
    }
}


fn encrypt(key: String, nonce: String, plaintext: Vec<u8>, outputfile: String) {
    let key = GenericArray::clone_from_slice(key.as_bytes());
    let aead = Aes256Gcm::new(key);

    let snonce = GenericArray::from_slice(nonce.as_bytes());

    let encryptthis: Vec<u8> = aead.encrypt(snonce, plaintext.as_ref()).expect("failed to encrypt text");
    
    let mut f = File::create(outputfile.trim()).expect("failed to create the file encrypted.txt");

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
    let mut f = match OpenOptions::new().read(true).write(true).append(true).open("keys.txt") {
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
                            panic!("failed to make the file keys.txt (hint: you could manually make it). Error code: {}", e);
                        }
                    }
                }
                _ => {
                    panic!("failed to open the file keys.txt");
                }  
            }
        }
    };

    f.write(&format!("  Your nonce is {}", rand_string).as_bytes()[..]).expect("could not write nonce to keys.txt");
    println!("your nonce has been writen to keys.txt");
    f.sync_all().expect("could not sync_all for nonce");
    rand_string
}


#[allow(unused_mut,unused_variables)]
fn rand_key() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect();

    println!("your key is {}", rand_string);

    let mut f = match OpenOptions::new().read(true).write(true).append(true).open("keys.txt") {
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

    f.write(&format!("  your key is {}", rand_string).as_bytes()[..]).expect("failed to write your key to keys.txt");
    println!("Your key has been written to keys.txt.");
    f.sync_all().expect("could not sync_all for key");


    rand_string
}




#[allow(dead_code)]
#[allow(unused_variables, unused_mut)]
fn decrypt(
    key: String,
    nonce: String,
    filename: String,
    outputfile: String
) {
    let thekey = GenericArray::clone_from_slice(key.trim().as_bytes());
    let thenonce = GenericArray::from_slice(nonce.trim().as_bytes().as_ref());
    let mut cyphertext: Vec<u8> = Vec::new();
    let aead = Aes256Gcm::new(thekey);
    let mut f = match File::open(&filename[..].trim()) {
        Ok(file) => {
            file
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => {
                    panic!("permission denied on opening the encrypted file");
                }
                ErrorKind::NotFound => {
                    panic!("your encrypted file was not found!!");
                }
                _ => {
                    panic!("failed to open your encrypted file");
                }
            }
        }
    };
    f.read_to_end(&mut cyphertext).expect("failed to read your encrypted file to a string");
    println!("writing the decrypted contents of {} to {}", filename, outputfile);
    let mut f2 = match OpenOptions::new().read(true).write(true).open(outputfile.trim()) {
        Ok(file) => {
            file
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    println!("The file {} was not found!", outputfile);
                    println!("Creating {}...",outputfile);
                    match File::create(outputfile.trim()) {
                        Ok(file) => {
                            file
                        }
                        Err(e) => {
                            panic!("failed to create {}", outputfile);
                        }
                    }
                }
                ErrorKind::PermissionDenied => {
                    panic!("Permission denied to open {}", outputfile);
                }
                _ => {
                    panic!("Failed to open {}", outputfile);
                }
            }
        }
    };
    let decryptedtext: Vec<u8> = aead.decrypt(thenonce, cyphertext.as_ref()).expect("Couldn't decrypt file!");
    match f2.write_all(&decryptedtext[..]) {
        Ok(file) => {
            file
        }
        Err(e) => {
            println!("wasn't able to write the decrypted text to {}: {}",outputfile, e);
            println!("Here is your decrypted bytes or text: \n {:?}", &decryptedtext);
        }
    }
}