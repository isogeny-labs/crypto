# Review pointer: a separate cover/sieve lead, not a new trial

16 November2026, during stage19 compilation. No method implementation or
measurement is selected by this note. [pcc:unverified]

Sources checked (KB): MCPabsent; wrapper queries about Joux–Vitse weak Fp6
cover decomposition/Rust and transfer-kernel/exceptional negative results have
no executable; /tmp/ic-cover-survey-kb-search{1,2}.log. Fallback native
jv_cover.rs/jv_sieve.rs, ICtechniques weak-cover section, cover ledger and
jv_cover_crossover_20261101/README.md read from exact tracked Git paths.
Sparse absence is not a missing-source result. [pcc:unverified]

Primary: Joux–Vitse, *Cover and Decomposition Index Calculus on Elliptic Curves
Made Practical*, [author PDF](https://www-fourier.ujf-grenoble.fr/~viva/research/articles/CoverDecomp.pdf),
SHA2567ca88c54ebfcb6fac8d05d313bfe8c26d3cb133081c55c4ab6ea4c1b45729b38,
local /tmp/ic-cover-decomp-primary.{pdf,txt}. Sections3.2/3.3 read: a quadratic
parameter sieve over Fp² tabulates parameter roots at base abscissae; counts
of six small roots leave a degree-two cofactor for two possible large primes.
The paper explicitly links this variation to structured elimination and
large-prime relations. It applies only when the intermediate cover is small
and the specified field structure is available. No speed/exponent transfer to
our cubic S4 square systems or arbitrary prime-order curves. [pcc:unverified]

The repository already implements the genus3 Fp² cover and univariate sieve
for a restricted weak elliptic family over Fp6. Reuse that implementation;
do not reconstruct a new cover from a summary. Its crossover report compares
actual recovery with two rho implementations and retains reproduction/cost
limitations: the stronger reference yields unresolved intervals at the two
larger tested primes, and the reproduction control changes the linear-stage
instrument. Those results are pointers, not a matched benchmark for this
branch. No inherited parity or admitted ranking assertion. [pcc:unverified]

A later review can consider a two-large-prime adapter into the symbolic graph,
with verified Jacobian relations and correctly protected descent/constant
coordinates. Before selection: inspect existing sieve/filter/large-prime
implementations and newer ledger outcomes, validate the transfer subgroup and
full witness, freeze fresh instances/strongest references/full cost boundary,
and retain actual failures. Cover construction, factor-base setup, collection,
graph filtering, sparse solve, descent and elliptic scalar replay must be
charged for any end-to-end comparison. No trial or implementation authorized
by this note alone; the user's ongoing task supplies the overall scope.
[pcc:unverified]
