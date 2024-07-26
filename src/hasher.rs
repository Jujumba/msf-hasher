use crate::Algorithm;

use crypto::{
    digest::Digest,
    md5::Md5,
    sha1::Sha1,
    sha2::{Sha256, Sha512},
};

/// A small wrapper around hashing functions
pub struct Hasher {
    digest: Box<dyn Digest>,
}

impl Hasher {
    /// Constructs `Hasher` from the given [`Algorithm`]
    pub fn from_algorithm(algorithm: Algorithm) -> Self {
        let digest: Box<dyn Digest> = match algorithm {
            Algorithm::Sha1 => Box::new(Sha1::new()),
            Algorithm::Sha256 => Box::new(Sha256::new()),
            Algorithm::Sha512 => Box::new(Sha512::new()),
            Algorithm::Md5 => Box::new(Md5::new()),
        };
        Self { digest }
    }

    /// Hashes the given bytes
    pub fn hash<A: AsRef<[u8]>>(&mut self, data: A) -> String {
        self.digest.input(data.as_ref());
        let res = self.digest.result_str();
        self.digest.reset();
        res
    }

    /// Verifies the given bytes against the provided hash
    pub fn verify<A: AsRef<[u8]>, S: AsRef<str>>(&mut self, data: A, hash: S) -> bool {
        let hash = hash.as_ref();
        let data_hash = self.hash(data);
        data_hash == hash
    }
}
