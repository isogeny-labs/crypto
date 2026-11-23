# Lead-atlas delta from this study (to apply in crypto-autoresearcher)

The atlas (`isogeny-labs/crypto-autoresearcher`, `knowledge/leads/ecdlp/`) is vault-encrypted
(AES-256-SIV, KMS-unlocked with an AWS identity). The session that produced this study had no
AWS identity, so it could neither read nor edit the lead files; the sparse read-only clone shows
ciphertext (`VLT1` header) for both files named below. This note records the edits a session with
vault access should make. Every number is `[pcc:unverified]` and points at `rows/*.json` here.

## `knowledge/leads/ecdlp/exotic/LEAD-EXO-hashed-factor-base-tensor-ecdl.yaml`

- **best_measured / evidence**: add
  `isogeny-labs/crypto research/semaev_tensor_treewidth_20261111` (PR #3941, merged 2026-11-12):
  treewidth census of the Bochum network and three comparison languages on toy prime-order
  curves, b = 8–40.
  - Bochum network (Fourier gadget, hashed group law, m = 2): width lower bound 197 (b = 8),
    206 (12), 389 (16), 404 (20), 405 (24); min-degree upper 263–870. The transform vertex is a
    clique of 132 / 260 / 520 variables, so the width is set by the multiplier, not the relation.
  - Same language with schoolbook multipliers: lower 27 → 104, best upper 92 → 1,157 over
    b = 8 → 40. Lower-bound slope 2.4 per bit against 0.5 per bit for enumeration/rho (b/2).
  - Exact Boolean contraction of any of these networks therefore costs at least 2^(≈2b) against
    2^(b/2): the encoding, not the solver, is the bottleneck. The D.1 exponent s/√log₂s is
    30–150× the best measured width, so the generic bound is loose by two orders of magnitude
    and closing it would still lose.
- **next_action (a)**: mark done (audit `research/hashed_ecdl_tensor_audit_20261107`, verdict
  (ii)+(i)) and replace the text "check whether the obstruction in LEAD-ICB applies: #SAT exponent
  outgrows enumeration once the CNF has ω(l√log l) variables" with: "it does not apply (the
  Bochum encoding is linear in l); the closure rests on admission (λ ≥ 2^17, stated bounds from
  2^85 at depth 0 and 2^510 at full depth) and on constants; and, independently, on width: the
  network's treewidth is ≥ 2b at every measured size (this study)".
- **revisit_when**: add "a bit-level encoding of a b-bit modular multiplication with primal
  treewidth below b/2 (none is known; the schoolbook and Fourier gadgets measure ≥ 2b and
  ≥ b^(3/4)·const respectively)".
- **mechanism**: the crates in `crypto` and `cryptanalysis` are no longer identical sources
  (audit §7.5; `crypto` now also carries `relations.rs`, `treewidth.rs`, the multiplier switch and
  the `tensor-treewidth` binary).
- **Semaev variant** (the owner's follow-up question): encoding S3 (m = 2) or chained S3 (m = 3)
  as a constraint graph on the small-x factor base gives 3.7–5.3× fewer edges and 20–30% less
  width than the hashed group law at equal b, with the same slope: constants, not a regime.
  Disposition: closed by this route; the remaining lever is field-level elimination (see
  `FIELD_LEVEL.md` in this directory once landed).

## `knowledge/leads/ecdlp/ic-binary/LEAD-ICB-decomposition-encoding-and-sat.yaml`

- In the paragraph that states the ω(l√log l) obstruction against counting solvers: name the
  Bochum gadget (edges linear in l) as the exception that the obstruction does not cover, and
  record that it is instead killed by admission and constants (audit) and by width (this study:
  treewidth ≥ 2b for bit-level encodings of the relation, Fourier or schoolbook).
- Cross-reference `KN-LIT-caa51d` with the same correction.

## Literature notes

- `KN-LIT-caa51d` ("Against enumeration" paragraph): correct for Tseitin encodings; add the
  Bochum exception as above.
- `KN-LIT-eb8749`: no change needed (Lemmas 10–12 were re-read in the audit; no error).
