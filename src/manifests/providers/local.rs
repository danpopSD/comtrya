use super::{ManifestProvider, ManifestProviderError};

use std::path::PathBuf;

#[derive(Debug)]
pub struct LocalManifestProvider;

impl ManifestProvider for LocalManifestProvider {
    /// The Local provider is essentially our final chance to
    /// resolve the url. It'll match anything and try to find a
    /// directory
    fn looks_familiar(&self, _url: &String) -> bool {
        true
    }

    fn resolve(&self, url: &String) -> Result<PathBuf, ManifestProviderError> {
        PathBuf::from(url).canonicalize().map_err(|_| {
            return ManifestProviderError::NoResolution;
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::{ManifestProvider, ManifestProviderError};
    use super::LocalManifestProvider;

    #[test]
    fn test_resolve_absolute_url() {
        let local_manifest_provider = LocalManifestProvider;

        let cwd = std::env::current_dir().unwrap();
        let cwd_string = String::from(cwd.to_str().unwrap());

        assert_eq!(cwd, local_manifest_provider.resolve(&cwd_string).unwrap());

        assert_eq!(
            Err(ManifestProviderError::NoResolution),
            local_manifest_provider.resolve(&String::from("/never-resolve"))
        );
    }

    #[test]
    fn test_resolve_relative_url() {
        let local_manifest_provider = LocalManifestProvider {};

        let cwd = std::env::current_dir().unwrap().join("examples");

        assert_eq!(
            cwd,
            local_manifest_provider
                .resolve(&String::from("./examples"))
                .unwrap()
        );

        assert_eq!(
            Err(ManifestProviderError::NoResolution),
            local_manifest_provider.resolve(&String::from("never-resolve"))
        );
    }
}
