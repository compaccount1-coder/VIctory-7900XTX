//! Everything specific to the title: its container formats, build
//! identities, and the hooks we place in its code (located by per-build
//! offsets or byte-pattern signatures). What may and may not be stored here
//! is defined in docs/asset-policy.md.

pub mod sgd5;

/// A specific build of the title executable we know how to run.
#[derive(Debug, Clone, Copy)]
pub struct KnownBuild {
    /// Our own label, e.g. "1.0.0 retail".
    pub label: &'static str,
    /// BLAKE3 of the original executable, hex.
    pub blake3: &'static str,
}

/// Builds we support: the ones the renderer patches' byte checks and the
/// per-build offsets were made against.
pub const KNOWN_BUILDS: &[KnownBuild] = &[KnownBuild {
    // CodeView 2A6912BAEB4F427F4C4C44205044422E, age 1; the image name
    // `victory-image-b99f876b74d4a77f.dll` in crash reports.
    label: "Series X|S",
    blake3: "b99f876b74d4a77f03af5f878223b27bd647b5c35a79d5decd9f716504239e93",
}];

/// Looks up a build by the BLAKE3 hash of its executable.
pub fn identify(blake3_hex: &str) -> Option<&'static KnownBuild> {
    KNOWN_BUILDS
        .iter()
        .find(|b| b.blake3.eq_ignore_ascii_case(blake3_hex))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_builds_are_found_by_their_hash_in_either_case() {
        for b in KNOWN_BUILDS {
            assert_eq!(b.blake3.len(), 64, "{}", b.label);
            assert!(identify(b.blake3).is_some());
            assert!(identify(&b.blake3.to_uppercase()).is_some());
        }
        assert!(identify(&"0".repeat(64)).is_none());
        assert!(identify("").is_none());
    }
}
