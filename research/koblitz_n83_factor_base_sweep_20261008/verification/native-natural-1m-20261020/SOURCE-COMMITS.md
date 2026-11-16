# Exact source history for frozen native N83 1M records

Git CLI write credentials are unavailable; the connected GitHub app publishes a
fast-forward snapshot on the existing PR branch with an exactly matching local tree.
[SOURCE-COMMITS.bundle](SOURCE-COMMITS.bundle),526148bytes, retains every local
experimental source revision and the integrated validation source commit
`7abf1175b7fd9f2bfac86ff1657c19b430e964e9`. The final timed/assessed controller
freeze is `66ab009b1d91792c46d078e15adfeda45eacf29f`; earlier f26/8b6/c601/f315
revisions remain recoverable with their original exact IDs. The worker is not rebuilt.

[Verification](SOURCE-COMMITS.verify.txt) and [SHA256](SOURCE-COMMITS.sha256).
Prerequisite commits, both retained on the upstream PR history:
`13f97c73059fd04421af0c036bf63863e66f11df` and
`7dbc3b790860fb04a5132fb42e5f85986abe7b6e`. The latter merged current main
while these experiments ran. Local integration preserves it and both benchmark
index insertions; no upstream work is replaced or force-pushed.

After fetching those prerequisites:

```sh
git bundle verify SOURCE-COMMITS.bundle
git fetch SOURCE-COMMITS.bundle HEAD:refs/heads/n83-1m-original-source
git show 66ab009b1d91792c46d078e15adfeda45eacf29f:tools/n83-sweep-runner/src/systemd_scope.rs
```

Later commits add only publication/verification/source-history evidence;
post-merge validation names the exact tested source above. Published snapshot
commit IDs differ, with source history retained explicitly rather than lost.
