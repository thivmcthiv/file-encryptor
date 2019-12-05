# file-encryptor
A file encryptor using rust. After runing this program, it askes the user for the name of a file they want to encrypt, then reads the bytes from the file then encrypts the read bytes using an AES-GCM algorithm developed by the RustCrypto group, then creates a file (lets you name it) and writes the encrypted bytes to it. This program also generates a random key as well as nonce (iv), and writes the key and nonce to another file (keys.txt). 

  You can also decrypt a file that was encrypted with this algorithm. Simply run the program, and follow the steps.
  
  WARNING: The aes-gcm crate has never officially been tested by professionals. 
  WARNING: Don't loose your key and nonce.
