# Frozen independent full-grid S4 replay for proposal 24

Frozen 2026-11-27 (America/Los_Angeles), before verifier compile/run, on the public prime-order curve `E/F_1019: y²=x³+x+20`, `#E=1033`. This is a verification follow-up to the previously frozen [second-curve relation pipeline](EC_S4_REPLICATION_024_PROTOCOL.md), whose producer stdout SHA-256 is `48d4b66dd0b0f56ad1d1e500bcc05c0952ce927d4fe0bbc5dbc49317d6c37db0`. It is established Semaev S4 algebra, not a new relation algorithm. The prior accepted-row qualifier independently checked 17 selected signed rows and modular rank 17, but the producer alone checked the entire candidate grid. This verifier addresses that specific gap. `[pcc:unverified]`

## Frozen inputs and independent arithmetic

Use the same `G=(0,64)`, `Q=(38,245)`, and 16 factor-base abscissae `[7,8,10,13,16,18,19,20,22,23,24,28,29,31,35,36]`. The 11 target points are `(a,b)=(1,0),…,(10,0),(0,1)` in that order, with `R=aG+bQ`. For each target, inspect all `16³=4096` ordered triples and all eight sign patterns. This is 45,056 S4 evaluations and 360,448 signed patterns, matching the original workload. No target logarithm or producer relation row is used in the verifier computation.

The [independent Rust verifier](EC_S4_FULLGRID_VERIFY_024.rs) computes affine EC addition and scalar multiplication directly, with a separate modular inverse and curve-membership check. For each base x it picks the smallest square-root y; this may differ from GP's deterministic root, but enumerating all eight signs leaves the set and multiplicity of signed witnesses unchanged. It evaluates `S4(x,y,w,z)` through the explicit two-quadratic resultant identity `Res(A t²+B t+C,D t²+E t+F)=(AF−CD)²−(AE−BD)(BF−CE)` in `F_1019`, rather than invoking GP's `polresultant`. It checks for every triple that S4 zero is equivalent to at least one exact signed group sum equaling `R`. It also checks G/Q order, base membership, every target x, per-target zero/witness counts, totals, and a completion sentinel. Stop at the first mismatch; preserve any compiler or runtime failure.

The predeclared producer counts for targets `(1,0)` through `(0,1)` are S4 zeros `[12,42,48,57,36,79,30,27,0,12,21]` and signed witnesses `[12,42,48,57,36,126,30,27,0,12,21]`. Expected totals: 364 zero triples, 411 signed witnesses, 720,896 two-addition signed-sum operations. The difference at `(6,0)` is an expected degeneracy and must remain visible. A successful replay checks the exhaustive algebraic/point equivalence on this fixed grid; the existing accepted-row qualifier remains the separate rank and target-log check. It adds no new same-target solve or speed measurement. The prior pipeline's matched Floyd rho independently verified the same Q scalar, 316, in seven iterations and 22 point additions, while the relation pipeline used 720,896 signed-sum additions and 45,056 S4 evaluations. No speed claim follows from this verifier.

The repo technique inventory and [original Semaev paper](https://eprint.iacr.org/2004/031.pdf), Theorem 1, were reviewed in the second-curve protocol; this run replays its exact group/relation criterion. The checkpoint gate previously failed because `.resume/RESUME.md` was absent and its S3 credential was rejected. The user's explicit local bypass admits this bounded public-curve verifier. No cloud spend, external target, or PCC claim. `[pcc:unverified]`

Frozen source SHA-256: `d06a1711152f053024b1d183b835f4a0838447a5ddebf6f2ff87ae8a9b828bdf`. Native compiler: `rustc 1.93.1 (01f6ddf75 2026-02-11)`. Run once from this repository root, with the local binary outside the repository, recording both statuses:

```sh
rustc -O --edition 2021 research/prime_field_100_ideas_20261123/EC_S4_FULLGRID_VERIFY_024.rs -o /tmp/ec_s4_fullgrid_verify_024.bin 2> research/prime_field_100_ideas_20261123/EC_S4_FULLGRID_VERIFY_024.stderr.txt
compile_status=$?
if [ "$compile_status" -eq 0 ]; then
  /tmp/ec_s4_fullgrid_verify_024.bin > research/prime_field_100_ideas_20261123/EC_S4_FULLGRID_VERIFY_024.stdout.txt 2>> research/prime_field_100_ideas_20261123/EC_S4_FULLGRID_VERIFY_024.stderr.txt
  run_status=$?
else
  run_status=not_started
fi
printf 'compile=%s,run=%s\n' "$compile_status" "$run_status" > research/prime_field_100_ideas_20261123/EC_S4_FULLGRID_VERIFY_024.exit.txt
```

The source, compiler diagnostic stream, raw stdout, and exit record are the frozen evidence paths. Do not alter workload constants or restart silently after an error. No verifier output is admitted at freeze.

## One-shot result and custody

The frozen Rust source compiled once and ran once. The exit record is `compile=0,run=0`; stderr is empty and stdout ends with `complete independent full-grid EC S4 replay`. All 11 target x coordinates, per-target zero counts and signed-witness counts matched the earlier producer. Across all 45,056 triples, the direct quadratic resultant was zero exactly when at least one of the eight independently computed signed point sums equaled the fixed target. The verifier counted 364 zero triples and 411 signed witnesses in 360,448 patterns, with the predeclared `(6,0)` degeneracy preserved as 79 versus 126. `[pcc:unverified]`

| Artifact | SHA-256 |
|---|---|
| [Frozen Rust source](EC_S4_FULLGRID_VERIFY_024.rs) | `d06a1711152f053024b1d183b835f4a0838447a5ddebf6f2ff87ae8a9b828bdf` |
| [Raw stdout](EC_S4_FULLGRID_VERIFY_024.stdout.txt) | `e2cb38f4f288f3bd15619ad3346b78dc3a957e86d36bc7f407de65d05e6809fd` |
| [Empty stderr](EC_S4_FULLGRID_VERIFY_024.stderr.txt) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Compile/run exit](EC_S4_FULLGRID_VERIFY_024.exit.txt) | `1cdf81fd1f9bd313a87a218fbc05c6b97c5b685e98cb1637180f9b357f52d344` |
| Local `/tmp/ec_s4_fullgrid_verify_024.bin` (unstaged) | `7cbf6c862396963cdf8b51e9446ed28455155d66561c19bc2d7e430ca2ddea2b` |

This closes the **fixed-grid equivalence replay** gap for the prior second-curve experiment. It does not independently rederive the producer's choice of 17 accepted rows or measure collection time. The separately preserved accepted-row GP qualifier covers rank and target point verification; the same-target rho arm remains the prior matched cost baseline. No broader-curve or asymptotic conclusion follows. `[pcc:unverified]`
