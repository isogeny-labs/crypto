# iso-a, fetch 1 (2026-11-15 04:23Z, job still running)

Measured. `[pcc:unverified]`. **Partial and unscored.** These rows have not been through the certificate, G2, G3, the
independent recount, or `score_iso.py`. No reading may be taken from them.

- The manifest check passed: every file matches the pod's own `/manifest` (`manifest.json`). Tarball SHA-256 in
  `tarball.sha256`.
- Host (`host.txt`): AMD Ryzen Threadripper 7960X, `nproc` 16, rayon 12.
- G1: pass. G-AUX on this host: "pass 16 draws, 0 mismatched" (`gates/gaux.log`, `gates/gaux/gaux_check.json`).
- `runs/A-m2-iso-p1.jsonl`: closure rows for `t = 1..9` so far, both draws, every row with `aux_order_method: mestre`.
