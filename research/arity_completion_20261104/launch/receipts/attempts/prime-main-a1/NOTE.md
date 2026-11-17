# Prime-tower main attempt 1 (Job arity-prime-m5-main-20261104): stopped at the controls gate

- Launched by CARD-arity-launch-prime-main (launcher 367b7870); Job d376c6cc,
  pod eed0517c, 16:11:57Z–16:21:09Z; pod exit 31 ("controls verdict failed;
  no m = 5 cell run"). All uploaded files verify against SHA256SUMS.
- Controls (Kummer, p1 = 2013265921, two random targets each):
  - m = 4, N = 8: D = 6, 6 (recorded 6) — ok
  - m = 3, N = 15: D = 7, 7 (recorded 7) — ok; 551 s, peak RSS 1.21 GB
  - m = 2, N = 10: D = 4, 4 (gate expected 5) — MISMATCH
- The gate's 5 is a preregistration error. The recorded Kummer m = 2, t = 5
  (N = 10) rows give solving_degree_max = 4 on both random targets at
  p = 786433 (`research/pkm_tower_round2_20260925/runs/XV1-m2-tower-null.jsonl`)
  and p = 3221225473 (`.../XV5-kummer-m2-p32.jsonl`); D = 5 at N = 10 is the
  isogeny tower's value (XV1), and Kummer reaches D = 5 at N = 12. The
  measured 4, 4 equals the recorded Kummer rows.
- Cosmetic defect: `cell-exits.jsonl` "class" strings read "complete\ninconclusive_cap"
  (a shell precedence slip in job-prime-tower.sh); the "exit" fields are correct.
