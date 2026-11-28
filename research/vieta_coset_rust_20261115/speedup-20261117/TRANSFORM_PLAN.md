# NTT plus half-GCD successor

Question: can fast polynomial products make half-GCD effective enough to exceed
2x full-cold improvement over the immutable Euclid/Newton reference at word62
m16384, with the same complete relation set? This is a distinct candidate after
the field-only calibration fell short on word and rational fixtures.

Reuse the literal owner evaluator through a small generated product hook;
assert each changed anchor exists exactly once. Owner source files and prior
study pins stay unchanged. Reuse native Fp64/Fp224, including their constructors.
The transform schedule is adapted from src/cryptanalysis/isogeny_eval/ntt.rs
(Gentleman–Sande forward, Cooley–Tukey inverse). These specific primes admit
large power-of-two transforms; this does not assume every prime has them.
Root search certifies exact order in field arithmetic, and unsupported lengths
are rejected. No cross-prime CRT or floating-point transform is used.

Prospective choices: min operand128 coefficients, allocation cap2^21,
root candidates2..64. All root search/powers, twiddle construction, butterflies,
point products/scaling are counted; clear per-Arithmetic tables, never global
cache. Double-query control builds a fresh Arithmetic stage for its second
query, so both queries pay root/table setup. Native field constructor operations
remain unpriced by core counters, but setup is inside the full cold clock.
Failed transform prefix counters may be incomplete if an internal limit returns
before its work is attached; such failures stay censored, never wins.

Keep legacy schoolbook, Newton/half and shared-field Newton/half references.
New transform arms explicitly name NTT and NTT-half. Confirm fresh primary
against Newton; separate field-only and NTT calibration panels from primary.
All candidates build certified descriptor and full point base inside cold;
lookup additionally builds its hash set. Target/oracle preparation is outside
worker clocks and reported separately. These are relation queries, not scalar
DLP recoveries. No fastest-external-tool claim, L2 clock admission or independent
signed standing follows from local measurements.

Sources checked: repository NTT owner at c5cbb67b19 tree, SHA256
9648552e5cdccbf8d64cfd0b67c824e98bb73298a9f7a24727f9b0aab07c23f7;
FLINT hgcd.c/gr_poly/hgcd.c previously cached; official multiplication and GCD
API docs https://flintlib.org/doc/nmod_poly/mul.html and
https://flintlib.org/doc/nmod_poly/gcd.html (documentation itself flags generated
portions; implementation and exact arithmetic tests govern this experiment).
KB MCP unavailable and exact/negative/application wrapper queries127; no
novelty/absence claim. No new agent, paid/cloud or S3 work.
