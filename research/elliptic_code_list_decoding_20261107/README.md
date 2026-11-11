# Elliptic-code list decoding as a relation finder (ecld), 2026-11-07

Lead: `LEAD-ICP-elliptic-code-list-decoding` and twin `LEAD-EXO-elliptic-code-list-decoding` (crypto-autoresearcher `knowledge/leads/ecdlp/{ic-prime,exotic}/`). Every number below is `[pcc:unverified]`. Evidence class is stated per item: proved (from a cited theorem and a short argument), derived (counting under a stated model), checked (development check at tiny size, not a measurement), planned (frozen, not yet run).

## Goal

Decide the lead as far as reading plus a frozen toy design allow: (1) read Zhang-Liu and Cheng; (2) decide on paper whether a list-decoding relation finder supplies about k guessed agreements; (3) freeze a toy protocol (p ~ 2^16, B ~ 2^8, k = 2..6, decoder arm vs enumeration arm); (4) implement it and check it on tiny cases; (5) propose an atlas disposition change.

## Sources checked

- research-kb: NOT available in this session (`/home/user/crypto/scripts/research-kb` is not executable). No KB query was made; this is a gap.
- `tools/lead_atlas.py --match` and `tools/build_frontier_map.py --match` in the crypto-autoresearcher checkout: both fail at import (`ModuleNotFoundError: orchestration`; the sparse checkout lacks that package), even with `sys.path` extended. No hits recorded from them. Instead I read the two lead YAMLs, `knowledge/literature/KN-LIT-643.md` and the 2026-11-04 desk report `docs/reports/ecdlp-leads-desk-20261104/elliptic-code-desk.md`, and grepped the leads tree: the lead appears only in the two YAMLs and `notes/exotic-leads-20261103.md`.
- Conductor: not installed here; not installed by me.
- Zhang and Liu, "Solving ECDLP via List Decoding", ePrint 2018/795 / ProvSec 2019: full text NOT obtained. eprint PDF behind a Cloudflare challenge (HTTP 403), Springer chapter paywalled, Wayback blocked by the session proxy, dblp bot check, Semantic Scholar 429. None bypassed. Only the abstract was read. Full fetch table: `sources/NOTES.md`.
- Cheng, "Hard Problems of Algebraic Geometry Codes", arXiv cs/0507026 v1: fetched and read in full (`sources/raw/cheng_arxiv.pdf`, text in `sources/cheng.txt`). The 2008 IEEE version was not obtained. Exact statements and line locators: `sources/NOTES.md`.
- Guruswami-Sudan agreement bound t > sqrt(n deg G): cited from the 2026-11-04 desk report (which quotes the 1999 abstract), not re-read here.

## What was done

1. Fetched and logged all source attempts; read Cheng. Zhang-Liu remains unread.
2. Desk analysis of the mechanism class (section Results, items R1-R4).
3. Wrote and froze `PROTOCOL.md` before any panel data.
4. Implemented `ecld_toy.rs` (std-only Rust, about 840 lines after the formatter rewrap): curve and prime-order search, factor base, Cheng's code and received word, three exact arms (E1, E2, D1), brute-force D0 for checks, `bound`, `plan`, `panel` modes. Built with `rustc -O` (no Cargo changes). Development checks run at p in [200, 1100) and a smoke panel at p ~ 1000. A single tiny p ~ 2^16 calibration instance (k = 4, B = 48, 2 targets) was run only to read unit costs; it is not part of the panel and is not evidence.
5. Wrote the job list `ecld_jobs.txt` (64 independent argv lines) and a cost estimate.

## Results

