# A. Literature: index calculus in hyperelliptic Jacobians, genus 2..16

Status: IN PROGRESS (outline written first; sections filled as sources are read).
Date: 2026-11-17. Scope: DLP in Jac(C)(F_q), C hyperelliptic of genus g, 2 <= g <= 16.
Reference cost: Pollard rho ~ sqrt(pi #J / 2) ~ q^{g/2} group operations (less sqrt(2) with negation map, less sqrt(2g) style factors with Frobenius in subfield cases).

Convention: "verified" = statement read in the primary source (abstract or body) during this session;
"unverified" = from memory or secondary source, not re-read here.

## Outline
1. Gaudry (2000): degree-1 factor base, random-walk relation search
2. Enge-Gaudry (2002), Enge (2002): subexponential L(1/2), g > c log q regime
3. Large primes: Thériault (2003), GTTD (2007), Nagao (2007), Diem; graph handling
4. Factor-base reduction, optimal fraction, relation vs linear algebra balance
5. Enge-Gaudry-Thomé (2011) L(1/3), C_ab; relevance to hyperelliptic g=2..16
6. Sieving: Vitse-Wallet, Joux-Vitse, Laine-Lauter, sieve replacing random walk
7. Record computations
8. Genus 6..16: crossover with rho
9. Recent work 2018-2026 (incl. negative results)
10. Biggest speed-up levers by regime (table)

## Method and verification limits (read first)
- WebFetch could not reach eprint.iacr.org, arxiv.org or hal.archives-ouvertes.fr from this session
  (DNS refusal by the session network policy: these hosts must be allowed in the environment's network
  settings for primary-source PDF reading). Web *search* worked, so statements below are marked:
  - **[verified-snippet]**: the statement appears in an abstract/snippet returned by search for the primary source.
  - **[memory]**: from my knowledge of the paper; not re-read this session = *unverified* in the lab's sense.
- No research-kb MCP server or `scripts/research-kb` fallback was available in this checkout; no source_id recorded.
- All claims here are literature assertions: `[pcc:unverified]`.
