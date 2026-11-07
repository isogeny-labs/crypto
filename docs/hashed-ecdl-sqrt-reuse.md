# Hashed-ECDL square-root setup reuse

2026-10-30. Factor-base enumeration now retains Tonelli–Shanks setup across
compressed-point decodings. For primes congruent to 3 modulo 4, extraction
uses one exponentiation followed by an exact square check. These classical
implementation changes reduce repeated setup work on correctness fixtures;
wall-time and end-to-end performance remain unmeasured here.

## Sources and implementation

Before editing, reviewed the native arithmetic, decoder, factor-base loop,
tensor documentation and earlier input/replay receipts. The
[pinned Bochum v0.2 source](https://github.com/bochum-crypto/subexponential-ecdl/tree/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f),
section 3.5, enumerates inverse-hash encodings by solving `y² = x³ + ax + b`
and selecting the encoded parity. Its cited
[Handbook of Applied Cryptography, chapter 3](https://cacr.uwaterloo.ca/hac/about/chap3.pdf),
Algorithm 3.34 and Fact 3.35, separate prime-only setup from extraction.
The modulus decomposition, sampled nonresidue and its initial power can be
reused. Algorithm 3.36 gives `(p+1)/4` for `p ≡ 3 mod 4`; checking the square
of that candidate also rejects nonsquares without another exponentiation.

`arithmetic::SquareRootContext` borrows one fixed odd prime and lazily stores
its decomposition and successfully computed initial nonresidue power.
Each extraction clones that initial power before changing local state.
Storage is O(log p) bits, with no root table. The caller still certifies
primality and supplies randomness. Zero needs no setup, nonsquares do not
sample a nonresidue, and random-source or resource failures remain errors.
Every nonzero call checks the current meter's bit limit even after reuse.

`curve::PointDecoder` binds the context to one certified curve, and
`FactorBase::enumerate` uses one decoder for its loop. Range, parity,
curve-equation replay, subgroup checks, ordering and duplicate rejection
retain their existing checks. Existing `square_root` and `Curve::decode`
one-shot APIs remain available. No dependency, resource limit, validation
gate, verifier equation or receipt representation changed.

Fewer RNG draws can change later seeded query histories and diagnostics.
Reproducing an execution therefore includes its source version. Canonical
point selection is preserved, and the earlier scalar-4 certificate still
replays with the rebuilt CLI. Frozen evidence retains its original bytes.

## Validation

Five new Rust tests cover the change:

- The same 18 nonidentity points on the public F17 curve are obtained from
  an independent subgroup scan, separate decoders and inverse-hash listing.
  A controlled test RNG is called 18 times by separate decoders and once by
  the batch decoder. The old enumerator fails the one-call assertion.
- Exhaustive independent residue tables for primes 3, 5, 7, 17, 19, 43, 97
  and 257 agree in both input orders; every returned root is squared and
  checked. This catches accidental mutation of cached setup state.
- Lazy setup, zero/nonsquare handling, random-source failure and retry,
  noncanonical input and new narrower/exhausted meters are covered. Another
  sequence requires one cached setup versus six separate setups.
- On F7, both a square and a nonsquare fit the existing five-step abstract
  budget for one two-bit exponentiation. The earlier two-exponent path fails
  the same test. RNG and meter observations are unit diagnostics, not timings.

Final algorithm source at validation launch:
`c84df306cc583957f8351ff233c50ef422900697`.

| Check | Observed result |
| --- | --- |
| Standalone native release panel, `cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml` | exit 0; 39 core + 5 runner + 3 strict-JSON tests, all 47 passed |
| Native strict all-target Clippy | exit 0 |
| Native Rust 1.78.0 all-target check | exit 0; existing unused `check-cfg` manifest warning only |
| Strict native API docs, `RUSTDOCFLAGS='-D warnings'`, `--no-deps` | exit 0 |
| Native formatting | exit 0 |
| Rebuilt CLI, existing stored receipt | exit 0; `certificate verified: scalar 4` |
| Mirrored cryptanalysis selected release panel | exit 0; 32 core + 5 runner + 3 JSON + 20 binding + 19 FFI + 14 documentation tests, all 93 passed |
| Crypto root, uncapped `cargo test --release --lib` | exit 0; complete parent result: 5430 passed, 0 failed, 123 pre-existing ignored; all 5553 accounted for |

The [transcript and receipt](hashed-ecdl-sqrt-reuse-tests.txt) retain raw-output
digests, original reproduced failures, exact commands and validation source
identity. The first new test was initially outside its test module and failed
to compile for a missing import; it was moved inside. The old enumerator and
earlier two-exponent path then fail the intended assertions. Final selected
panels pass without weakening either assertion or limit. Harness elapsed
times are unpaired development output and establish no CPU speed ratio.

Newer cryptanalysis main adds independent prime-proof, functional-contraction
and panel-runner changes. A separate source export of its merge preview with
PR 1309 passes all 106 selected-package tests (42 core, 5 runner, 3 panel,
3 JSON, 20 binding, 19 FFI, 14 docs), strict lint, Rust 1.78, API docs,
formatting and old-receipt replay. The transcript records its exact tree and
19-file source manifest. This preview leaves the conflicted session log and
published PR head untouched. Its first wrapper returned status 1 despite a
complete passing panel and internal Cargo status 0; preserved that mismatch
and repeated via direct Cargo execution, which returns 0 with all 106 passed.
Those preview results apply to a future resolution only if source identity
matches; they do not replace current-head CI and review.

No research experiment, benchmark, external-tool comparison, challenge solve,
cloud resource, leaderboard promotion or artifact-store upload was performed.
Any oversized full-library output stays local with NOT_UPLOADED status and a
compact digest receipt; a local path is not a durable remote artifact.

## Integration and source identity

This extends [crypto PR 2906](https://github.com/isogeny-labs/crypto/pull/2906),
which also contains the separately validated strict-input correction.
The [input receipt](hashed-ecdl-tensor-input-validation.md) and
[earlier main-integration receipt](hashed-ecdl-tensor-input-integration-validation.md)
remain historical snapshots. Their root and native passes are not substituted
for the fresh checks above.

Merged main in `11497c764e6c0037e0c79ede979932b5748ded32` after the Copilot
request received no response for over 50 minutes and the documented fallback
was explained on the PR. The only conflicts were unsealed session logs for
October 29 and 30. Both parents' complete ordered line sequences were checked
as subsequences of the resolved files. Source changes merged automatically;
main's prime certificates, canonical witnesses and bounded GPU-base hook were
preserved. No frozen evidence conflict was resolved by choosing a side.

The new [18-file manifest](hashed-ecdl-sqrt-reuse-sources.sha256) covers native
Cargo, source and test files. Its SHA-256 is
`3b2893f8984ab4895c195e88f46bb172c006a2a29147cc17dbfe23166b95037c`.
From `crates/hashed-ecdl-tensor`, run
`sha256sum -c ../../docs/hashed-ecdl-sqrt-reuse-sources.sha256`.
The new square-root, decoder and enumeration fragments match cryptanalysis
PR 1309 head `b02b919d4eec0691dd00438d0e0d9ca7f26fedce` byte for byte.
Whole crate trees differ because of independent main-branch changes; the
receipt records fragment hashes rather than claiming whole-tree parity.

Conductor scope checks returned 127 because the executable is absent.
`docs/CONDUCTOR.md` documents the warn-and-allow fallback; no shared
reservation is claimed. Before normal guarded MERGE, require current-head
`gate`, `size`, `artifacts`, actual protection/review requirements and no
blocking feedback.

The full independent mathematical and complexity audit stays open in
[the canonical problem](../problems/hashed-ecdl-tensor-backend.md). This
classical refactor adds no new theorem or Lean proof, full conditional
subexponential replication, end-to-end speed, fastest-tool, sub-rho or
deployed-curve claim.
