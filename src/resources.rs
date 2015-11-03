//! Contains code related to loading resources.
//! 
//! All the resources included with the base game must be prefixed by `core/`. If you request
//! a resource with a name that starts with `core/`, it will be searched for in the resources
//! included with the game. Otherwise it will be searched in the current directory.
//! 
//! Status: figured out but needs to handle non-core resources
//!
use std::str::FromStr;
use std::io::Cursor;

/// Identifier for a resource.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ResourceId(());

impl FromStr for ResourceId {
    type Err = ResourceParseError;

    #[inline]
    fn from_str(s: &str) -> Result<ResourceId, ResourceParseError> {
        Err(ResourceParseError { name: s.to_owned() })
    }
}

// Loads a resource.
#[inline]
pub fn load(resource_name: &ResourceId) -> Cursor<&'static [u8]> {
    Cursor::new(&[])
}

/// Error when turning a resource name into a resource id.
#[derive(Debug, Clone)]
pub struct ResourceParseError {
    /// Name of the resource that failed.
    pub name: String,
}
