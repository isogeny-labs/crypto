# Common scalar square: prospective exact finite census

This study asks whether replacing the entry normalizer by one common scalar
removes the earlier mode proliferation. It is component research on the
crossing matrices from the prime-field ECDLP programme. It neither solves a
curve instance nor measures execution speed.

Let L=2^a, S={-L,...,L-1}, B_d(T)=prod_(s!=d)(T-s)/C'(0),
rho_d=B_d(d), R(T)=sum_(s in S) B_s(T)/rho_s^2.
Compare original B_d, entry square B_d^2/rho_d, and scalar square R B_d^2.
R(d)=1/rho_d; therefore all source-node values are preserved. This includes
zero, so the identity at the origin is retained. Value-preserving C(T)H(T)
jet completion cannot change it. Derivative-rank reduction remains a separate
question. [pcc:unverified]

## Registered census and analysis

For L16,32,64, every nonzero t in [-L+1,L-1], zeta=-1,1,3,
T=t+2L*zeta, i=v2(t), stride=2^(i+1). Tile origins y0 range [0,stride),
rows x range [0,L) with bit_i(x)=bit_i(y0), columns y=y0+stride*j.
Every matrix entry is computed as a GMP rational. Newton differences must
reconstruct every entry. Smith factors modulo 2^q retain both transformation
matrices and reconstruct the original residues. Assert v2(R)=0 at every T.

At q16,24,32,48 compare each kernel with original at equal precision.
Additionally for base q16,24 compare entry square at ceil(4q/3), and scalar
square at 2q, against original at q. These are explicit conservative precision
proxies, not actual full recovery budgets or total algorithm costs. Degree
upper bounds are 2L-1, 4L-2, 6L-3 respectively. The source recovery formula
also has additive terms, so no claim of fully charged cost follows.

There are 654 (L,t,zeta) triples. Four precisions times three kernels times two
factorizations give 15696 same-precision rows. Two base precisions times two
candidates times two factorizations give 5232 proxy rows. Total20928 rows per
physical invocation. The old L16,32/q16,24 cells are calibration; L64 and
q32,48 are held out relative to the previous study, not to literature.

Count nonnegative exponent vectors with weighted sum < used_q exactly using
a GMP integer coin loop. The separately frozen analyzer uses grouped
generating functions and binomial multiplicities, checks all20928 unique keys,
recounts41856 tuple totals, emits every level and the total fewer/equal/more
counts, and requires byte-identical replay. Factorizations remain separate.
All losses and ties are reported. No statistical test or adaptive selection.

## Execution and controls

The physical paired-panel arm contains all kernels and controls. Its replay arm
runs the identical executable. One instance and one replicate per arm give two
executions, no retries. Seed0 names the deterministic census. Each invocation uses one logical CPU, 2GiB address-space
limit, 900s hard timeout. Local backend uses the user's standing approved
override; no paid/cloud work. No timing metric; host contention affects
completion only, not exact arithmetic results. Preserve partial output on any
failure and stop before the next invocation. No third panel after results.

Embedded controls: original-vs-original equality in every row; full process
A/A replay; doubling original shifts Smith weights by one below cutoff;
L4/d2/node2 unnormalized square25 differs from required5; all source nodes
at L4,8; modular48 wraparound/product and known Smith1,2; tuple5/6 controls.
These are embedded assertions, not invented physical arms. Control objects
excuse absent separate control processes and point here to their actual scope.

Before data: commit and publish instrument/analyzer/adapter/protocol, record
source/binary hashes and freeze receipt. Export has two physical rows, counted
rows_completed, no valid clocks and verification no_claim. Native Rust checks
both canonical JSON schemas plus study-specific freeze/source/analysis/binary/
file bindings and flow. This deliberately scoped checker is not represented
as equivalent to the canonical general Python checker. Independent review and
signed PCC standing remain external obligations.

## Sources checked

KB unavailable (MCP absent; repository wrapper status127). Fallback: prior
study check.cpp, analyze.cpp, RESULTS_local_20261107.md and lean-normalizer;
research/prime_common_kernel_20261030/sources.md primary-source pins;
crypto-autoresearcher experiment-protocol skill, normative document and schemas.
Atlas scalar-normalizer/Lagrange/Cauchy query produced adjacent leads only.
Lagrange interpolation and squared geometric series are established tools;
no novelty or literature-exhaustion assertion is made.
