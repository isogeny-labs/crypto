# Frozen non-fibre Jacobian coset slice: proposal 13

Frozen 2026-11-28 (America/Los_Angeles), before compile or producer execution. This is a bounded **coset-slice control**, not the proposed linear-section algorithm. Public fixed input: `p=211`, `C:y²=F(x²)` with `F=[122,208,210,149]`, elliptic quotient `E1:y²=x³+210x²+186x+126` of order 218, subgroup order `r=109`, `G=(165,19)`, and exact target `Q=(178,46)`. These are the p=211 actual pullback instance in the [chained-transfer V3 receipt](../chained_prime_transfer_20261111/V3.md) and its `certs_v3.jsonl`; the scalar in that receipt is **not** an input to this collector. The same public `G,Q` go to a separately initialized same-target rho arm. No external target or cloud spend. `[pcc:unverified]`

## Construction, acceptance, and cost

Use the repository's explicit degree-two bielliptic maps `φ1:C→E1` and `φ2:C→E2` (`src/cryptanalysis/chained_transfer/bielliptic.rs`). Select `B` as the first rational `E2` point in the implementation's deterministic enumeration for which `K=φ2*(B)` is nonzero, `[2]B` is nonzero, and `φ1_*K=0`. This is a structural rule, independent of relation success. For 128 fixed xorshift-stream pairs `(a,b) mod 109` from seed `0xC0FFEE13`, form `R=aG+bQ` and `D=φ1*(R)+K`. All 128 candidates, including infinity, degree-one, nonsplit, ramified, equal-image and projected-collapse outcomes, must be printed. No cap extension, K replacement or post hoc slice selection.

For each degree-two `D=(u,v)` with **two distinct rational roots** of `u`, reconstruct both divisor places, map them separately to `E1` points `P1,P2`, and require the exact point identity `P1+P2=φ1_*D=[2]R`; also require `φ2_*D=[2]B`. Accept a non-fibre relation only if both projected points are finite and distinct and their cofactor-cleared images `H_i=[2]P_i` remain finite and distinct. Require `[109]H_i=O` and `H1+H2=[4]R`. A row involving the target has `b≠0`; preserve the first exact signed point witness. Distinguish this from **target rank gain**, which is computed separately.

Build the complete natural factor-base dictionary **before** collection: all rational `E1` points with a rational place on `C` (the source x-square membership test), after `[2]` projection to the order-109 subgroup. For every accepted row, record `log(H1)+log(H2)-4b·log(Q)=4a (mod 109)` as a coefficient vector. Report base-column rank, rank with the target column, and their difference, without supplying a planted log. A rank difference of one is the exact algebraic condition for eliminating base columns to determine the target; a target-bearing row alone is weaker. The source stream may already have a generic `R` collision, so count these separately. The factor base can be large; incomplete rank is an admitted result, not an invitation to change it.

Time cold cover construction, K selection and factor-base setup, candidate collection, rank, and total slice work separately with `Instant`. The same-target rho separately constructs `E1`, checks `G,Q`, runs `measure::rho_on_ec` at seed `12648439`, and verifies any recovered scalar by `[k]G=Q`. Its counted group operations and cold wall time are reported. Slice wall timings and rho group operations are **different cost units**; no speed ratio is admitted. Candidate projection and factor-base enumeration are inside the slice interval. Compiler time is outside both arms. The repository's prior [pushforward study](../cover_pushforward_20261112/PUSHFORWARD.md) found whole-fibre rows give only `[2]R=R+R`; this fixed complementary translate tests whether a non-fibre split divisor can give **distinct** projected points. The primary [Diem index-calculus construction](https://eprint.iacr.org/2005/119.pdf) requires verified split-divisor relations, but does not itself imply an advantage for this slice. The shared KB search on 2026-11-28 found prior split-Jacobian and same-field transfer leads; this experiment is a narrow new candidate against that local evidence, not a novelty claim.

The user's earlier explicit local checkpoint bypass remains in force. This worktree has no `.resume/RESUME.md`, so its required S3 checkpoint gate cannot pass locally; this public bounded run is admitted under that override. No PCC claim is minted. Source is [the native example](../../examples/ec100_jacobian_coset_slice_013.rs), frozen SHA-256 `8ca8e5fbd6da6edba91b27d4b3e415639ae98174f7117919875548235d892dde` on `origin/main` base `1df77237f7a6879b6985fc797ec2da20bdef1835` with Rust/Cargo 1.93.1. Run exactly once from repo root; preserve failure before any amended retry:

