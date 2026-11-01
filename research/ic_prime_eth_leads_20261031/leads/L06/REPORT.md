# L06 - Finite-size contraction width of the hashed-ECDL relation network

Status: measured (partial; counters blocked). Verdict: CLOSED (known negative with measured width constants).
Labels: [M] measured (path), [D] derived, [P] plan.

## Provenance and what was NOT done
- The Rust crate could not be built: `cargo build` was refused by the permission system (don't-ask mode). No graph was dumped from the crate. [M]
- Substitute: `model.py` re-implements the *graph structure only* (which bits share which vertex) of `encoding.rs` (integer_multiply Fourier gadget, modular ops, valid_point, point_add, hash_member, decomposition_network, equality trees from `Encoder::finish`). [D]
- Validation against the crate's own published ECCp-131, m=2 graph (docs/hashed-ecdl.md l.33: 1,564,506 edges, max degree 1,032): model gives max vertex arity 1,032 (exact) and 1,685,148 edges (+7.7%) [M, raw/fourier_sizes.txt]. Edge counts are therefore approximate; arities are exact. Cause of the 7.7% not isolated (modulus bit patterns, first-addition handling, or crate version).
- Tools found: networkx 3.2.1, numpy, scipy, opt_einsum, kissat, cadical. NOT found: cotengra, sharpSAT-TD, d4, ganak. Counter arms: **blocked**. No CNF exporter exists in the crate (FINITE_SIZE.md), so kissat/cadical sanity was not run. Own elimination code used (no networkx call needed).

## Network definitions
Relation network = Lemma 11 / `reduction::decomposition_network`: m input points (x,y in b bits), each curve-valid, each hash-member of a factor base (s of b+1 bits), m point additions starting from the identity, final equality to a constant target. Variables = bit edges; every shared bit is expanded to a degree-3 equality tree (as the crate does). Cofactor 1.
- `fourier`: as built (Theorem 3 two-stage Fourier gadget; black-box vertices). Vertex = clique in the primal graph (dense tensor).
- `school`: same network with the crate's own optional `schoolbook_multiply` (arity <= 5 gates). This is the gate-level network that a CNF/model counter would see. It is the only arm where exact elimination widths are meaningful.
- Prototype binary network (continuity): `btn_ecdl_20261029_backup/width.py`, re-run read-only: n=5 min-fill [23,23,23], min-degree [23,23,23] [M, raw/prototype_width_n5.txt]; n=7 value 34 quoted from README (not re-run). Ratio to enumeration (m-1)n/m = 2.5 at n=5: 9.2x.

Width definition: log2 of the largest contracted table = |S_f u S_g| over a pairwise (variable) elimination; min-degree and min-fill orderings (fill counted exactly on the edge hypergraph). Lower bounds on the treewidth of the primal graph: degeneracy and minor-min-width MMD+ (least-c). All are valid bounds for the modelled network. Commands: `python3 analysis.py school B M deg,fill,lb`, `python3 cut.py MUL B M POLICY KS none|deg|fill`.

## Results
### School (gate-level) arm [M, raw/school_grid.jsonl]
| b | m | enum (m-1)b/m | vertices | edges | min-degree UB | min-fill UB | MMD+ LB | degeneracy |
|---|---|---|---|---|---|---|---|---|
| 4 | 2 | 2.0 | 10,798 | 16,420 | 111 | 67 | 15 | 4 |
| 4 | 3 | 2.7 | 16,338 | 24,921 | 91 | 67 | 16 | 4 |
| 6 | 2 | 3.0 | 20,712 | 32,968 | 170 | 99 | 20 | 4 |
| 6 | 3 | 4.0 | 31,327 | 49,983 | 156 | 104 | 19 | 4 |
| 8 | 2 | 4.0 | 33,778 | 55,084 | 221 | 146 | 27 | 4 |
| 8 | 3 | 5.3 | 51,076 | 83,461 | 275 | 148 | 27 | 4 |
| 10 | 2 | 5.0 | 49,996 | 82,768 | 286 | 178 | 31 | 4 |
| 10 | 3 | 6.7 | 75,585 | 125,355 | 294 | 204 | 33 | 4 |
| 12 | 2 | 6.0 | 69,366 | 116,020 | 423 | 258 | 36 | 4 |
| 12 | 3 | 8.0 | 104,854 | 175,665 | 421 | 235 | 37 | 4 |
| 16 | 2 | 8.0 | 117,562 | 199,228 | 505 | 331 | 49 | 4 |
| 16 | 3 | 10.7 | 177,672 | 301,533 | 520 | 389 | 51 | 4 |

Slopes (least squares over b=4..16) [D]: MMD+ lower bound about 2.8-2.9 per bit (m=2: 15 -> 49; m=3: 16 -> 51); min-fill upper bound about 22-27 per bit (heuristic, loose). Enumeration slope 0.5 (m=2) and 0.667 (m=3). Lower bound exceeds enumeration at every size, ratio 5.7x (m=2) and 4.8x (m=3) at b=16. b=24..64 were not run for the school arm (graph 0.4M..1M edges, Python elimination too slow for the 90-minute cap); extrapolation is [P].

### Fourier (as built) arm
Max vertex arity (clique lower bound arity-1 on treewidth, holds for any dense-table contraction) [M, raw/fourier_sizes.txt]:
| b | 8 | 16 | 24 | 32 | 48 | 64 | 131 |
|---|---|---|---|---|---|---|---|
| max arity | 132 | 260 | 260 | 520 | 520 | 1032 | 1032 |
| enum m=2 / m=3 | 4/5.3 | 8/10.7 | 12/16 | 16/21.3 | 24/32 | 32/42.7 | 65.5/87.3 |
Edges: m=2: 64.9k, 128k, 235k, 256k, 456k, 509k, 1.685M; m=3: 97.8k, 193k, 354k, 385k, 687k, 766k, 2.53M (b = 8,16,24,32,48,64,131). 92 (m=2) / 138 (m=3) modular-multiply gadgets per network.
b=8, m=2 exact runs [M, raw/cut_fourier.jsonl]: min-degree width 315, MMD+ 197, arity 132 (vs enum 4). b=16 Fourier elimination did not finish in the cap.
Caveat [D]: with *sparse functional* vertices (outputs determined by inputs) the dense bound is an artefact; the live-index count of a forward column vertex is first*k input bits (e.g. 64 at b=64, 128 at b=131), still >= b > (m-1)b/m. Not rigorous as a treewidth bound; see fourier-width.md (not found on disk at the given path).

### Cut arm (conditioning) [M, raw/cut_school.jsonl, raw/cut_fourier.jsonl]
School, MMD+ LB after pinning (value at k=0 in brackets), k + width never below the k=0 width:
- k input bits (sequential or random): b=6,m=2 [20]: k=4:20-22, 8:18-22, 16:20, 24 (all inputs):21; b=8,m=2 [27]: k=4:27-28, 8:25-27, 16:25-26, 24:25, 32 (all inputs):24-25; b=6,m=3 [19]: all 36 inputs: 20-21. Pinning *every* input bit lowers the bound by at most 3. The width is carried by existential witness bits (slope, quotient theta, z) and the b x b constant-multiplier adder arrays, which no input cut touches.
- k internal words (other cut classes tried): slope words, b=8,m=2: k=1: 27, k=2 (16 bits): 22; partial sums (m=3, b=8): k=1: 27, k=2 (32 bits): 21; (b=6,m=3: 22 / 17 at k=1,2). A drop of 5-6 for 16-32 pinned bits: worse than bit-for-bit.
Fourier, b=8, m=2: pinning 1 transform word (33 bits) leaves MMD+ 197 and arity 132 (no change in the clique bound); min-degree UB 264/292 vs 315 at k=0 (noisy heuristic). Transform words carry <= min(J+1, d) bits of information (the DFT is invertible), so conditioning on k transform words costs at least as many assignments as the input bits that determine them [D]. No cut class had width growing slower than bit cuts.

## Decision rule and verdict
Rule (LEADS L06): width >= (m-1)b/m at every size with no cut class beating bit cuts => closed; only a cut class with slower-growing width is positive.
- Width lower bound > enumeration exponent at every measured size, both arms, both m: school 15..51 vs 2..10.7; Fourier clique bound 131..1031 vs 4..43. [M]
- No cut class (inputs, slopes, partial sums, transform words) reduced width faster than bit cuts. [M]
=> **CLOSED**: "exact contraction (or component-caching counting) of the arithmetic relation network does not beat enumeration at finite size; measured width constants: school LB ~2.9b (UB ~22-27b), Fourier dense width >= 8b-16b arity". Re-proposals of "just run sharpSAT-TD/d4" are answered for this encoding (counter timing not measured; width LB makes dense/structured counting exponential in b with a constant >= 5x the enumeration slope).
Limits of this verdict: (a) graph is a faithful-structure model, not a dump (edges +7.7%); (b) LB only certified on the model's primal graph; the UB is a poor greedy and not the true treewidth, true slope lies in [2.9, ~22]; (c) counters not run; (d) b <= 16 for the school arm; b=24..64 [P]; (e) different encodings (e.g. a CNF with Karatsuba/binary multiplier sharing) could change constants (L11).

## Skeptic re-run
`cd /Volumes/SSD990-2/ic-prime-eth-leads-20261031/leads/L06 && python3 model.py 131 2 fourier && python3 analysis.py school 8 2 deg,fill,lb`
(first line must print max_vertex_arity 1032; second should give MMD+ 27, min-fill about 146 for the seed-0 run.)

## Next step
1. Owner approves `cargo build --offline` in a scratch copy (or a dump flag upstream) so the real graph replaces the model; compare edges (expect the 1,564,506).
2. Export the school network as CNF (Tseitin of the gates in model.py is direct) and run kissat/cadical sanity at b=4..8; with owner-approved sharpSAT-TD/d4/ganak/cotengra, time them at b=6..10 to confirm exponent >= LB slope.
3. Extend the school arm to b=24..64 with a compiled min-degree (Rust/C) and add a stronger LB (e.g. QuickBB/exact on subnetworks, single modular-multiply sub-network width ~b by Bryant-type argument).
