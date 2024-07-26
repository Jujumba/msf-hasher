use clap::ValueEnum;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, ValueEnum)]
#[clap(rename_all = "UPPER")]
pub enum Algorithm {
    Sha1,
    Sha512,
    #[default]
    Sha256,
    Md5,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sha1 => write!(f, "SHA1"),
            Self::Sha512 => write!(f, "SHA512"),
            Self::Sha256 => write!(f, "SHA256"),
            Self::Md5 => write!(f, "MD5"),
        }
    }
}
