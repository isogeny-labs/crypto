# Resume: crypto @ aburan28/loving-heisenberg-rbzzbu

## Goal
Run the 2-isogeny arm of the PKM prime fall-degree study at `P1 = 2013265921`, `t = 1..13`, on a reserved RunPod CPU pod.
Score predictions P-ISO-1 and P-ISO-2: does the isogeny ladder stay "flat at 5" at `t = 9..13`, where Kummer reaches 6?
Lead: `LEAD-ICP-pkm-tower-algebraic-oracle`. Every number is `[pcc:unverified]`.

## Lead / hypothesis
P-ISO-1: the isogeny `P1` medians at `t = 1..8` equal `P0`'s (3, 5, 5, 5, 5, 5, 5, 5).
P-ISO-2: at `t = 9..13` the isogeny `P1` medians stay at 5.
Exact rules: `research/pkm_falldegree_unblock_20261107/AMENDMENT_R4_ISOGENY_P1.md` §5, scored by `iso/score_iso.py`.

## Spec
- Preregistration: `research/prime_fall_degree_20261027/PREREGISTRATION.md`, Amendments 15 (route) and 16 (arm), frozen in
  commit `6ef033c4f4`.
- Job set: pins in `iso/PINS.json` and `iso/BUNDLE.sha256` (hashes named in Amendment 16); argv in `iso/entry_iso.sh`.
- Runner: `runpod/server.py`; client `runpod/rp.sh`.
- Binary: `pkm_tower_pilot_r4` `705b151b…`, a musl build of f13a690df0 plus `gaux/aux_order_f13a690df0.patch`; recipe in
  Amendment 16 and `iso/PINS.json`.

## State
- Gate G-AUX passes (`gaux/v3/`).
- Self-tests pass: certifier, recount, scorer (cases A–F, H).
- Local integration of `g1 gaux` passes on pre-guard runners; a tampered pin is refused.
- Owner decisions, 2026-11-14 ("Yes to all"): decision 4's ceilings (US$40 per job, US$400 for the study) apply to reserved
  pods, and the RunPod balance covers the spend.
- S3 replaced by git (owner decision 12, Amendment 17). 16-vCPU pods and a 120,000 s limit (Amendment 18).

## Next steps
1. With S3 working: upload `705b151b…` to the artifact store; push this checkpoint; `benchrec.py init` a record for `iso-a`.
2. `TOKDIR=<scratch> runpod/rp.sh create pkm-iso-a 12`. Check `costPerHr ≤ 0.769`, memory ≥ 56 GiB and the image.
3. `rp.sh upload-iso <id> <bindir> <commit>`, then `SECONDS_LIMIT=172800 rp.sh start-iso <id> '["g1","gaux","A","A-big","DA","g3","cert","cap-A"]'`.
4. Check in at about 50 min, then about 4 h: `rp.sh status` and `rp.sh fetch`. Delete the pod at the first check-in after it ends.
5. Run `iso/recount.py` with a naive_count build; run `iso/score_iso.py`. Run the X bracket jobs only where `d_F` is undecided.

## Runs in flight
As of the 2026-11-15 05:15Z check-in. Fetches are under `results/ISO/<job>/` and `results/KUMMER/<job>/` (Amendment 17).
- `iso-a`, pod `w3ao0j8mfsiasz`: `g1 gaux A A-big` done; on `DA` (F4, `t = 1..13`).
- `iso-14`, pod `91y9xgsnne97pj`: **done**, exit 0, 04:36:56–04:43:21Z. **Pod deleted.** Recount of the `t = 14` curve
  running; no reading before it passes.
- `pfd-x-t13-k0-c6-rp`, pod `q7scp0zg6ic3oj`: E3.7 control passed; on `cap:13:0:6`.
- `pfd-b-rp-1`, pod `agyp0kbpguz1ss`: control and `g1` passed; on `hky-B` (`B-m3-tower`).
- `pfd-b-rp-2`, pod `ug0ohhn9h6pdv4`: control and `g1` passed; on `d-B` (`DB-m3-tower`).
- Spend so far (derived, US$1.04/h per pod): about US$2.5 at 05:15Z. Ceiling at the limits: US$176.78 of US$400.
- Pod tokens are in the session scratchpad, not in git.

## Stop and success conditions
Stop on: any gate failure (G-AUX, G1, G2, G3); a Mestre panic; a scorer STOP (exit 2); a recount mismatch; a cost refusal.
Success: P-ISO-1 and P-ISO-2 read under the frozen rules, with every row certified and recounted. Any reading is a result;
none is an improvement claim.

## Open questions
- The Kummer retries (`pfd-x-t13-k0-c6`, stage B) and the `t = 14` and `m = 4` cells: the owner has not decided whether to
  run them on this route.
