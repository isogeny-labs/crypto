# Artifact-access CI receipt

Observed PR4357head43972040e7c2c7cef38723d5c86eb586ebfb845d on2026-11-16.

- backend-leaderboard/current: https://github.com/isogeny-labs/crypto/actions/runs/43918147338/job/133477819208
- ic-leaderboard-recompute/recompute: https://github.com/isogeny-labs/crypto/actions/runs/43918147255/job/133477817510

Both completed with failure in the step that pulls required stored artifacts. Logs show HeadObject HTTP403 Forbidden and artifact downloader exit2. This receipt does not claim that subsequent validation would pass after access is restored; it records where these runs stopped. No CI gate is waived and no unchanged workflow rerun is requested while access remains denied.

The PR-description edit initially hit a GitHub TLS handshake timeout. Live-body read confirmed unchanged content; retry succeeded. This does not affect source or scientific execution state. Original frozen experiment remains unexecuted and all scientific output paths remain absent.
