# Reproduce original-root extension controls

Verify SOURCES.sha256 from the repository root and the two archives in
FROZEN.md. Rust1.100.0 and the exact Cargo.lock identify the native build.
Use cargo build --release --locked with the new crate's manifest. A fresh
build needs a NEW source/binary qualifier; do not overwrite the retained v1
binary or silently treat a different executable as the qualified binary.

The frozen qualifier-v1.sh accepts a NEW output-directory argument. It
checks all49 source subjects and the retained executable hash before calls,
refuses an existing destination/control log, and writes compiled source and
binary hashes to the new native pre-call file before evaluating controls.
Its fixed retained-binary path is environment-specific; a different machine
creates a separately qualified reproduction, preserving this script.

Native comparison trace order: the four membership fixtures in PROTOCOL.md,
j ascending, real coordinate0..100, imaginary coordinate0..100. Bit0 is
original membership and bit1 indexed extension existence. Only canonical
extension values are enumerated; every representable t is checked on an
original root. Native output explicitly does not exhaust arbitrary auxiliary
assignments. The final24024 bytes enumerate ordered distinct-raw signed
leaves and the13 sorted image-group targets, bits0/1/2 for original/new/
expected target decisions. Offsets and complete counts are in controls.json.

The actual known witness is the unchanged original v2 six-member fixture,
indices0,2,4,6,8,10 of the old member controls. Labels enter only this
extension harness; no root-discovery solver is called. The original system
is read from its unchanged frozen gzip and compared with the freshly compiled
original before augmentation. Keep all original equation/variable prefixes.

Run only the sibling lean-indexed-v1 focused proof gate, with its pinned
toolchain/manifest and existing CryptoLean/Mathlib revisions. Gate logs and
explicit axioms are retained; the first redundant-tactic source/log is
preserved separately. No gate/policy/keyring exception is needed.

Local release test/fmt/strict Clippy are construction checks. Incidental
build/test durations are not admitted timings or a comparative result.
No paid job, solver sampling/campaign or broad root-library rerun is part
of this reproduction. Full-accounting solver work is a later protocol.
