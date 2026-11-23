# P-192 marked orbit representative evidence

This focused publication contains rounds 44–46 and the exact files named by their dependency manifests. Earlier study rounds are not republished. Round 4 and round 43 JSON files are historical dependency records, not independently rerun certificates in this PR.

Start with [round46 report](round46-p192/report.md) and its [PDF](round46-p192/report.pdf). Round 44 gives the labelled cached walk, collision controls, storage accounting and global carrier-dimension argument. Round 45 develops marked two-coordinate matrices, including preserved first-attempt accounting and its amendment. Round 46 constructs a three-coordinate representative with explicit point action and independent replay.

The best checked round46 schedule is 65 group doublings plus 45 additions, including input tables and deriving the fifth orbit point. Setup of the stored triple, negations and selection are separate. This is an exact schedule result, not a runtime measurement. Full three-coordinate walk accounting and cycle controls remain open. No class-number or seed-selection claim is established by this package.

## Replay

Python 3.9+ is sufficient for the independent verifiers. The round46 producer additionally needs python-flint (original environment 0.6.0). PDF rendering uses typst and pymupdf. Perform replay in a disposable copy because verifier scripts write verification JSON and logs; sealed source files should remain immutable.

For each of rounds 44, 45 and 46, run `sha256sum -c manifest.sha256` and `sha256sum -c dependencies.sha256` from its directory. Then run `python3 verify.py`. For rounds 45 and 46 also run `python3 verify_points.py`. Each verifier must report all_checks_pass. The complete publication replay receipt is in `publication-validation.json`.

Some search/build scripts execute at import time: run them as programs only. Round18 cached_orbit.py and round38 lattice.py are safe imported dependencies. Producer re-execution is not needed to check the saved certificates.

D and A denote group operations, not field multiplications/additions. Saved cost comparisons have their own stated models and exclusions. Public known-answer fixtures test arithmetic and labels; they do not measure collision distributions.

The PCC record for this publication pins the local replay and certificate bytes. Its scientific standing is unverified pending a trusted signed verdict. [pcc:unverified]

Hash-pinned records remain loose because the manifests and replay tools address their exact paths. Historical round reports retain their original bytes and terminology.

The affected Python checks passed. The broader Rust library check in the migrated legacy checkout timed out after 30 seconds during compilation; no Rust test pass is claimed. See publication-environment.json.
