# Proposed atlas update: Iwasawa leads for prime-field ECDLP (applied in crypto-autoresearcher#3300)

Status (2026-11-15): applied in isogeny-labs/crypto-autoresearcher#3300 after mapping onto existing cards; cards 3, 4 and 5 became scoped evidence entries on existing leads, see REPORT.md §11 [pcc:unverified].

Target: `crypto-autoresearcher/knowledge/leads/ecdlp/`. This session could not read or edit the atlas.
The repository is vault-encrypted and `KEY` was not available. The card IDs below are therefore
proposals: the maintainers should map each one onto an existing card where one covers it, and
create a card only where none does. Source for every statement:
`research/iwasawa_leads_20261113/REPORT.md` (merged in isogeny-labs/crypto#4040 and #4146).
Every statement is an unverified assertion by the producing session [pcc:unverified].

## Knowledge-base receipt

- research-kb: **unavailable**. No MCP tool was exposed. The `scripts/research-kb` fallback found no
  executable and no `catalog.sqlite` in this container. No `source_id` was checked.
- Literature checked by web search (abstracts only): Huang–Raskind arXiv:0710.2363; Ulmer
  arXiv:1002.3313; Conceição–Hall–Ulmer arXiv:1307.4251.
- Until a KB sweep runs, the novelty status of these cards is unknown. A KB miss would not prove
  absence anyway [pcc:unverified].

## Cards

### 1. `LEAD-TRF-signature-calculus-euler-system` (transfer)

- **status:** closed (derivation)
- **obstruction.statement:** Every auxiliary prime ℓ of an Euler system for Ẽ[n] (Kolyvagin primes,
  n-admissible primes) has Frobenius eigenvalue ±1 on Ẽ[n]. So n divides ℓ + 1 ∓ a_ℓ ≤ (√ℓ + 1)²,
  which forces ℓ ≥ (√n − 1)² (Lemma A). The derivative classes therefore live over a ring class
  layer of degree Ω(n), and building one costs Ω(n) > √n.
- **forecloses:** producing Huang–Raskind global classes or signatures in fewer than √n operations
  via Euler systems.
- **does not foreclose:** other sources of global classes; the Huang–Raskind equivalence itself
  stands.

### 2. `LEAD-STR-function-field-iwasawa-tower` (structure)

- **status:** closed (derivation)
- **obstruction.statement:** For ℓ ≠ p, a global function field over F_q has exactly one
  Z_ℓ-extension, the constant one (Lemma C, class field theory: the ℓ-part of tame inertia is
  finite).
  - At ℓ = n the only tower is F_{p^{n^k}}(t), which is the embedding-degree route (MOV).
  - The ℓ = p (Artin–Schreier–Witt) towers see only p-primary Selmer groups, which are 0 here.
  - The GL₂(Z_n) division tower starts at a modular curve of level n, genus about n³.
- **forecloses:** non-constant Z_n-towers over F_p(t) as an ECDLP lever for prime n ≠ p.

### 3. `LEAD-TRF-xedni-dependent-lift-height` (transfer; update to an existing xedni / lifting card if present)

- **status:** closed (derivation, conditional)
- **obstruction.statement:** If Ẽ/Q has lifts with k·Q̃ = m·P̃ + T and n > 12, then k·x ≡ m (mod n)
  (Mazur). For uniform x, the probability that min max(|k|, |m|) ≤ √n/8 is at most 1/16. Hence
  ĥ(P̃) or ĥ(Q̃) is at least (n/64)·ĥ(R) (Lemma D). Under Lang's height conjecture, a dependent lift
  has coordinates with Ω(n) digits.
- **forecloses:** dependent lifts of polynomial size. This replaces the independence heuristic for
  xedni with a counting bound plus Lang.
- **open dependency:** unconditional lower bounds on ĥ(R).

### 4. `LEAD-TRF-padic-noncanonical-section-psi` (transfer; sibling of `LEAD-TRF-padic-canonical-section`)

- **status:** closed (measured null at toy scale)
- **statement:** The torsion wall kills homomorphisms and quadratic forms to Q_p. The invariant
  ψ(Q) = log(n·L(Q))/(n·p), with L the x-preserving Hensel lift, is neither, so the wall does not
  cover it.
- **evidence:** preregistered exact runs, p ∈ [2¹¹, 2¹⁴], 12 curves per statistic, anomalous and
  known-cubic positive controls, shuffled negative controls. Null for:
  - affine leakage (S1, S2);
  - zero carry cocycle (S3);
  - single-bit χ² (S4);
  - degree of ψ/y in x (S5), which is full on all curves.

  Protocols, tool and results are in `research/iwasawa_leads_20261113/r1_carry/`.
- **does not foreclose:** multi-bit or higher-dimensional leakage; cryptographic sizes.

### 5. `LEAD-IC-ulmer-specialization-factor-base` (ic-prime)

- **status:** closed (derivation)
- **obstruction.statement:**
  - Factor-base logs from specialised Mordell–Weil sections are equivalent to a basis of the
    specialisation kernel, which is the DLP (Lemma B).
  - For a uniform target, uneven images do not help: each candidate hits with probability 1/n, and
    the generic-group bound applies.
  - Ulmer's explicit points have x ∈ u₀·μ_{p^f + 1}, and μ_{p^f + 1} ∩ F_p* = {±1}. The Legendre
    family forces 4 | #E. So the structured factor base is empty for prime-order E/F_p.
- **forecloses:** global Mordell–Weil sections as a free-log factor base over a prime field.

## Not proposed

The eleven brainstorm ideas dismissed in REPORT.md §0 reuse obstructions that existing cards already
cover: canonical lift, Igusa tower, perfectoid tilt, constant tower, elliptic units, CM
parametrisation, class-group walk, volcano, rational isogeny, Cohen–Lenstra. Cite them, do not add
them.
