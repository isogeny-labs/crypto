# Archived Lean source custody

These thirteen files are historical/repeated source copies in the report evidence tree. They are preserved byte-for-byte locally and in the delivered review archive. They are excluded from Git source discovery by the narrow report-root `/evidence/**/*.lean` rule, so the repository core-only proof gate does not treat failed or repeated review inputs as canonical theorem modules. Other evidence remains unaffected. The canonical four-source Lake project is delivered separately.

Archive: `same-support-review-v1.tar.gz`.

SHA-256: `8a3bc72d274f5c890b4480619f2df78fd22204c4307db5c5f150bad86adef074`.

Each local source was independently hashed and compared to the exact archive member extracted to stdout. All thirteen comparisons passed. Original file bytes, evidence manifests and report hashes were not changed. The exact mapping is also retained as `source-custody.tsv`.

| Local report path | SHA-256 | Archive member |
|---|---|---|
| `evidence/ProjectiveLawBoundary.lean` | `0cbdbb636e11f34ad94493353f2fc748a353c89f248f868accbdaaf02fc88e2b` | `same-support-review/report-v2/evidence/ProjectiveLawBoundary.lean` |
| `evidence/RuntimeSemantics.lean` | `9e015c3c79f465b2b16f6e0dae5c744f9de3400f51646f746c61c54c4722c318` | `same-support-review/report-v2/evidence/RuntimeSemantics.lean` |
| `evidence/SameSupport.lean` | `c0cd495c938a7370d0e8b3b199fe41b67301ae0377d462237bf533435e26977a` | `same-support-review/report-v2/evidence/SameSupport.lean` |
| `evidence/candidate-attempt3-setup-failed/SameSupport.lean` | `c0cd495c938a7370d0e8b3b199fe41b67301ae0377d462237bf533435e26977a` | `same-support-review/report-v2/evidence/candidate-attempt3-setup-failed/SameSupport.lean` |
| `evidence/candidate-attempt3b/Readback.lean` | `5ccd69490b5daea9461ab6cfe2edc53694af10939e3d3d207c9e6b412b9ed5e8` | `same-support-review/report-v2/evidence/candidate-attempt3b/Readback.lean` |
| `evidence/candidate-attempt3b/SameSupport.lean` | `ecb3a5f91f7217d1ab107466c470a491559f7cfe0276ca6d72661fffef66170b` | `same-support-review/report-v2/evidence/candidate-attempt3b/SameSupport.lean` |
| `evidence/candidate-attempt4/Readback.lean` | `2cc9d86bce34f3bbb9433c18166ec69ab8e60dc52d58376773d5b429b4cb1952` | `same-support-review/report-v2/evidence/candidate-attempt4/Readback.lean` |
| `evidence/candidate-attempt4/SameSupportClassifier.lean` | `7f21fd6153d972309fe5f0d4b7a5b00ff188eb4e6510cbfbd8c33f7a12efa41e` | `same-support-review/report-v2/evidence/candidate-attempt4/SameSupportClassifier.lean` |
| `evidence/independent-replay/ProjectiveLawBoundary.lean` | `0cbdbb636e11f34ad94493353f2fc748a353c89f248f868accbdaaf02fc88e2b` | `same-support-review/report-v2/evidence/independent-replay/ProjectiveLawBoundary.lean` |
| `evidence/independent-replay/Readback.lean` | `1c66e4c0b16773ba2236b3e2c4a28c9a0afbdc05003dafab54fa5ff4d9721b51` | `same-support-review/report-v2/evidence/independent-replay/Readback.lean` |
| `evidence/independent-replay/RuntimeSemantics.lean` | `9e015c3c79f465b2b16f6e0dae5c744f9de3400f51646f746c61c54c4722c318` | `same-support-review/report-v2/evidence/independent-replay/RuntimeSemantics.lean` |
| `evidence/independent-replay/SameSupport.lean` | `ecb3a5f91f7217d1ab107466c470a491559f7cfe0276ca6d72661fffef66170b` | `same-support-review/report-v2/evidence/independent-replay/SameSupport.lean` |
| `evidence/independent-replay/SameSupportClassifier.lean` | `7f21fd6153d972309fe5f0d4b7a5b00ff188eb4e6510cbfbd8c33f7a12efa41e` | `same-support-review/report-v2/evidence/independent-replay/SameSupportClassifier.lean` |

To materialize a single archived copy after hydrating the delivered archive using its artifact manifest, verify the archive hash first, then extract the selected member to a new scratch directory. For example:

```bash
printf '%s  %s\n' \
  8a3bc72d274f5c890b4480619f2df78fd22204c4307db5c5f150bad86adef074 \
  /path/to/same-support-review-v1.tar.gz | sha256sum -c -
mkdir -p /path/to/new-review-scratch
tar -xzf /path/to/same-support-review-v1.tar.gz \
  -C /path/to/new-review-scratch \
  same-support-review/report-v2/evidence/candidate-attempt3b/SameSupport.lean
```

Hash the extracted file and compare it with the row above before use. Do not change the repository proof gate or infer that all archived attempts compiled; failed-attempt sources remain deliberately included in this archive.