```sh
cargo build --release --example ec100_jacobian_coset_slice_013 2> research/prime_field_100_ideas_20261123/EC_JACOBIAN_COSET_013.stderr.txt
compile_status=$?
if [ "$compile_status" -eq 0 ]; then
  target/release/examples/ec100_jacobian_coset_slice_013 > research/prime_field_100_ideas_20261123/EC_JACOBIAN_COSET_013.stdout.txt 2>> research/prime_field_100_ideas_20261123/EC_JACOBIAN_COSET_013.stderr.txt
  run_status=$?
else
  run_status=not_started
fi
printf 'compile=%s,run=%s\n' "$compile_status" "$run_status" > research/prime_field_100_ideas_20261123/EC_JACOBIAN_COSET_013.exit.txt
```

Require shell compile/run zero, all 128 candidate lines, a completion sentinel, valid exact point checks, and an independently checked source point for any rho scalar. No output is admitted at protocol freeze. `[pcc:unverified]`
## Frozen same-input toolchain retry, 2026-11-28

First attempt used PATH `cargo`/rustc 1.93.1 and ended at dependency MSRV check (`compile=101,run=not_started`); its stderr and exit receipt remain unchanged. The source SHA-256 remains `8ca8e5fbd6da6edba91b27d4b3e415639ae98174f7117919875548235d892dde`. Retry once with installed Rust 1.99.1, identical source, public curve, target, candidate stream, and all stopping/acceptance conditions. Keep V2 stdout, stderr, and exit in separate `EC_JACOBIAN_COSET_013_V2.*` files. No producer output has been inspected because compilation did not begin on attempt 1.

Exact retry: `$(rustup which cargo --toolchain 1.99.1) +1.99.1 build --release --example ec100_jacobian_coset_slice_013`, redirect stderr to V2 stderr; if compile succeeds, execute `target/release/examples/ec100_jacobian_coset_slice_013` once, redirect stdout and append stderr to V2 receipts. Record both statuses in V2 exit.

## Frozen launcher correction V3, 2026-11-28

Attempt 2 also ended before compilation: the explicit toolchain Cargo binary rejected the redundant `+1.99.1` argument (`compile=101,run=not_started`). Its V2 stderr and exit are retained. Retry once with `rustup run 1.99.1 cargo build --release --example ec100_jacobian_coset_slice_013`, directing compiler stderr to V3 stderr, then if successful execute the same example binary once and capture V3 stdout/stderr/exit. Source SHA, all mathematical inputs, candidate stream, controls, and stop/acceptance rules are unchanged. This launcher command binds Cargo and rustc to the 1.99.1 toolchain. No producer output exists from attempts 1–2.

## Frozen explicit compiler correction V4, 2026-11-28

Attempt 3 again stopped at rustc 1.93.1 MSRV (`compile=101,run=not_started`): `rustup run 1.99.1 cargo` resolved the Homebrew Cargo on PATH and did not override its chosen compiler. Retain V3 stderr/exit. Use the exact installed toolchain binaries for both processes: `RUSTC="$(rustup which rustc --toolchain 1.99.1)" "$(rustup which cargo --toolchain 1.99.1)" build --release --example ec100_jacobian_coset_slice_013`; redirect build stderr to V4 stderr. If compilation succeeds, execute the same example binary once, with V4 stdout and appended stderr, and record both statuses in V4 exit. Same source SHA, input and acceptance rules; none of the first three attempts executed the producer.

## Frozen sparse-checkout repair V5, 2026-11-28

Attempt 4 reached Rust 1.99.1 compilation but the tracked repository module `research/quartic-diem-sparse_20261115/src/qds_core.rs` was missing from this worktree's sparse checkout; compile=101 and the producer did not start. Retain V4 stderr/exit. Expand sparse checkout to this exact tracked module, without changing its contents. Retry once with the same explicit Cargo/RUSTC command, directing to V5 stdout/stderr/exit. Source SHA and every mathematical input, seed, control, and acceptance condition remain frozen.

## Post-V5 independent target certificate protocol

V5 compiled and ran successfully. The 128 candidate records contain 72 accepted signed relations; these were not used to modify the original collector. For equations `A*x + t*d = c`, the observed `rank(A)=42`, `rank([A|t])=43` means that a left-null vector of `A` has nonzero dot product with `t`, and so **pins** `d`. My initial message to the parent misinterpreted this rank gain; the independent verifier below corrects it without changing the frozen V5 source or raw.

Freeze the separate PARI/GP verifier `EC_JACOBIAN_COSET_013_VERIFY.gp` before its first run. Extract only accepted V5 lines into the local temporary `gpdat` file with this exact command:

