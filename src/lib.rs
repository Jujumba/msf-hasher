mod algorithm;
mod hasher;

pub use algorithm::*;
pub use hasher::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing_sha1() {
        let mut hasher = Hasher::from_algorithm(Algorithm::Sha1);
        assert_eq!(
            hasher.hash(b"Hello world"),
            "7b502c3a1f48c8609ae212cdfb639dee39673f5e"
        );
    }

    #[test]
    fn test_verify_sha1() {
        let mut hasher = Hasher::from_algorithm(Algorithm::Sha1);
        assert!(hasher.verify(b"Hello world", "7b502c3a1f48c8609ae212cdfb639dee39673f5e"));
    }

    #[test]
    fn test_hashing_sha256() {
        let mut hasher = Hasher::from_algorithm(Algorithm::Sha256);
        assert_eq!(
            hasher.hash(b"Hello world"),
            "64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c"
        );
    }

    #[test]
    fn test_verify_sha256() {
        let mut hasher = Hasher::from_algorithm(Algorithm::Sha256);
        assert!(hasher.verify(
            "Hello world",
            "64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c"
        ));
    }

    #[test]
    fn test_verify_sha512() {
        let mut hasher = Hasher::from_algorithm(Algorithm::Sha256);
    }
}
