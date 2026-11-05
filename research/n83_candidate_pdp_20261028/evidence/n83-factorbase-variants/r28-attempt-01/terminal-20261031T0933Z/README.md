# r28 factor-base completion

Job `n83-fb-variants-r28-20261031-r1` (UID `2a68d878-4723-4679-b258-5646794540bf`) succeeded. The producer exit code is 0. It enumerated all 2^27 selector x-coordinates for the fixed `spike28` coordinate map on each of the Koblitz, general-binary, and prime curves; each completed base is reusable for m=2..9.

The native manifests and S3 listing join verify all 384/384 shard objects, with matching content-addressed SHA keys and byte sizes, and record every candidate x as checked for omitted lifts. The final S3 results listing contains 793 objects: 786 immutable snapshot objects, including the compressed shards, and seven run/index/source metadata objects. The terminal index object is named in `summary.json`. Captured status, logs, and all eight exact results-listing pages are beside this README.

The compressed shard data remain in the immutable prefix recorded in `summary.json`. The managed workspace currently cannot assume the AWS role needed to fetch the terminal S3 file-index bytes locally, so this package preserves the exact read-only results pages and the producer's already verified per-shard SHA/size join; it does not claim a local byte-for-byte restore of the full 4.5 GB shard set.

This is factor-base enumeration/custody evidence only. It contains no PDP systems or degree-of-regularity labels.
