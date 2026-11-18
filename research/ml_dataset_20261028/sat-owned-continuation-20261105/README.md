# Original-owned SAT continuation validation

The isolated release build passed. The corrected runtime inventory selected 317 SAT tests: **313 passed, zero failed, four ignored**. All 18 required controls passed. They cover native accounting/refusal, fresh model verification, intermediate-prefix rejection, and retention of the genuine both-twist original owner at the closed native entry gate. Source and binary hashes remained unchanged.

The earlier runner compiled successfully but failed its module-name inventory assertion before invoking native tests. That diagnostic receipt is preserved unchanged in `compile-and-selector.receipt.json`; `native.receipt.json` records the subsequent runtime check against the same binary. This was an inventory correction, not a repeated failed native test.

The compiled binary SHA-256 is `d791f824b6797ee391894ece3753d74c668305d7451d4c8e8f7061fb1403aa7d`. Source is the isolated native admission commit `80fc3b565c4f` plus the reviewed seven-file owner continuation and checked-model ingress/prefix correction. The implementation remains local pending the full release-library gate, which started on the unchanged source with a 3,600-second limit, one test thread, and zero retries.

The check demonstrates owner retention and model-checking components. It does not demonstrate a completed genuine both-twist nonroot SAT solve. Search entry gates remain closed. Native positive qualification, complete terminal custody, central replay integration, and CLI activation remain required.

The custody receipt records a conditional S3 upload followed by a fresh ETag-bound download, archive-byte comparison and verification of every contained hash. To replay custody, download its URI, verify its byte count and SHA-256, extract into a fresh directory and run `sha256sum -c SHA256SUMS`. The archive includes the exact tested binary, all 1,435 recorded source inputs, original compile/inventory diagnostic, successful runtime log, proposals and reviews.
