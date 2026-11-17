# hetaudit: desk audit of the hashed-factor-base tensor ECDL claim (2026-11-07)

## Goal
Independent audit (atlas lead `LEAD-EXO-hashed-factor-base-tensor-ecdl`, next_action step (a)) of Akshima et
al., *A Subexponential Time Algorithm for Elliptic Curve Discrete Logarithm* (Bochum v0.2, pin `1ee44e56`) and
of the Theorem D.1/D.4 dependency on the openai/math ETH preprint (pin `7ef5c0cc`): correctness of the
reduction, size of the Boolean encoding, whether D.1's hypotheses can hold at cryptographic size, the exponent
comparison against enumeration and rho, and whether the LEAD-ICB `omega(l sqrt(log l))` obstruction applies.
Full derivation: [AUDIT.md](AUDIT.md).

## Sources checked
* research-kb: NOT available here (`scripts/research-kb` has no executable); stated per the parallel rules.
* `python3 -I tools/lead_atlas.py --match …` and `build_frontier_map.py --match …` in the crypto-autoresearcher
  sparse checkout: both fail with `ModuleNotFoundError: orchestration` (the sparse checkout lacks the package);
  no atlas hits could be recorded by tool. The two lead YAMLs were read directly.
* Manuscript: `bochum-crypto/subexponential-ecdl` @ `1ee44e562b1b9a50ea339f3b8f81112b5a24b96f` (cloned; HEAD is
  the pin; one PDF). Text extracted with pdftotext: `sources/bochum.txt` (1,248 lines, read in full).
* Preprint: `openai/math` @ `7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a`, `preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/build/{quantitative-reduction,quantitative-contraction}.tex`
  (sparse clone; HEAD and pin identical on that directory). Theorem D.1 and Appendix D.4 read in full.
* Copies and hashes: `sources/SHA256SUMS` (PDF `82e27362…`, contraction tex `671642d7…`).
* Repo notes: `knowledge/leads/ecdlp/exotic/LEAD-EXO-hashed-factor-base-tensor-ecdl.yaml`,
  `…/ic-binary/LEAD-ICB-decomposition-encoding-and-sat.yaml`, `knowledge/literature/KN-LIT-{caa51d,eb8749}.md`,
  `docs/reports/ic-prime-eth-leads-20261102/{clustering_closure,planted_leak}.md`,
  crypto `problems/hashed-ecdl-tensor-backend.md`, `research/ic_prime_eth_leads_20261031/{LEADS,SYNTHESIS}.md`,
  `leads/SKEPTIC_DIGEST.md`, `leads/L06/REPORT.md`, `leads/L09/{REPORT.md,table2.json}`,
  cryptanalysis `docs/hashed-ecdl.md`, `experiments/hashed-ecdl-tensor-reference/prime-development/challenge-tensor.{txt,json}`,
  crate `crates/hashed-ecdl-tensor/src/{encoding,reduction,admission}.rs` (both repos; `diff -rq` shows crypto
  has 8 extra source files, so "identical sources" in the atlas is out of date).

## What was done
1. Read the manuscript in full and Appendix D.1/D.4 of the preprint from the pinned sources (first time either
   text is pinned in the repos; LEADS §9 erratum 5 and L09 recorded them as "not on disk").
2. Re-derived Lemmas 1–13, Theorem 3, Corollaries 1–2 and Remark 1 (AUDIT.md §5); checked Lemma 5's integer
   inequalities for d ≤ 5000 and the principal-root claim exactly; counted the Theorem 3 network as built.
3. Derived s(m, b) from the paper's own gadgets and cross-checked against the implementation receipts
   (ECCp-131: degree 1,032 reproduced to the unit from J = 128, L₁ = L₂ = 4; edges per slot-bit 5.3k floor vs
   5.97k recorded).
4. Re-derived the D.4 admission arithmetic at depth 0 and at full depth; reproduced the L09 margins.
5. Exponent comparison at b = 128/192/256 for a ∈ {1, 2600, 6000}, c = 1 (`checks/hetaudit_checks.py`, 0.05 s,
   output `checks/hetaudit_checks.out`).
6. Wrote AUDIT.md with every step labelled.

