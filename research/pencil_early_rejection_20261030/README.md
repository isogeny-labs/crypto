# Prospective pencil early-rejection instrument

This isolated crate forks the frozen native solver source with SHA256 e71e9cf9b356db1f16781f394994b0ca13abbb05b4a7db82bfd2a6925a0ef4df. The original bytes remain in provenance/frozen-native-solver.rs.txt. It reuses the existing producer engine, kernel and measurement module through a path dependency. The runtime change extracts candidate verification into a helper and moves complement construction below the existing final-bucket length check. No frozen source, ecbench arm or protocol is modified.

This is a prospective software instrument, not an executed target cohort. The runtime solve_one API remains available for later separately admitted work; it has not been invoked in this validation. No fresh performance estimate, finite-cost ratio or timing claim is produced.

## Bounded software validation

```
CARGO_TARGET_DIR=/tmp/pencil-early-rejection-target cargo +1.99.1 test -j2 --locked --manifest-path research/pencil_early_rejection_20261030/Cargo.toml --lib early_rejection_tests -- --test-threads=1
CARGO_TARGET_DIR=/tmp/pencil-early-rejection-target cargo +1.99.1 clippy -j2 --locked --all-targets --manifest-path research/pencil_early_rejection_20261030/Cargo.toml -- -D warnings
cargo +1.99.1 fmt --check --manifest-path research/pencil_early_rejection_20261030/Cargo.toml
```

The selected five tests are ordinary deterministic software fixtures. Twelve inherited source tests are filtered out; some invoke solve_one or rho, so running unfiltered tests is outside this validation's no-solver scope. Clippy compiles all targets but executes no tests. Test and formatter commands use Rust1.99.1; arithmetic tests bind both moduli and enable/reset the native arithmetic and kernel semantic meters.

- Source fidelity pins the frozen file, checks the baseline helper against the exact original closure, and checks the fork outside helper extraction against that source modulo whitespace.
- Polynomial-only rejected-length fixtures cover0,1,2,4,5. For independently derived D4,L1,T17,V16, removal is24base multiplications,30base additions,78visits,4copies,18initializations,27predicates; no inversion or group-operation delta.
- A bounded tiny smooth Scholten curve over F19 provides actual accepted geometric and exceptional-denominator candidate paths. Both helpers retain the same accepted output and field/native/semantic counters; accepted rows produce the same graph cap result.
- The all-lambda branch uses an explicit singular polynomial h=x³(x²+1), u=x²,v=0. This is outside smooth-curve equivalence and is only a software branch fixture. The separate retained static proof explains why the all-lambda condition cannot occur on a smooth quintic.
- Phase snapshots and terminal graph working-cap retention remain checked.

## Scope and retained development failures

The conditional field argument assumes normalized residues and p<=2^63 for helper add/sub safety. The inherited solve_one API does not enforce this narrower proof domain. No arbitrary-u64 panic, allocation, interruption or deadline equivalence is claimed. Rejected candidates intentionally skip work and assertions; future measurements require new source bindings and prospective admission. The static count bound is at most2 skipped complements per sieve and32 over16walks under its stated smooth-quintic hypotheses; it is not a speed estimate.

Attempt1 retained two fixture failures: a tuple field was mistaken for all-lambda count, and the first singular polynomial did not make q0 vanish at the exceptional root. Attempt2 corrected those fixtures and passed. Attempt3 retained a source-fidelity test failure caused by rustfmt's closure braces and a Clippy unnecessary-unwrap finding. Final tests fix the expected source form and use pattern matching. No observed experiment data informed these software corrections.

Literature/source review: the source-bound static proposal review, polynomial identity argument, smooth-quintic rejection-count bound, frozen complement/sieve implementation and existing meter definitions were checked before this implementation. This is a control-flow optimization with no novelty or broader mathematical claim. All provenance sources and software evidence are retained; no external performance result is used.

## Review-strengthened origin coverage

The initial passing smooth fixture only tracked whether a divisor had exceptional denominators; review correctly required classifying the actual candidate origin. The final test traverses the complete bounded point-pair inventory on the same F19 smooth curve, reconstructs exceptional lambdas from q0/(2v) at u(x)=0, and separately counts ordinary point lambdas at u(x)!=0 before exceptional appends. It asserts every candidate has exactly one origin and explicitly exercises exceptional-only accepted and rejected cases. Final software counts are612ordinary-origin,470exceptional-only-origin, of which214accepted and256rejected; total accepted826. These are software fixture coverage counts, not new targets or performance results. The former passing logs are retained as pre-origin-review files.

Graph checks establish source-bound parity for the selected accepted rows and explicit synthetic working-cap case. They are not a general proof of all graph states or all cap boundaries. Frozen graph implementation and cap checks are unchanged. No PDF/report performance qualification follows from these software checks.

Required release verification also passed with the same selected five software tests (12 inherited tests filtered): `cargo +1.99.1 test -j2 --release --locked --manifest-path research/pencil_early_rejection_20261030/Cargo.toml --lib early_rejection_tests -- --test-threads=1 --nocapture`. Raw output is validation/release-tests.log. Debug and release runs are repeat software checks of the same fixtures, not independent experimental observations.
