#set page(paper: "a4", margin: 1.8cm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#set table(stroke: 0.4pt + rgb("#c3c2b7"), inset: 4pt)
#show table: set text(7.6pt)
#text(13pt, weight: "bold")[Run report: run-20261018T1648Z-sweeper-native]

*What ran.* The native port of round 1's sweeper, `examples/isogeny_loop_sweeper.rs` in the main crate (class group by `src/isogeny/class_group_subexp.rs`, relation lattice and weighted shortest loop by `src/isogeny/loop_lattice.rs`), on the branch `worktree-agent-a2b18b3c4769eec4f` at commit c9ba859db (the source of the binary; 9c6603970 is the same source after `rustfmt`). rustc 1.97.0, release. Host: Intel Xeon \@ 2.10 GHz, 4 CPUs, cloud container, not isolated; several of these runs overlapped one another and a test build (load average 5–13, recorded in `groups_ad_env.txt`). All outputs are in `../../sweeper_native/`.

#table(columns: 5,
  [*run*], [*command (after `isogeny_loop_sweeper`)*], [*start UTC*], [*wall*], [*files*],
  [scan 128], [`scan --scan-from 3 --scan-to 3000 --bits 128 --cost-model projective,affine,xonly --top 1 --frozen-scan …/scan_D3-3000_bits128.json`], [16:48:01], [4 m 54 s], [`scan_bits128.*`],
  [scan 256], [same with `--bits 256` and the 256-bit frozen file], [16:53:14], [5 m 00 s], [`scan_bits256.*`],
  [scan char 2], [`scan … --bits 128 --char2 --cost-model projective,xonly`, frozen `…_char2.json`], [16:53:15], [1 m 48 s], [`scan_bits128_char2.*`],
  [groups a + d], [`curve --groups group_a_koblitz_and_known.json --groups group_d_certicom_small.json --cost-model projective,affine,xonly --top 6`], [16:55:33], [10 m 51 s], [`groups_ad.*`],
)
Defaults in force, as in round 1: primes $l <= 100$, doubling 8 M, enumeration radius = the saving $8 ceil(log_2 r slash 2)$ M (capped by the LLL bound), `--max-disc-bits 140`, maximal order, seed `0x5eedc1a556000001`. Cost models: projective $7.5 l$; affine $2l + 2$; x-only $2l$ for odd $l$ and 4 for $l = 2$ in odd characteristic; in characteristic 2 the step above 2 is $F$ = 2 M forward and $V$ = 10 M backward in every model.

*Checks asserted per run.* $t^2 - 4q = f^2 D_K$; every relation verified in the lattice; held-out relations (32) in the lattice; $|log(h slash h_"approx")| < log sqrt 2$ (Euler product to $2^22$); every loop generator $alpha$ verified against its norm; class number, cycle structure and cheapest projective cost compared with the frozen PARI records. All runs exit 0.

*Recorded values.*
- Scans: 911 fundamental $D in [-3000, -3]$ (306 with $D equiv 1$ mod 8 in characteristic 2). Round-1 projective costs reproduced 911/911, 911/911 and 306/306. Median cost / saving: 128-bit 0.293 (projective), 0.102 (affine), 0.078 (x-only); 256-bit 0.146, 0.051, 0.039; char. 2 0.066, 0.047. Maximum: 1.699 (128-bit projective, two $D$ above 1), else below 0.64.
- Groups a + d (13 curves): $h$ and cycle structure match PARI on 12 (CryptoPro-A is skipped by both at 258 bits); cheapest projective cost matches on 12. Cheapest non-Frobenius loop, cost / saving, projective / affine / x-only, with "UB" = LLL upper bound above the exhaustive radius: sect113r1 21.0 / 5.9 / 5.7 UB; ECC2-89 20.5 / 5.3 / 5.1 UB; ECCp-89 49.4 / 13.7 / 13.2 UB; ECC2-79 23.4 / 6.5 / 6.3 UB; ECCp-79 20.4 / 5.6 / 5.4 UB; CryptoPro-B 0.125 / 0.039 / 0.033; secp256k1 0.051 / 0.016 / 0.014; the five $D = -7$ Koblitz curves 0.004–0.009 in every model (the Frobenius). Exhaustive within the radius: true for every curve and model. Per-curve wall times: 0–1 s for $h = 1$, 16–52 s for the 79–89-bit discriminants, 540 s for sect113r1 (115 bits; 121 s in an uncontended earlier run, `groups_ad_projective_before_fix.log`; PARI 14 s).

*Earlier and failed runs, kept.*
+ `groups_ad_projective_before_fix.*` (15:46Z, projective only, source b0f4a12e5): the same 12 matches; wall 2 m 38 s uncontended.
+ `group_b_attempt1_before_fix.log` (15:49Z): group b (131-bit discriminants) with the same source. The class group never reached full rank: "rank deficient" at 2,895, 3,628, …, 13,955 relations, the target growing by a quarter each time; stopped by hand after 18,000 relations.
+ `ecc2-131_debug_before_fix.log` (about 16:00Z, 15 min): the diagnostic run on ECC2-131's discriminant alone. 104,276 relations, 2,546 of 2,547 columns never hit: every trial relation was the ramified $727^(-2)$ or a zero relation. Cause: reduced forms have $a <= sqrt(|D| slash 3) approx 2^64.7$, and a product of at most four factor-base primes ($<= 49,470$) has $a <= 2^62.3$, so the product was already reduced and factored back into its own primes. At 115 bits the threshold is $2^56.7$ and the method worked. Fix (fa264086a): multiply factor-base primes until the product passes $sqrt|D|$ by 12 bits before reducing. Second fix (c9ba859db): that made the term count constant for tiny $D$, which trapped $D = -163$ at $h^* = 2$ (even coefficient sums); 0–2 random extra terms restore the varying count. After both, `cargo test --release --lib -- isogeny::class_group_subexp isogeny::loop_lattice`: 7 passed, including all 6,079 fundamental $|D| < 20,000$ against brute force.
+ Groups b and c (97–131-bit discriminants) with the fixed source: see the §5 update in `../../README.md`; the ECC2-131 class group was still running when this report was written (GROUPS_BC_PLACEHOLDER).

#figure(image("../../figures/sweeper_scan.svg", width: 100%), caption: [Cumulative distribution of cheapest-loop cost / saving over the scanned fundamental discriminants, per cost model (recorded values, run-20261018T1648Z-sweeper-native).])
#figure(image("../../figures/sweeper_curves.svg", width: 100%), caption: [Cheapest non-Frobenius loop per challenge curve and cost model; hatched bars are LLL upper bounds above the exhaustive radius (no loop below the saving exists).])

*Observations only.* Interpretation: `../../README.md` §5.
