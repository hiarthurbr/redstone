use once_cell::sync::Lazy;
use regex::Regex;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

static NS_RE: Lazy<Regex> = Lazy::new(|| Regex::new("[a-z0-9.-_]").unwrap());
static VAL_RE: Lazy<Regex> = Lazy::new(|| Regex::new("[a-z0-9.-_/]").unwrap());

/// Identifiers are a namespaced location, in the form of `minecraft:thing`.
/// If the namespace is not provided, it defaults to `minecraft` (i.e. `thing` is `minecraft:thing`).
/// Custom content should always be in its own namespace, not the default one.
/// Both the namespace and value can use all lowercase alphanumeric characters (a-z and 0-9), dot (`.`), dash (`-`), and underscore (`_`).
/// In addition, values can use slash (`/`)
pub struct Identifier {
    pub namespace: String,
    pub value: String,
}

/// Represents an error that can occur when parsing an identifier.
/// This is returned from [`Identifier::new`] and [`Identifier::from_str`].
///
/// [`IdentifierError::NameSpace`]: The namespace does not match the following regex: `[a-z0-9.-_]`
///
/// [`IdentifierError::Value`]: The value does not match the following regex: `[a-z0-9.-_/]`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum IdentifierError {
    /// The namespace does not match the following regex: `[a-z0-9.-_]`
    #[error("The namespace does not match the following regex: [a-z0-9.-_]")]
    NameSpace,
    /// The value does not match the following regex: `[a-z0-9.-_/]`
    #[error("The namespace does not match the following regex: [a-z0-9.-_]")]
    Value,
}

impl Identifier {
    /// Creates a new identifier.
    ///
    /// # Errors
    ///
    /// Returns [`IdentifierError::NameSpace`] if the namespace does not match the following regex: `[a-z0-9.-_]`
    ///
    /// Returns [`IdentifierError::Value`] if the value does not match the following regex: `[a-z0-9.-_/]`
    ///
    pub fn new(namespace: String, value: String) -> Result<Identifier, IdentifierError> {
        if !NS_RE.is_match(&namespace) {
            return Err(IdentifierError::NameSpace);
        }

        if !VAL_RE.is_match(&value) {
            return Err(IdentifierError::Value);
        }

        Ok(Identifier { namespace, value })
    }
}

impl FromStr for Identifier {
    type Err = IdentifierError;

    fn from_str(str: &str) -> Result<Identifier, IdentifierError> {
        let (namespace, value) = str.split_once(':').unwrap_or(("minecraft", str));

        if !NS_RE.is_match(namespace) {
            return Err(IdentifierError::NameSpace);
        }

        if !VAL_RE.is_match(value) {
            return Err(IdentifierError::Value);
        }

        Ok(Identifier {
            namespace: namespace.into(),

            value: value.into(),
        })
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.value)
    }
}
