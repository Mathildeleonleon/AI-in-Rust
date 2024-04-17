/* auto-exports start exclusions=[AesKey, xor_bytes] */
mod aes;
mod another_rot13;
mod baconian_cipher;
mod base64;
mod blake2b;
mod caesar;
mod chacha;
mod diffie_hellman;
mod hashing_traits;
mod kerninghan;
mod morse_code;
mod polybius;
mod rail_fence;
mod rot13;
mod salsa;
mod sha256;
mod tea;
mod theoretical_rot13;
mod transposition;
mod vigenere;
mod xor;

pub use aes::{
	aes_encrypt,
	aes_decrypt
};
pub use another_rot13::another_rot13;
pub use baconian_cipher::{
	baconian_encode,
	baconian_decode
};
pub use base64::{
	base64_encode,
	base64_decode
};
pub use blake2b::blake2b;
pub use caesar::caesar;
pub use chacha::chacha20;
pub use diffie_hellman::DiffieHellman;
pub use hashing_traits::{
	Hasher,
	HMAC
};
pub use kerninghan::kerninghan;
pub use morse_code::{
	encode,
	decode
};
pub use polybius::{
	encode_ascii,
	decode_ascii
};
pub use rail_fence::{
	rail_fence_encrypt,
	rail_fence_decrypt
};
pub use rot13::rot13;
pub use salsa::salsa20;
pub use sha256::SHA256;
pub use tea::{
	tea_encrypt,
	tea_decrypt
};
pub use theoretical_rot13::theoretical_rot13;
pub use transposition::transposition;
pub use vigenere::vigenere;
pub use xor::xor;
/* auto-exports end */

mod sha3;
pub use sha3::{sha3_224, sha3_256, sha3_384, sha3_512};
