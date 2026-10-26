# Native validation and provenance

Experiment source/lock frozen at 6811c867f8b86aa3880d48aa8ddaa5f37efed89c.
The input/source protocol hashes are in SOURCE_SHA256SUMS. The dedicated
study executable SHA256 is
`bb7bbf78e572dec9c1130dcaf7ff41af6cd68c9b09a16a13b80a5850306a028c`.
It is retained locally outside Git at the card's bound path; a new build's
byte hash is not assumed identical. Replays compare exact census output.

- Standalone: five release tests, strict release all-target Clippy,
  formatting and production build pass. The exact new CI build/test/lint
  Bash step also passes in the local source tree with its own target directory.
- Director/mechanical relay:70 bindings verified, one native attempt exit0,
  all output/stream hashes verified, no write-scope deviation. The director
  assessment says executed_as_specified and relay_verbatim:true.
- Census:42 distinct nonempty bases, 313 unions, 355 cases, no cap.62,986 unique
  pair sums directly checked. Every completed old singleton coverage agrees.
  SHA256 of census.json is
  `a559f87aeb434906eaf26ae8abe2f682b3d518d212cc13d948158db7e834e884`.
- All 60 inventory-copy hashes match the original8546-member manifest;
  original archive checksum remains
  `eda98dced594f2bd2daaab63528c8a30ac46610cecba24df115eacd24bb060d5`.
- Nine CI-scope tests pass; curve naming reports 0 problems. The new draft
  backlog card validates with exactly two intentional TODO warnings, for
  environment recipe and runnable argv. It remains unrun and zero-funded.
- Root library: ONE complete `cargo test --locked --release --lib -- --test-threads=1` command passes: 4,400 passed, 0 failed, 118 existing ignores, 0 filtered. No source, fixture, assertion, limit or ignore changes. The same previously bound unit binary was reused, not freshly rebuilt.
- Final raw archive: 605,012 bytes, SHA256 `4719a65f670d47d27db24bfc22392c397eeb250a5b7eadac4cd40f78896f9d18`. All member hashes and byte-identical deterministic extracted-tree repacking pass. Both exact dedicated CI Bash steps pass locally; the complete rebuilt census is byte-identical. Reference extraction and replay occur in a separate directory, leaving the original raw tree unchanged. GitHub CI is not yet a pass.

Root production code, Cargo files, old fixtures, registry/alias/store and
old studies/cards are unchanged. All 1,632 existing native input hashes from
source 819d6b172c8a58983bf1b2463fe24576f3f58094 agree. The existing library
binary was previously bound to 731 tracked compiled dependencies and the
exact traits fixture. Its SHA256 remains
`8ce288f3bfcc1529dc63f91d71bdffa852226582fd3bef110b26e789e8261e1f`.
This preserves the earlier provenance rather than claiming a fresh rebuild.
The required full serial library test command is rerun for this branch.

The root command uses Rust 1.98.0, CARGO_BUILD_JOBS2, RAYON_NUM_THREADS2,
--test-threads=1 and networking including loopback. This is correctness
validation on a shared managed host, not an isolated timing benchmark.
No Python performs group/field arithmetic, research computation or verification;
Python is used only for metadata/workflow handling, canonical task-card
orchestration and report rendering. jq formats/filters recorded native values.

Commands, host, logs, original failure observations and input checks are
under runs/validation after extraction. The current Conductor command is
absent 127 and the pinned workflow fetch returned 404; neither is passed.
Lean and Lake commands are absent 127; see FORMALIZATION.md. No assertion,
limit, ignore or source coverage was weakened. See SETUP_OBSERVATIONS.md for
failed setup, sparse staging and relay-tool behavior, and LITERATURE.md for
lookup access limitations. No historical claim is silently corrected here.
