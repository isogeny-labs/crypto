# R4 route integration test (local), 2026-11-14 18:48–18:50Z

Measured. Not data. No `P1` row was produced.

- `runpod/server.py` (commit `1e1e0f596b`, the runner's `iso` mode) ran on the session sandbox on 127.0.0.1:8000.
- **Uploads**, SHA-256 as reported back by the runner:
  - `pkm_tower_pilot_r4` `705b151b6f35…`
  - `prime_fall_degree_tools` `7e8a67030e8d…`
  - `hky_fp_tower_check` `c36d324e8a60…`
  - `repo.tar` `2f59cfa727c2…`, which is `git archive 1e1e0f596b $(cat iso/BUNDLE_PATHS)`
- `/start {"entry": "iso", "stages": ["g1", "gaux"]}`. The runner checked every hash against `iso/PINS.json` plus `repo.tar`,
  unpacked the bundle at `/opt/repo`, and ran `iso/entry_iso.sh` with 12 rayon threads.
- **Result.** State `done`, exit 0. `progress.txt`:
  - `g1` exit 0
  - `gaux` exit 0, 18:48:49 to 18:49:46Z
  
  `gates/gaux.log`: **"pass 16 draws, 0 mismatched"**.
- The runner's local files (`/results`, `/work`, `/opt/repo`, three binaries in `/usr/local/bin`) were left on the throwaway
  sandbox. Removing them was refused by a safety check; they leave with the container.

## Second run, with bundle pins (re-review edit R2-E5), 2026-11-14 19:08–19:09Z

Commit `172ce02e2894`; `repo.tar` `d869e98482e3…` (`git archive` of `iso/BUNDLE_PATHS`); pins from `iso/BUNDLE.sha256` at that commit.

- **Negative control.** The same request with the pin of `iso/score_iso.py` replaced by `00` was refused: HTTP 409,
  `{"error": "bundle sha256 mismatch", "detail": {"…/iso/score_iso.py": ["00", "b89aa304ea0b…"]}}`. Nothing ran.
- **True pins.** State `done`, exit 0. `g1` exit 0; `gaux` exit 0, 19:08:42 to 19:09:23Z.
  `gates/gaux.log`: **"pass 16 draws, 0 mismatched"**.
- `progress.txt` also holds the first run's four lines, because `/work` from the first run was not removed (see above).
