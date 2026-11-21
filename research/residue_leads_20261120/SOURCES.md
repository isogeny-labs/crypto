# Sources checked (before code)

KB unavailable: `scripts/research-kb status` -> "research-kb: no executable found"; MCP `research-kb` not loaded.
Used `rg` over docs/, research/, RESEARCH_*.md instead (2026-11-20).

| ref | what it says | bearing |
|:--|:--|:--|
| `docs/ic/factor-base-encyclopedia/README.md` C5 (trace-zero) | V ⊂ ker Tr doubles subspace yield; as a target pre-filter it rejected 0 of 29,624 checks because odd-order targets lie in 2E (`research/ic_improve_20261014/03-tracezero-freerel/README.md`, PR #1862) | R1-bin already observed in production; R1 here supplies the theorem and exact small-m check. R3-bin asks the new question: the *other* 2^m − 2 functionals |
| same file, D3 `Z[τ]`-multiplier closure | orbits of α(P), α = a + bτ of norm ≤ N, as factor-base classes (S ≈ 0.94 at norm 64) | related to R4 (non-unit endomorphism orbits); there used for factor bases, not rho classes |
| same file, C11 orbit-valued quotient | Frobenius-orbit invariants as base: full degree, negative | R4: quotient invariants beyond Frobenius already negative for the base |
| `docs/ic/factor-base-encyclopedia/README.md` §P-192 2-Sylow halving | x-only point halving computes the 2-part | R1-2desc: 2-part information is cofactor information only |
| literature (from memory, not re-read here) | Kohel–Shparlinski (ANTS 2000) character sums over elliptic-curve subgroups O(√q); Boneh–Shparlinski (CRYPTO 2001) hard-core bits of ECDH; Duursma–Gaudry–Morain (ASIACRYPT 1999) and Wiener–Zuccherato (SAC 1998) automorphism classes for rho | R2 bound and R4 √#Aut speed-up; the constants are not relied upon, the exact numbers are measured |
