//! Generic graph and set-theoretic building blocks: Merkle trees, sorted-set
//! membership proofs, weighted graph propagation, and the exact fixed-point
//! arithmetic that keeps propagation results bit-identical across runs.
//!
//! Nothing here knows what a caller's identifiers mean. Nodes are `String`,
//! hashes are `[u8; 32]`, weights are [`Fixed6`]. A caller with richer
//! identifiers (a content-addressed CID, a database key, a URL) converts at
//! its own boundary and gets an ordinary generic-algorithm result back --
//! the conversion is where whatever theory the caller's identifiers answer
//! to enters the picture, not before it.
//!
//! # Provenance
//!
//! Extracted from the `publet` workspace (`publet-merkle`'s `log` and
//! `membership` modules, `publet-eval`'s `fixed` and the generic core of
//! its `propagate`), after auditing each for coupling to publet's own
//! types. None of the code moved here referenced a content identifier, a
//! signed object, or any other publet-specific type before the move --
//! only `publet-eval`'s trust-propagation entry points took a
//! publet-specific `Policy` as an argument, and only to read four plain
//! fields out of it, which is now the caller's job (see [`propagate`]).

pub mod fixed;
pub mod log;
pub mod membership;
pub mod propagate;
pub mod tree;

pub use fixed::{Fixed6, SCALE};
