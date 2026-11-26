# Graph component development receipts

These are unsigned producer records, **[pcc:unverified]**. They qualify the
specified source and development controls only. No authentic capture was
collected/replayed here and no measured speed improvement is asserted.

- `01-lock-default.*`: the initial offline dependency lookup failed because
  the default Cargo wrapper selected an unpopulated cache. Its stderr was
  preserved from the tool response; later attempts use the already installed
  hydrated toolchain directly. Cached libc 0.2.190 was explicitly pinned.
- `02-lock-cache.*`: successful offline dependency resolution, followed by the
  committed lock. This is dependency setup, not a scientific run.
- `03-check.*`: the first source check failed because Rust's `#[path]` module
  layout did not locate nested original modules. The fix uses the established
  source-exact relative-symlink layout; original modules remain unchanged.
- `04-tests.*`: development/debug integration gate, seven tests passed.
- `05-tests.*`: first release integration gate, seven tests passed; 128 seeded
  dense raw-block projection oracles are included inside one test.
- `05-sources-before.sha256` and `05-sources-after.sha256`: identical hashes of
  every new executable/test source, Cargo manifest/lock and imported-source pin
  manifest around that first release gate.
- `05-release-cli/`: exact planted eight-row M31 fixture, point manifest, four
  separate child outputs/stderr and coordinator summary. All four original
  arms had LP rank 3, left-nullity 5, useful/augmented rank 3, five output rows,
  and recovered scalar 37 verified on the registered ordinary curve. These are
  known-answer development controls, not natural relation yield.
- `RECEIPT.json`: compact scope-bound result; `ARTIFACT_SHA256SUMS` binds every
  retained evidence file. Import pins and executable hashes establish identity,
  not independent signed verification.

The 05 coordinator performed one duplicate group admission before its replay
timer and outside child limits. The targeted **06** revision removes that work
from the coordinator: it performs only byte-capped loading/hash/row-shape checks,
times this preflight explicitly, and starts its total clock at entry. All group
admission is in capped children. `06-tests.*`, the 06 source-before/after hashes,
and `06-release-cli/` retain the corrected-source gate; its `invalid-identity/`
subdirectory retains all four child refusals for a wrong subgroup identity.
Each child's admission, filtering, exact rank, source replay and recovery remain
inside its enforced 60-second CPU/wall and 4-GiB address-space envelope. Read the
terminal 06 exit/output for the current gate; 05 evidence is preserved unchanged.

The gate does not qualify the full root crate, collector provenance or omitted
rejected-attempt costs. `complete_cold_solving_cost` remains null. Intermediate
compiler failures are retained rather than overwritten by the passing receipt.
