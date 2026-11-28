# Tensor-contraction ECDLP review, clique-sum thresholds and polynomial decay (2026-11-21/22)

Moved from isogeny-labs/workspace#60 on the owner's instruction that research PRs belong in this
repository. Tracking PR: isogeny-labs/crypto#4904.

| File | Content |
|---|---|
| `REVIEW.md` | Emergency review of "Conditional Subexponential Attacks on Structured LWE" (ChatGPT / N. Smart, 2026-11-21): theory, source verification, connection to the Bochum tensor-contraction ECDL preprint and `cryptanalysis/crates/hashed-ecdl-tensor`; §5a–§5f constants of OpenAI Theorem D.1, literature, threshold dichotomy for ECDLP, sub-2^420 #3-SAT crossover (2^385 proved by re-reading, 2^324 with Lemma Q), distance to ECDLP-256 / AES-128 / Ring-LWE-1024 |
| `POLYNOMIAL_DECAY.md` | The search for a clique sum with polynomially decaying exponent: target as a constant-growth level, input-floor limits (β ≤ 1, C ≥ 1), β = 1 implies bounded-occurrence #3-SAT in 2^Õ(n^{2/3}) under Theorem A's overhead (2^Õ(√n) only with a 2^{o(K²)} overhead; corrected 2026-11-25), reach criterion U < G² (ECDLP-256 and Ring-LWE-1024 unreachable even at the optimum), coverage lemmas, literature |
| `threshold_model/` | Reimplementation of Majdoddin's main-term threshold model (arXiv:2610.41622) with validation against his published columns, the overhead corrections, Lemma Q (`qb_model.py`), lever sweep, and the crypto-distance table; see its README |

Docs and Python only. No measurement, no speed claim, no leaderboard change.