## Results (evidence class)
* **Correctness** [proved here]: no error in the reduction. Relations found are genuine group identities;
  hash injectivity, coverage second moment (constant 0.4702 > 2/5), cofactor clearing (A = [c]S lies in the
  r-Sylow subgroup, count inflated by c^m, tolerated), extraction (V uniform, ξ = −U/V) all hold. Two
  presentation slips: footnote 4 writes u₀ = ⌊b/2^17⌋ (should be λ = ⌈log₂ s⌉ of the network, not b = ⌈log p⌉);
  the limitations paragraph's 2^{2^17} is the u₀ ≥ 1 condition, not the proof's stated regime.
* **Encoding size** [proved here, checked numerically]: s = a·m·b, linear in b and m, with a ≥ 5.3·10³ edges
  per slot-bit at b = 131 from the manuscript's own gadget list (≥ 46 integer multiplications per point
  addition, 15,230 edges each as built); implementation 5,971 [pcc:unverified receipt]. Degree
  2·max(L₁,L₂)(J+1): 1,032 at b = 131, 2,056 at b = 256.
* **Admission** [proved here, checked numerically]: recursion needs λ ≥ 2^17 (s ≥ 2^{131071}); the proof's
  stated bound λ^{4/5} ≤ u needs λ ≥ 2^85 at depth 0 and **λ ≥ 2^510 at the deepest active call**; below, D.1's
  algorithm is direct summation 2^s (2^{1,564,506} at ECCp-131; receipts say `root_u = 0`). Clustering note
  item 1 confirmed but incomplete (depth-0 only).
* **Exponents** [checked numerically]: with D.1 taken as c·s/√log₂ s, c = 1, the composed algorithm is above
  rho by ≥ 120 bits at b = 128–256 even at the impossible a = 1, and at ~10⁵ bits with the paper's constant;
  below rho only for log₂ b > 256(ac)² (> 2^{256}-bit groups at a = c = 1, log₂ b > 1.7·10⁹ at a = 2,600).
  Linear algebra alone exceeds rho for every m ≤ 6. Beats neither enumeration nor rho.
* **LEAD-ICB obstruction** [proved here]: does **not** apply; s is linear in l = b/m, so for fixed m the
  counting bound asymptotically beats per-attempt enumeration. The closure comes from admission and constants.
* **Verdict**: (ii) the asymptotic claim stays conditional on exactly Theorem D.1 (openai/math @ 7ef5c0cc,
  App. D.4, unspecified constants); and (i) the lead is closed as a research direction under the cost model
  "composed exponent vs rho b/2 and vs per-attempt enumeration, D.4 parameters as published".

## Decisions made (and why)
* Cloned the two public repositories read-only (add_repo said anonymous read is served) instead of auditing
  from the repo's notes only: the task's audit cannot be done from paraphrases, and the notes themselves say
  the texts were not on disk.
* Did not build or run the crate's tests (desk audit; 34/37 tests are producer receipts and the shared machine
  rule prefers no builds without need). Said so.
* Treated every repo number as `[pcc:unverified]` and re-derived rather than trusted where possible.

## Open threads
* Correctness of the D.4 proof itself (precision bookkeeping, mode identity (D.16), packet degree invariants)
  is still unaudited by anyone in the program; out of scope here.
* Atlas text to correct: next_action (a) wording about the ICB obstruction; "2^85" → "at least 2^85, 2^510 at
  full depth"; "identical sources" (crypto crate has diverged); "34 tests" (now 37 in cryptanalysis).
* The ECC2K-130 companion graph reuses the integer Fourier gadget on a binary curve; whether it encodes a
  correct binary relation is not claimed or audited anywhere I read.
* Step (b) of the lead (84-case comparison panel) is unaffected and still blocked as recorded.

## Errors
* `lead_atlas.py --match` / `build_frontier_map.py --match` crash (`ModuleNotFoundError: orchestration`) in the
  sparse checkout; recorded, not fixed (read-only checkout).
* A repo-wide grep for the source names ran past the 120 s limit (many large experiment trees); its output was
  only used to confirm the file list already found by direct `ls`.
* research-kb unavailable; Conductor not installed (not attempted).

## Ready-to-run
Nothing to run. The only executable artifact is `python3 -I checks/hetaudit_checks.py` (0.05 s, exact
integer checks; output committed in `checks/hetaudit_checks.out`).

## Shared files edited
None. All writes are inside `research/hashed_ecdl_tensor_audit_20261107/`. Clones live outside the repos at
`/home/user/bochum-crypto/subexponential-ecdl` and `/home/user/openai/math` (sparse); copies with hashes are in
`sources/`.
