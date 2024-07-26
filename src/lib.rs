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
    fn test_hash_sha512() {
        let mut hasher = Hasher::from_algorithm(Algorithm::Sha512);
        assert_eq!(
            hasher.hash("Hello world"),
            "b7f783baed8297f0db917462184ff4f08e69c2d5e5f79a942600f9725f58ce1f29c18139bf80b06c0fff2bdd34738452ecf40c488c22a7e3d80cdf6f9c1c0d47"
        );
    }

    #[test]
    fn test_verify_sha512() {
        let mut hasher = Hasher::from_algorithm(Algorithm::Sha512);
        assert!(
            hasher.verify(
                "Hello world",
                "b7f783baed8297f0db917462184ff4f08e69c2d5e5f79a942600f9725f58ce1f29c18139bf80b06c0fff2bdd34738452ecf40c488c22a7e3d80cdf6f9c1c0d47"
            )
        );
    }
}
