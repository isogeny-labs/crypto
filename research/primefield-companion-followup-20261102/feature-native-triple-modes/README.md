# Triple-mode coefficient reuse

An optional direct-image update obtains modes0,1,2 from one mode1 anchor stream when m>=3 divides h=(r-1)/2. Both adjacent-binomial denominators join the same refill inversion as stride denominators. Mode0's known leading coefficient is restored separately. This removes a separate mode0 stride traversal when the staged update would also evaluate modes1/2; it does not store a full coefficient vector.

`TripleReader` has bounded refill storage: count<=cap triples, at most3*cap denominators and2*cap interval requests. `triple_modes` shares quotient powers across up to four branches. `DirectImage::update_triple` applies the same two gcd conditions using representatives in the old quotient. `recover_triple` is a separate adapter with the original source schedule, initialization, fallback and final point verification. The original staged adapter is unchanged.

The tradeoff is explicit: fusion evaluates modes1/2 before C0 pruning, potentially in a larger quotient or for a branch the staged path would discard. Extra neighbor arithmetic, larger refill vectors, third-component ring work and leading-power restoration are charged obligations. No default-policy or runtime-superiority claim is made.

## Reproduce software qualification

Run `python3 generate.py` then `python3 validate.py FRESH_DIR`. Generator copies the exact terminal-arithmetic predecessor, records reversible edits and creates the renamed optional adapter. Validation uses optimized Rust with overflow checking,90s compile,60s focused checks and60s inherited checks. Every attempt uses a fresh directory. Python performs orchestration/hashes, not arithmetic.

Focused fixtures cover40 readers across eight(r,m) pairs and five block caps; three coefficient streams; scalar quotient moduli including repeated roots and nilpotents; full candidate-support updates; and843 public target/cap pairs on E/F257, y²=x³+x+7, P=(1,3), subgroup order281. The original4563-case cohort is unchanged. Full inherited stdout is compared byte for byte with preserved output.

Source bindings and receipts identify actual checks. Producer assertions remain unsigned pending independent signed review. No cloud/timed workload or production width panel runs here. Historical full Cargo missing-fixture limitation remains.

Sources checked: fused.rs/fused-block.rs, neighbor.rs, direct.rs/direct-adapter.rs, reused.rs and coefficient definitions in the predecessor. KB unavailable; direct source/algebra fallback. Adjacent-binomial identities are standard; this is a specific bounded reader and decoder integration, not a novelty claim for those identities.

Next: compare the optional fused and staged update on a frozen matched panel, preserving original degree, C0-surviving degree, refill work and quotient operations. Keep zero-survivor cases, where staged pruning may win. Source-cost and characteristic-transfer research remain separate active obligations.
