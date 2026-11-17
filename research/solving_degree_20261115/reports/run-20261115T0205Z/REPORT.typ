#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")

#align(center)[#text(size: 15pt, weight: "bold")[Static solving degree alongside $d_"ff"$ and $d_"lf"$ for the ECDLP index-calculus systems]]
#align(center)[Study `research/solving_degree_20261115`, run 2026-11-15T02:05Z, 4 vCPU local container (no isolation level earned). Every value [pcc:unverified].]

= Outcome

For a unit prime chain instance (m summands, depth k, generator ĝ of ordinary degree δ) the static (XL, no-mutant) solving degree is
$ "sd"^"stat" = delta + "maxwt"("supp" g^(-1)), $
where $g^(-1)$ is the inverse of the normalized generator in the group algebra $FF_p[(ZZ\/2^k)^m]$ (proof: ml-cryptanalysis `docs/proofs/SD1-solving-degree.md`; red-team read found no gap). Hence $d_"lf" <= "sd"^"stat" <= delta + m k$, with equality on the right iff the top coefficient $c_"top" = N^(-m) sum_(alpha in H^m) (alpha_1 dots.c alpha_m) \/ ĝ(alpha)$ is nonzero. Caminata–Gorla's solving degree (mutants added; their Thm 3.1, re-read) is $max(d_"lf", "max.GB.deg")$ and equals $d_"lf"$ for unit ideals, so the static and the iterated degrees are the two ends of the Gröbner cost scale.

#figure(image("sd_growth.svg", width: 100%), caption: [Recorded values only (lanes A, B, C). Left: prime $m = 2$, $"sd"^"stat" = 2k+2$ on every unit instance $k = 3..13$; $d_"lf"$ measured 4,4,4,5,5,5 ($k = 3..8$), recorded exact 5, 6, 6, 6, 6, 7 at $k = 9..14$ (repository runs), law $floor((k+14)\/4)$ drawn through; $d_"ff" = 4$. Middle: prime $m = 3$, $"sd"^"stat" = 3k+6$, $d_"lf" = 7$. Right: binary proper $m = 2$ systems, medians over exact rows: $d_"full"^"stat" in {k+1, k+2}$, LFD in {3,4,5}, $d_"ff" in {2,3}$, semi-regular $d_"reg"$.])

= Checks and sample sizes

#table(columns: 3, align: left,
[check], [result], [source],
[brute-force ordinary-ring Macaulay vs $2 + "maxwt"$], [18,828 / 18,828 agree, $k = 3..6$, 9 primes], [lane A `static_bruteforce.py`],
[independent brute force (own elimination, raw and random generators)], [174 / 174 agree, $k <= 3$], [lane D `check_small.py`],
[three summands, brute force], [40 / 40 agree at $k = 2, 3$ (6 and 9 variables, $D <= 15$)], [lane B],
[inverse verified by convolution $g dot g^(-1) = 1$], [all 29,775 rows (lane A); selftests (Rust)], [lane A, `rust_check`],
[Python vs Rust on shared instances], [4 / 4 identical (incl. the $p = 17$, $k = 3$ exception)], [`rust_check/data/CHECK_LIST.md`],
[exceptions ($c_"top" = 0$, $"sd" = 2k+1$)], [159 of 29,775 rows, all at $k = 3, 4$, rate about $1\/p$], [lane A, Rust: 112 / 1983 at $p = 17$],
[binary: rows with $d_"full" < "LFD"$ or $D^* > d_"full"$], [0 of 3,772], [lane C],
[binary proper $m = 2$: $d_"full" in {k+1, k+2}$], [185 / 185 exact rows, $k = 2..8$], [lane C `static_scan.py`, own GF(2) kernel],
)

= Predictions (frozen before data)

P1 brute-force agreement: held. P2 generic fraction $>= 1 - 3\/p$: held in every cell with unit instances. P3 gap $"sd"^"stat" - d_"lf" >= k$: held (4, 6, 8, 9, 11, 13 at $k = 3..8$ measured; 15..22 at $k = 9..13$ with recorded $d_"lf"$). P4 three summands $6 + 3k$: held. P5 binary $d_"full" >= d_"lf"$ and growing: held. P6 proof survives red-team: held with three qualifiers (δ is the ordinary degree of ĝ; no lower bound without genericity; the $O(1\/p)$ bound is $4^k - 1$ targets per curve, vacuous once $4^k >= p$).

= Wall times and what was not run

Lane A about 80 min (k = 13 at 51 s per draw); lane B 62 min (k = 3 brute force 2406 s); lane C 91 min; Rust runs 17 s + 100 s. Not run: $k = 14$ (memory), $d_"lf"$ at $k >= 9$ (recorded runs, not re-measured), binary exact $d_"full"$ at $k >= 9$ (next degree needs 5–6 GB), any Lean statement, any validator pass. Raw records: ml-cryptanalysis `experiments/ic-bounds-solving/lane*/data/`, crypto `research/solving_degree_20261115/rust_check/data/`.