```
perl -ne 'if (/^candidate trial=\d+ a=(\d+) b=(\d+) status=accepted .* h1=\((\d+), (\d+)\) h2=\((\d+), (\d+)\)/) { print "[$1,$2,$3,$4,$5,$6]\n" }' research/prime_field_100_ideas_20261123/EC_JACOBIAN_COSET_013_V5.stdout.txt > /tmp/ec13_coset_rows_20261128.gpdat
gp -q research/prime_field_100_ideas_20261123/EC_JACOBIAN_COSET_013_VERIFY.gp > research/prime_field_100_ideas_20261123/EC_JACOBIAN_COSET_013_VERIFY.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_JACOBIAN_COSET_013_VERIFY.stderr.txt
```

Record GP exit separately; require exactly 72 extracted rows, all 72 independent EC group identities, ranks 42 and 43, a printed 72-entry left-null combination `y`, `y·t ≠ 0`, `d=(y·c)/(y·t)` in F109, and `[d]G=Q` using GP's EC arithmetic. The verifier bases only on the observed projected points, which has the same matrix column span as the frozen preconstructed base after removing zero columns. No scalar from the V3 certificate or producer rho is supplied to the verifier.

## Frozen GP V2 parser repair

The first GP verifier attempt printed top-level multiline parse errors on stderr, while GP returned shell exit 0 and misleading partial stdout. It is a failed qualifier and both receipts remain unchanged. The only V2 repair is to wrap the arithmetic in a GP function. V2 SHA-256 is `2659bac5c784101bb56df0e62f591c00ef8d6f4a8218f9d8c115a6e4d7ac7a5f`. It reads the identical 72-row local `gpdat` and independently checks all group identities, modular ranks, target scalar, and the fixed four-row combination at trials 12, 51, 55, 70 with weights 9, 100, 9, 100. Run `gp -q EC_JACOBIAN_COSET_013_VERIFY_V2.gp`, saving separate V2 stdout/stderr/exit. Acceptance additionally requires empty stderr, the completion line, four-row base residual zero, target coefficient 1, RHS 8, and `[8]G=Q` in GP.

## Observed outcome and limits

V5 source SHA-256 `8ca8e5fbd6da6edba91b27d4b3e415639ae98174f7117919875548235d892dde`, raw stdout SHA-256 `da4d4a2bf96dce9709be6a22c8816a4fb56fdd126ce4b00ce7b66844e587c87a`, `compile=0,run=0`. Exactly 128 candidate lines: 72 accepted, 53 nonsplit/ramified, 2 infinity, 1 projected collapse. There were 73 split distinct divisors, 72 distinct cofactor-cleared projections, zero full fibres, and all 72 accepted rows had nonzero `b`. The complementary point was `B=(1,143)` on E2 and its pullback had Mumford coefficients `u=[210,189,1]`, `v=[145,121]`. The source and split-place pushforward identities were asserted for each accepted row.

The preconstructed natural base had 84 columns. Its row matrix rank was 42; adjoining the target column raised rank to 43. The independent GP V2 verifier used the 52 actually observed base coordinates, found the same ranks, checked all 72 group equations using PARI/GP EC arithmetic, and derived `d=8` from a left-null vector (`y·t=4`, `y·c=32`). GP V2 SHA-256 is above; stdout SHA-256 `b2173b9828ec2c66e617c983e88a3d85bbbc62dc84d9eab1a628ebfb4ac08950`, stderr empty, `gp=0`. Its compact replayable certificate combines accepted trials 12, 51, 55, 70 with coefficients 9, 100, 9, 100 modulo 109. All factor-base terms cancel; weighted `a=2`, weighted `b=27`, so the target coefficient `−4·27=1` and RHS `4·2=8` modulo 109. GP independently checked `[8]G=Q`.

This is a valid non-fibre relation construction and a same-target scalar witness on this fixed public small group. It is **not** evidence of a competitive ECDLP method: source enumeration itself had 48 source-point collisions in 128 trials, with the first at trial 26 (`(a,b)=(80,101)` and `(4,56)`), which already yields `76G+45Q=O` and `d=8`. The Jacobian pipeline therefore did not create an advantage over the direct collision available from its own source stream. The matched same-target rho arm verified `d=8` in 428 reported EC group ops and `51,625 ns` cold wall; the slice including K/base setup, 128 candidate evaluations, and rank took `1,891,459 ns` cold wall in this single run. The wall figures are diagnostic, with different group/cover operations; no speedup claim follows. Larger groups, base construction cost, coefficient support, and relation independence remain open gates.

The direct-collision control was also checked independently with `gp -q` on the same curve: `elladd(e,ellmul(e,G,80),ellmul(e,Q,101)) == elladd(e,ellmul(e,G,4),ellmul(e,Q,56))` returned `1`; `lift(Mod(-76,109)/Mod(45,109))` returned `8`; `ellmul(e,G,8)==Q` returned `1`. Thus the upstream source stream had a scalar certificate by trial 26, before the fourth Jacobian certificate row at trial 70.
