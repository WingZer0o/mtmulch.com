use chacha20poly1305::{
    ChaCha20Poly1305, Key,
    aead::{
        KeyInit,
        generic_array::GenericArray,
        stream::{DecryptorBE32, EncryptorBE32},
    },
};
use rand::RngCore;

/// Plaintext bytes per STREAM segment (4 MiB).
pub const BLOCK_SIZE: usize = 4 * 1024 * 1024;
/// ChaCha20Poly1305 nonce (12 bytes) minus StreamBE32 overhead (5 bytes: 4-byte counter + 1-byte last flag).
pub const NONCE_SIZE: usize = 7;
/// AEAD authentication tag appended to each encrypted segment.
const TAG_SIZE: usize = 16;
/// Encrypted segment size on disk: plaintext block + tag.
pub const ENC_BLOCK_SIZE: usize = BLOCK_SIZE + TAG_SIZE;

pub type BlobEncryptor = EncryptorBE32<ChaCha20Poly1305>;
pub type BlobDecryptor = DecryptorBE32<ChaCha20Poly1305>;

pub struct EncryptionKey([u8; 32]);

impl EncryptionKey {
    pub fn from_env() -> Self {
        let hex_key =
            std::env::var("BLOB_ENCRYPTION_KEY").expect("BLOB_ENCRYPTION_KEY must be set");
        let bytes = hex::decode(&hex_key).expect("BLOB_ENCRYPTION_KEY must be valid hex");
        assert!(bytes.len() == 32, "BLOB_ENCRYPTION_KEY must be 32 bytes (64 hex chars)");
        let mut key = [0u8; 32];
        key.copy_from_slice(&bytes);
        EncryptionKey(key)
    }

    /// Returns a fresh random nonce and a [`BlobEncryptor`] ready for streaming upload.
    /// The caller must prepend the nonce to the first uploaded block.
    pub fn begin_encrypt(&self) -> ([u8; NONCE_SIZE], BlobEncryptor) {
        let mut nonce = [0u8; NONCE_SIZE];
        rand::thread_rng().fill_bytes(&mut nonce);
        let enc = self.make_encryptor(&nonce);
        (nonce, enc)
    }

    /// Creates a [`BlobDecryptor`] initialised with the given nonce (must be [`NONCE_SIZE`] bytes).
    pub fn make_decryptor(&self, nonce: &[u8]) -> BlobDecryptor {
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&self.0));
        DecryptorBE32::from_aead(cipher, GenericArray::from_slice(nonce))
    }

    fn make_encryptor(&self, nonce: &[u8]) -> BlobEncryptor {
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&self.0));
        EncryptorBE32::from_aead(cipher, GenericArray::from_slice(nonce))
    }

    /// Convenience: encrypt an entire buffer in one call (used by tests / small payloads).
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, chacha20poly1305::aead::Error> {
        let (nonce, mut enc) = self.begin_encrypt();

        let n_segs = plaintext.len() / BLOCK_SIZE + 1;
        let mut out = Vec::with_capacity(NONCE_SIZE + plaintext.len() + n_segs * TAG_SIZE);
        out.extend_from_slice(&nonce);

        let chunks: Vec<&[u8]> = plaintext.chunks(BLOCK_SIZE).collect();
        match chunks.split_last() {
            None => out.extend_from_slice(&enc.encrypt_last(&[][..])?),
            Some((last, rest)) => {
                for block in rest {
                    out.extend_from_slice(&enc.encrypt_next(*block)?);
                }
                out.extend_from_slice(&enc.encrypt_last(*last)?);
            }
        }

        Ok(out)
    }

    /// Convenience: decrypt an entire buffer in one call.
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, chacha20poly1305::aead::Error> {
        if data.len() < NONCE_SIZE {
            return Err(chacha20poly1305::aead::Error);
        }
        let (nonce, ciphertext) = data.split_at(NONCE_SIZE);
        let mut dec = self.make_decryptor(nonce);
        let mut out = Vec::with_capacity(ciphertext.len().saturating_sub(TAG_SIZE));

        let chunks: Vec<&[u8]> = ciphertext.chunks(ENC_BLOCK_SIZE).collect();
        match chunks.split_last() {
            None => out.extend_from_slice(&dec.decrypt_last(&[][..])?),
            Some((last, rest)) => {
                for block in rest {
                    out.extend_from_slice(&dec.decrypt_next(*block)?);
                }
                out.extend_from_slice(&dec.decrypt_last(*last)?);
            }
        }

        Ok(out)
    }
}
