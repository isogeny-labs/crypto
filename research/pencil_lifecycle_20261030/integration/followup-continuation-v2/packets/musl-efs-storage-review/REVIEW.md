# Storage amendment source review

The v3 wrapper differs from reviewed v2 in exactly two substitutions: the source parent is `/home/ec2-user`, and the target is `/home/ec2-user/pencil-musl-target-b985-20261031`. Both manifest entries match their bytes and SHA256; Python AST parsing passes. No additional blocking discrepancy was found in this narrow amendment.

The receipt fix remains inside the protected try block. Commit, detached-source and cleanliness checks, compiler bindings, Cargo flags, build cap, failure handling and post-build ELF/map checks are unchanged. This review does not admit a build or qualify an image.

Before admission, bind effective Cargo configuration at the source, `/home/ec2-user`, `/home`, `/`, and CARGO_HOME; the parent change can alter configuration discovery. Retain the prior external-input/path dependency and source-cleanliness obligations. Prepare a direct, non-symlink fresh target with exclusive ownership. The wrapper's lock and local process probe cover cooperating/local owners; they do not establish absence of remote or noncooperating writers. Existing coordination plus the uniquely reserved target is the intended ownership boundary. The wrapper itself does not validate target symlinks.

Optional cache copying is unnecessary for correctness and must finish before admission, after reconciling the old owner. Preserve the old cache and do not manipulate compiler fingerprints. Source relocation may cause ordinary rebuilds. Fresh space checks must cover local receipts, logs, link map and the final binary copy, despite target/source placement on EFS.

A current read-only `df -h` reports 8.8 GiB available on the local root, rather than the proposal's historical 3.3 GiB. `/home/ec2-user` is mounted from `127.0.0.1:/agent-migration/home`. These observations neither establish build storage sufficiency nor change the selected storage plan. No checkout, cache mutation, build, cloud or scientific execution occurred in this review.
