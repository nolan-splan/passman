# Passman

Passman is a terminal-based password manager written in Rust. The idea behind the project was to provide a lightweight, secure, and local password manager that can be used on the command line.

## Why Local?
I believe that using a cloud-based password manager is overkill and unnecessary in most cases. I believe that by storing passwords with a cloud-based service, you are placing yourself at a greater risk of having your passwords compromised.

My top reasons for using a local password manager are:
1. **Privacy**: By storing your passwords locally, you are not sharing your passwords or user data with a third-party service that could be compromised at any time.
2. **Security**: By storing your passwords locally, you are in complete control of your own data and can ensure that your passwords are stored securely.
3. **Offline Access**: By storing your passwords locally, you are able access your passwords even when you're offline. You don't need an internet connection, often required by cloud-based 3rd party services, to access your own data and passwords.
5. **Open Source**: Passman is open source, which means that you can review the code and ensure that your passwords are stored securely.
6. **Cross-Platform**: Passman is written in Rust, which means that it can run on any platform that supports Rust. This means that you can use Passman on Windows, macOS, and Linux.

## Encryption Methods
Passman uses the powerful and efficient Advanced Encryption Standard with 256-bit key size in Galois/Counter Mode. This is commonly referred to AES-256-GCM. 

### AES-256
AES is a symmetric encryption algorithm, which means that the same key is used to encrypt and decrypt data. Passman uses the user's provided master password to generate a secure 256-bit key using the PBKDF2 key derivation function.

### PBKDF2
PBKDF2 (Password-Based Key Derivation Function 2) applies a pseudorandom function, such as hash-based message authentication code (HMAC), to the input password or passphrase along with a salt value and repeats the process many times to produce a derived key, which can then be used as a cryptographic key in subsequent operations

### GCM (Galois/Counter Mode) 
GCM is a mode of operation for block ciphers like AES that provides both encryption and authentication in a single operation. This mode ensures:
- **Confidentiality**: Data remains a secret, and only authorized parties with the decryption key can access it.
- **Integrity and Authenticity**: GCM produces an authentication tag that confirms that the data has not been tampered with. It verifies the authenticity of the encrypted data.

### How it Works
- **Counter Mode (CTR)**: GCM uses a variant of counter (CTR) mode, where a unique number (nonce) is combined with a counter to create a unique "initialization vector" (IV) for each block of data. This transforms the AES block cipher into a stream cipher and allows for faster, parallel processing.
- **Galois Field Multiplication**: For authentication, GCM uses operations from Galois fields (special algebraic structures) to produce a secure authentication tag.

### Summary
AES-256-GCM is a highly secure and efficient method for encrypting and authenticating data, combining AES encryption with the added security of Galois/Counter Mode to provide robust data protection.

### Dependencies
Passman utilizes the following crates to encrypt and decrypt user password data:
- [aes](https://crates.io/crates/aes)
- [aes-gcm](https://crates.io/crates/aes-gcm)
- [pbkdf2](https://crates.io/crates/pbkdf2)
- [rand](https://crates.io/crates/rand)
- [sha2](https://crates.io/crates/sha2)

## Installation
To install Passman, you can clone the repository and build the project using Cargo.

```bash
git clone git@github.com:nolan-splan/passman.git

cd passman

cargo build --release
```

## Configuration
Passman needs a place to store your encrypted password data. By default, Passman will look for this file in the user's config directory: `~/.config/passman/passwords.bin`. If this file does not exist, Passman will create it for you when you first run the program. You can override this default location by setting the `PASSMAN_FILE` environment variable. 

You **SHOULD NOT** manually create the passwords.bin file! Passman will create this file for you and will manage the encryption and decryption of your password data based on the master password that you provide.

## Usage
Passman exposes the following commands to interact with the password manager:
```bash
passman add <name> <password>     # Add a new password entry
passman get <name>                # Get a password entry
passman list                      # List all password entries
passman remove <name>             # Remove a password entry
passman update <name> <password>  # Update a password entry
```

**Note:** Each command will prompt you to enter your master password to encrypt and/or decrypt the password data.

## Contributing
Please fork the repository and create a pull request to the master branch with your changes. I am open to any suggestions or improvements that you may have.

## License

