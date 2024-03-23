#![cfg_attr(not(test), no_std)]
#![warn(clippy::all)]
#![doc = include_str!("../README.md")]

use core::fmt;
use core::hash::{Hash, Hasher};

use fnv::FnvHasher;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A unique, 128-bit numerical identifier that can cheaply generate new ID's.
///
/// Retains the parent ID as well as depth information.
///
/// Note: all algorithms are deterministic and platform-independent.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Id {
    id: u64,
    parent: u128,
    depth: u32,
    gen: u32,
}

impl Id {
    /// Create a new root-level `Id`.
    ///
    /// Note that subsequent calls return *the same ID,* which will generate
    /// *the same children!*
    pub fn root() -> Self {
        Id {
            id: 0,
            parent: 0,
            depth: 0,
            gen: 0,
        }
    }

    /// Returns the numerical ID for use.
    ///
    /// Returns 0 for the root ID.
    pub fn id(&self) -> u128 {
        (self.depth as u128 + 1).wrapping_mul(self.parent ^ (self.id as u128))
    }

    /// Returns the ID of the parent `Id`; 0 if this is a root ID.
    pub fn parent(&self) -> u128 {
        self.parent
    }

    /// Returns how many ancestors this `Id` has.
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// Returns the number of direct children this `Id` has produced.
    pub fn num_children(&self) -> u32 {
        self.gen
    }

    /// Generate a new, unique `Id` from this one.
    pub fn next_id(&mut self) -> Id {
        self.gen = self.gen.wrapping_add(1);

        let mut state = FnvHasher::default();
        self.id.hash(&mut state);
        self.parent.hash(&mut state);
        self.depth.hash(&mut state);
        self.gen.hash(&mut state);

        Id {
            id: state.finish(),
            parent: self.id(),
            depth: self.depth + 1,
            gen: 0,
        }
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Id {}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.depth.cmp(&other.depth)
    }
}

impl Hash for Id {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x}", self.id())
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x}", self.id())
    }
}

#[cfg(test)]
mod test {
    use std::collections::{HashSet, VecDeque};

    use crate::Id;

    #[test]
    #[ignore = "very expensive"]
    fn no_duplicates() {
        //! Ensure that there are no duplicate ID's.
        //! **WARNING**: eats CPU and RAM like pacman

        let mut queue = VecDeque::new();
        let mut set = HashSet::new();

        queue.push_back(Id::root());

        // u64::MAX would guaranteed OOM any computer
        // 100,000,000 gets SIGKILLed on my desktop
        for i in 0..50_000_000 {
            let mut current = queue.pop_front().unwrap();
            let next = current.next_id();

            if !set.insert(next.id()) {
                panic!(
                    "collision at {i} (id={} parent={} depth={})",
                    next.id, next.parent, next.depth
                );
            }

            queue.push_back(next);
            queue.push_back(current);
        }
    }

    #[test]
    #[ignore = "very expensive"]
    fn no_root_duplicates() {
        //! Ensure that `root` doesn't generate duplicate ID's too quickly.
        //! **WARNING**: also CPU/RAM intensive

        let mut set = HashSet::new();
        let mut root = Id::root();
        for i in 0..100_000_000 {
            let next = root.next_id();
            if !set.insert(next.id()) {
                panic!(
                    "collision at {i} (id={} gen={})",
                    next.id(),
                    root.num_children(),
                );
            }
        }
    }
}
