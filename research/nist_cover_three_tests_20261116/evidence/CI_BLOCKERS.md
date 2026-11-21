# PR4403 prior-head CI observations

First inspection2026-11-16 approximately05:41UTC, prior head f5472700fbf4b1eab4436406d3e9a2a22f230295. These are observations from GitHub CI, not automatic approval rejections. [pcc:unverified]

- [artifacts job133108400981](https://github.com/isogeny-labs/crypto/actions/runs/43809687003/job/133108400981): data-file check says `no data files committed`; workflow wiring check reports32 workflows missing/stale pull steps. Examples include `compact-orbit-n61-isolated.yml` and `ic-autolab.yml`.
- [records job133108334945](https://github.com/isogeny-labs/crypto/actions/runs/43809687031/job/133108334945): `benchmarks/records/20261115T0609Z-genus-ladder-sge-scale.json` is still open; global `benchmarks/progress` report is stale.
- `git diff --name-only origin/main -- .github/workflows tools/artifacts benchmarks/records benchmarks/progress` returns no changed paths for this lane. These services/files are outside the study's diff. No gate or record was altered to bypass their failures. [pcc:unverified]

The required artifacts check blocks merge until the shared wiring is corrected. These prior-head observations do not establish current-head CI status or signed research verification. The latest source update needs its own checks.
