# graphset

Generic graph and set-theoretic building blocks: Merkle trees, sorted-set
membership proofs, weighted graph propagation, and the exact fixed-point
arithmetic that keeps propagation results bit-identical across runs.

Nothing in this crate knows what a caller's identifiers mean. Nodes are
`String`, hashes are `[u8; 32]`, weights are `Fixed6`. A caller with richer
identifiers -- a content-addressed hash, a database key, a URL -- converts
at its own boundary and gets an ordinary generic-algorithm result back.

## Modules

- **`log`** -- an append-only Merkle log per [RFC 6962][rfc6962]: leaf and
  interior hashing, inclusion proofs, consistency proofs (that one tree is
  a prefix of a later one).
- **`membership`** -- a sorted-set membership tree: inclusion *and*
  absence proofs (a value is provably *not* present, bracketed by its
  would-be neighbours), plus a diff between two memberships.
- **`propagate`** -- personalized PageRank over a weighted, labelled
  graph, run to an exact iteration count (never a convergence criterion)
  with `BTreeMap`-ordered accumulation, so results reproduce exactly.
- **`fixed`** -- `Fixed6`, base-10^6 fixed-point arithmetic. No conversion
  from `f32`/`f64` exists on the type, and `tools/check-floats.sh` denies
  the float *types* (not just arithmetic on them) anywhere in `src/`.

## Provenance

Extracted from the [`publet`](https://github.com/schryer/publet) workspace
after auditing each piece for coupling to publet's own types (a
content-addressed identifier, a signed object, anything specific to that
protocol):

- `log` and `membership` came from `publet-merkle`. Neither referenced
  `publet-core` anywhere in source or tests before the move -- the
  dependency was declared in `Cargo.toml` and never used.
- `propagate` came from `publet-eval`, decoupled from that crate's
  `Policy`/`Root` types (a content identifier under the hood) to plain
  `String` node labels and a generic `Params`/`Seed` pair. The only place
  the identifier type ever entered the algorithm was reading one field as
  a string and discarding everything else about it.
- `fixed` came from `publet-eval` verbatim; it never referenced anything
  publet-specific to begin with.

`publet-merkle` now re-exports `log`/`membership` from here at their
original path, and `publet-eval` re-exports `fixed` and adapts `Policy`
into this crate's generic `propagate` entry points -- both so every
existing caller in that workspace keeps compiling unchanged.

[rfc6962]: https://www.rfc-editor.org/rfc/rfc6962#section-2

## Development

```
make check   # fmt, clippy --all-targets -D warnings, the float guard, test
```
