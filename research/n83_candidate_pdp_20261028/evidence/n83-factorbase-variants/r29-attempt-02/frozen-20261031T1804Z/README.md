# r29 attempt-02 protocol v11

Protocol v10 is retained as a superseded prelaunch record. Its prelaunch guard caught a mismatch between the protocol's runner SHA-256 (`ea3aa8...`) and the actual/indexed runner (`a880a3...`) before any output directory or attempt-02 data existed. v11 corrects that source identity and records an explicit `VERIFY_ONLY=1` check.

The mathematical scope and run bounds are unchanged: coordinate bit 29, all 2^27 selectors, 3 families x 128 shards, same base for m=2..9, the same binary/source/runner, 6 threads, 8 GiB virtual address-space cap, 5,400-second inner and 5,700-second outer timeout, one attempt. Output remains at the unique r29-r2 S3 prefix.

Protocol SHA-256: `6286acae353c7e56870584e7bb14220f8c6870d0aeb67c8b61610dd93564b259`. Source-index SHA-256: `77b607cf91ac606e6bf06f7ad4b2948ec2815d55a95aa0b66d54c6b250b05168`. Launch-command SHA-256: `0da0f168e213c3dd83f08230b50702f61718214a624ccbf4a4c208f89eae9606`.