R1 (proved, from Cheng Lemma 3 and the pole-order count). For the code ev_D(L((k-1)O)) and Cheng's received word R = ev_D(f'), every codeword has agreement at most k with R, and the codewords with agreement exactly k are in bijection with the k-subsets of D that sum to Q (each subset gives exactly one codeword once f' is normalised to coefficient 1). So the output of any exact list decoder at agreement k is the full relation set: decoding and relation finding are the same problem.

R2 (proved given the cited GS bound t > sqrt(n deg G)). For genus 1, deg G = m = k-1, the GS guarantee radius is t_GS = floor(sqrt(B (k-1))) + 1, which exceeds k as soon as B >= k^2/(k-1) (for B = 2^8, k = 2..6: t_GS = 17, 23, 28, 33, 36 against k = 2..6; `results/gs_bound_table.txt`). Since no codeword has agreement above k (R1), the GS list on Cheng's received word is EMPTY on every instance, not merely "unlikely to reach" the codewords. This is stronger than the lead's H1 (which says GS cannot reach them); it is a theorem-level statement for this received word. Brute-force check (checked, tiny size): the maximum agreement over all p^(k-1) codewords equals k when a relation exists and never exceeds k (`results/dev_check.txt`).

R3 (derived, random-word model; heuristic). For any received word w and any one-point code of dimension m, the expected number of codewords with agreement >= t is about C(B,t) p^(m-t). Evaluated at the GS radius t = t_GS(m) over all m < B, with B = 64, 128, 256 and log2 p = 16 (and 20 for B = 256), the maximum of log2 of this expectation is -10, -9, -8 (and -12): always below 0, attained at m = B-2, t = B-1 (`results/gs_bound_table.txt`). So in the random model GS never returns a nonempty list at any dimension below B, which is the exotic twin's predicted "hits only near k ~ B"; at those dimensions the code is nearly the whole space and nothing is being decoded. Caveat: this treats codeword agreements as independent with probability 1/p; the exact structured statement is R1/R2.

R4 (derived; this answers "is it enumeration in disguise"). Since decoding at agreement k returns exactly the relations (R1), the only question is the cost of an exact algorithm. The natural one (information-set decoding) guesses k-1 of the k agreeing positions, interpolates the unique codeword through them (a (k-1)x(k-1) system, singular only when the k-1 points sum to O, which cannot occur inside a relation unless a point equals Q), and finds the k-th agreement by a scan. That is enumeration of (k-1)-subsets with a heavier step than the group addition plus one hash lookup of arm E1: it needs C(B,k-1) interpolations, i.e. about k-1 guessed agreements, and a meet-in-the-middle (arm E2) needs only about C(B,k/2). Whether some cleverer exact decoder avoids k-1 guesses is not excluded by this argument. What is excluded is the polynomial-time GS route (R2). The statement "the Zhang-Liu received word needs about k guessed agreements" is NOT decided: their received word and decoder are unread. R1-R2 do show that if their received word is of Cheng's kind, the GS part of the pipeline returns nothing and all the work would sit in the guessing, which fits the author's own slide ("not efficient currently ... not of square-root time").

R5 (checked, tiny size, development only). `results/dev_check.txt`: for k = 2, 3, 4 at p = 881, 1009, 251 (prime group orders 863, 1049, 257), E1 = E2 = D1 = D0 as sets, all relations verify by independent group-law summation, planted relations recovered, max agreement over the whole code <= k, singular systems logged (10 at p = 251). Smoke panel (`results/dev_smoke_panel.jsonl`, 30 rows, p ~ 1000, k = 2..6): sets agree across arms, all verified, planted recovered, A/A identical. Unit costs from the single p ~ 2^16 calibration instance (`results/dev_calib_unit_cost.jsonl`): about 80-220 ns per counted group addition (E1), about 4.3 ns per D1 native operation. These are practicality notes, not measurements.

R6 (derived; relation-phase planning numbers, `results/panel_cost_plan.txt`). Expected relations per random target at p ~ 2^16 (taking N ~ 65000): k = 3, B = 256: about 43; k = 4: 2.7e3; k = 5: 1.4e5; k = 2: 0.5. Planned counts per instance at B = 256: E1 about 6.5e4 (k=3), 5.6e6 (k=4), 3.5e8 (k=5) group additions; E2 about 6.5e4, 9.8e4, 5.6e6; D1 about 1.4e7, 1.3e9, 9.5e10 native operations and zero group additions. Crossing weight w* = E1 gae / D1 native is about 4.5e-3 at k = 3, B = 256, so D1 is predicted to lose at w = 0.1 and w = 1. The nominal matched rho at this size is about sqrt(pi N / 4) ~ 225 group additions; E2 at k = 3, B = 256 spends about 1.5e3 additions per relation, so collecting B+1 relations is about 1700 times rho (derived, relation phase only, known factor-base logs; a distance-remaining figure, not a leaderboard row).

Not decided: Zhang-Liu's actual procedure, and any measured D1-versus-enumeration cost (panel not run).

## Decisions made (and why)

- Did not implement Guruswami-Sudan. R2 shows that on Cheng's received word a correct GS decoder returns the empty list, so an implementation could add nothing; the exact list decoder D1 (outputs all agreement-k codewords) is the strongest decoder any list-decoding scheme could be, and tests the mechanism class independently of Zhang-Liu's received-word choice.
- Used information-set interpolation (D1) instead of brute force over the p^(k-1) codewords for the panel: brute force needs 2^16 to 2^80 codewords for k = 2..6 and is only feasible in `check` at p < 1100.
- Chose E2 (meet in the middle) as the reference enumeration, not only the naive C(B,k) count, so that "D1 loses" cannot come from a weak baseline.
- k = 6 at B = 256 is outside the panel for D1 (about 2e12 native operations per instance); covered by smaller B and the fitted exponent, labelled extrapolation.
- Priced D1 under the section 5a band, and declared the w = 0 end degenerate (D1 performs no group additions, so it is priced at zero there) rather than letting it count as a win.
- Did not attempt any further route to the paper (no spoofed agent, no mirrors beyond a single Wayback attempt that the proxy blocked), per the no-bypass rule.

## Proposed atlas disposition change (for the Coordinator; NOT applied)

1. `LEAD-EXO-elliptic-code-list-decoding`: propose `closed_obstructed`, kind `proof` for the mechanism as the twin states it ("Guruswami-Sudan on an evaluation of L(M O) as a point-decomposition oracle"), citing R1-R2 (theorem-level for Cheng's received word) and R3 (heuristic counting for any received word). Its own discriminator, "non-empty lists at k << B would refute the bound", is met on the predicted side by the counting bound (R3) and by the tiny brute-force check (R5); the frozen panel would add measured confirmation but is not needed for the closure of the GS-oracle claim. Scope it explicitly to GS-type decoding and to received words of the Cheng form; it must not claim to close Zhang-Liu's procedure.
2. `LEAD-ICP-elliptic-code-list-decoding`: keep `open_next_step`, narrowed. Replace the heuristic H1 by the stronger R2 (GS list provably empty on Cheng's received word), and split next_action into (a) read Zhang-Liu (blocked: needs a PDF supplied by a person with access; the lead's own first step), and (b) run the frozen panel in `PROTOCOL.md` (ready, about 70 CPU-minutes). Decision rule (b) has three outcomes: enumeration in disguise (closure with the scope of LEAD-ICP-generic-table-decomposition for the exact-decoder class), below enumeration (charged comparison against matched rho), or unresolved. Prediction on record: enumeration in disguise.
3. Add to `forecloses` of the ICP lead nothing until (a) or (b) completes; add to `evidence` this directory and `sources/NOTES.md`.

## Open threads

- Zhang-Liu full text unread. Needs a person with PDF access to place it in `sources/` (the Springer chapter, or ePrint 2018/795 v-final). Until read, the statement about their received word and stated complexity stays open.
- Panel not run (by instruction). Random-word control and a k = 6, B = 256 enumeration anchor are not implemented (listed in `PROTOCOL.md`).
- Cheng 2008 IEEE version (the one-point case "left to the full paper") not obtained; arXiv v1 only.
- research-kb and the atlas query tools could not be used; no KB negative-search done.
- GS bound taken from the earlier desk report, not re-read in the 1999 paper.

## Errors

- `lead_atlas.py` and `build_frontier_map.py` failed (missing `orchestration` package in the sparse checkout); worked around by reading the YAMLs directly.
- eprint PDF 403, Wayback 403 (proxy), dblp bot page, Semantic Scholar 429; handled by logging and stopping, no circumvention.
- The source file was rewrapped on disk by something other than me during the session (line count rose from about 560 to 843, behaviour unchanged; `check` re-passed afterwards). The frozen hash in `PROTOCOL.md` is of the final file.
- Plan formula for D1 and E2 counts was wrong on the first version (E2 overcounted ~2-3x, D1 ~1.5x); corrected after comparing with measured tiny-case counters (k = 3, B = 24; k = 4, B = 48). The measured counters match the corrected plan within the checked cases.

## Ready-to-run

Nothing was launched. Artifacts: `PROTOCOL.md` (frozen), `ecld_toy.rs` (sha256 `de901e230d2558cb488dee653e8bdfe44ff44b928a611322a014670c5fa8769d`), `ecld_jobs.txt` (64 argv lines; the 8 for k = 5, B = 256 are split by curve and target, about 8 minutes each), `results/panel_cost_plan.txt`.

Estimate (planned): total about 70 CPU-minutes on one core, split as D1 about 60 minutes (almost all in k = 5, B = 256), E1 about 10 minutes, E2 under a minute; memory under 200 MB (E2 tables at k = 6, B = 64 are small); output a few hundred KB of JSONL. Each job is far under the 3600 s command-job cap.

What a launch still needs (not available now, so no `isolab.eks.command-request/v1` JSON was written; it could not pass `validate-request` without a commit SHA and an image digest):
1. Main session commits this directory; director freezes a command card per `.claude/skills/isolab-eks/SKILL.md` (digest-pinned image containing the binary built with `rustc -O` from the committed `ecld_toy.rs`; compare its sha256 with the one in `PROTOCOL.md`; each job's stdout uploaded to S3 by the command, since command jobs have no automatic result upload).
2. `scripts/checkpoint.sh init` for `.resume/RESUME.md`; `python3 tools/benchrec.py init ...` only if the Coordinator treats this as a benchmark (it is a deterministic operation-count study; wall time is a note, so it is arguably not a benchmark, but the hooks decide).
3. The executor runs each line of `ecld_jobs.txt` unchanged (replace `ecld_toy` by the image path of the binary); a failed or preempted line is re-run whole.
4. Director assembles `experiment.export/v1`, checks P1-P5 in `PROTOCOL.md`, applies the decision rule. Local re-check commands (development only): `ecld_toy check`, `ecld_toy bound`, `ecld_toy plan`.

## Shared files edited

None. New files only, all inside this directory: `README.md`, `PROTOCOL.md`, `ecld_toy.rs`, `ecld_jobs.txt`, `sources/` (raw fetches, `cheng.txt`, `NOTES.md`, `SHA256SUMS`), `results/` (development outputs). `bin/ecld_toy` is a local build product and should not be committed.
