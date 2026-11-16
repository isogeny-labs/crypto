# Frozen normalized-square kernel experiment

2026-11-06. Prospective protocol, before the first panel execution.

Question: can a node-preserving square gain crossing valuation without paying
for equally many low-weight moment modes? Candidate W_delta=B_delta^2/rho_delta,
where B_delta=C(T)/(C'(0)(T-delta)), rho_delta=C'(delta)/C'(0). Original B is the
baseline. This is a component experiment, not an ECDLP implementation.

Sources checked: pinned Oct25 manuscript D.4.3/D.4.5/D.4.6 and existing pole-
anchor and repeated-anchor reports. Lead-atlas exact normalized-square/Mahler/
Newton query returned adjacent generic/lattice records, not a priority finding.
KB MCP unavailable; wrapper status failed127. No novelty claim is established.

## Frozen panel

- L=4,8,16,32; S=[-L,L-1]; all t in S with 0<=v2(t)<log2(L).
- zeta=-1,1,3; T=t+2L*zeta. Precisions q=8,12,16,24.
- At level i partition columns into y=y0+2^(i+1)*j. Crossing rows have the
  same ith bit as y0. Use every row and every tile; no random seeds.
- Compute exact rational entries with GMP. Require chart integrality at
  evaluated T, unit rho, and the exact candidate-to-original identity.
- For each row/tile compute full Newton forward differences and independently
  reconstruct each entry with binomial(j,b). Guaranteed weight of a tile's
  bth moment is the minimum coefficient valuation over its targeted rows.
- Independently compute modular Smith pivots for each crossing tile at q.
  Track both change-of-basis matrices and reconstruct the entire original
  matrix modulo2^q. Smith modes are a more general per-cell factorization;
  their construction/description cost and uniformity are NOT free.
- Record all mode weights below q and exact counts of nonnegative exponent
  tuples with weighted degree <q, separately for original/candidate and
  Newton/Smith. Use arbitrary-precision dynamic programming, not fitted timing.
- Positive control: doubling the original matrix shifts every nonzero Smith
  valuation by one, with truncation at q. Negative control: an unnormalized
  square fails the node-value requirement. Repeated identical runs must agree.

## Decisions fixed in advance

A component gain requires fewer admissible tuples in the SAME cell and
factorization class. Report all ties and losses and per-level distributions;
never select only odd levels. A uniform proof attempt requires a bound on
weights across ALL required levels, actual source variables, degree/precision,
entry/basis construction and full child costs. Candidate degree4L-2 versus
original2L-1 must be charged: the old conservative3ML multiplicity bound
alone does not cover the new degree. No finite panel proves sub-exponential
complexity or an end-to-end speedup. [pcc:unverified]

## Execution limits and custody

One deterministic panel plus one exact replay; no adaptive extension. One
process, one CPU, <=2GiB, <=15 minutes per invocation; no speed/timing claim.
EKS is default, but no MCP/AWS caller or live shared-budget remainder is
available. Await explicit backend exception before running the panel locally.
Local compile and fixed expected-value correctness units remain development.
Cloud ceiling USD2 including builds/idle/failed attempts, zero cloud retries;
do not launch when shared budget remainder cannot be checked. Retain failed
attempts. Publish source/protocol before the campaign. No experiment has run
merely because this protocol or a development unit exists.
