mod algorithm;
mod hasher;

pub use algorithm::*;
pub use hasher::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash_verify() {
        let mut hasher = Hasher::from_algorithm(Algorithm::Sha256);
        assert!(hasher.verify(
            "64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c",
            "Hello world"
        ));
    }
}
