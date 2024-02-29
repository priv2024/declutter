use std::{cmp, fmt, hash};
use std::hash::Hash;

use url::Url;

/// Represent a processed url
/// Similar to url::Url but without query values and fragment
pub struct UrlHistoryItem {
    /// Pseudo-BNF but without query values
    serialization: String,
}

impl From<Url> for UrlHistoryItem {
    /// Converts a `Url` into a [`UrlHistoryItem`].
    ///
    /// The result is allocated on the heap.
    #[inline]
    fn from(s: Url) -> UrlHistoryItem {
        let mut serialization = String::with_capacity(s.as_str().len());

        serialization.push_str(s.scheme());
        if let Some(host) = s.host_str() {
            serialization.push_str(host);
        }
        if let Some(port) = s.port() {
            serialization.push_str(port.to_string().as_str());
        }
        serialization.push_str(s.path());
        for (name, _) in s.query_pairs() {
            serialization.push_str(name.as_ref());
        }

        return UrlHistoryItem { serialization };
    }
}

impl fmt::Debug for UrlHistoryItem {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("UrlHistory")
            .field("serialization", &self.serialization)
            .finish()
    }
}

/// URLs compare like their serialization.
impl Eq for UrlHistoryItem {}

/// URLs compare like their serialization.
impl PartialEq for UrlHistoryItem {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.serialization == other.serialization
    }
}

/// URLs compare like their serialization.
impl Ord for UrlHistoryItem {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.serialization.cmp(&other.serialization)
    }
}

/// URLs compare like their serialization.
impl PartialOrd for UrlHistoryItem {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// URLs hash like their serialization.
impl Hash for UrlHistoryItem {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        hash::Hash::hash(&self.serialization, state)
    }
}
