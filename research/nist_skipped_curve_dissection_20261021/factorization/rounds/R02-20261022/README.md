# R02: eight complete factorizations, one composite left

R02 completely factors two more degree-2 extension-twist orders: skipped
P-224 block 5736 and published P-224. **Eight of nine distinct integers
and thirteen of fourteen original DiSSECT timeout fields are resolved.**
The P-256 block-876 order retains a 444-bit composite. These are the
quadratic-twist orders over the degree-2 extension, `(p-1)^2+t^2` with
`t=p+1-n`, rather than the base-field twist orders.

| Target | R01 residual (bits) | R02 residual (bits) | Largest certified prime (bits) |
|---|---:|---:|---:|
| `skipped_224_5736-twist2` | 327 | 0 | 196 |
| `published_224-twist2` | 364 | 0 | 209 |
| `skipped_256_876-twist2` | 444 | 444 | 33 |

Zero means the whole target has a complete prime factorization. Bit lengths
come from native PARI verification of these fixed inputs; no sampling
interval or runtime comparison applies. [Exact partitions](results.json),
[field replacements](resolved-fields.json), [summary](summary.json),
[full run report](reports/run-20261022T0724Z/REPORT.md) and
[inspected PDF](reports/run-20261022T0724Z/report.pdf) give the evidence.

![Recorded residual bit lengths](reports/run-20261022T0724Z/residual-bits.svg)

## New factors

ECM at B1=11000000 found the remaining split for skipped P-224 block 5736
on curve 29. The new factors are

```
2563469704968499423924395332605645482413
74533784647009995798737425147716556977324660145660465361081
```

Together with the previously known `8581`, `8548615513285513` and
`51858865004599249`, all with exponent one, these completely factor its
448-bit target.

Native NFS, resumed from its retained relations after an initial exit 139,
split the published P-224 residual into

```
37309302922695684404996011132935712068656641601
718984363233019940901094618609503248485899884702029925470651453
```

Together with `27095768760825094922392397`, all with exponent one, these
completely factor the other 448-bit target. Both new splits have retained
PARI prime certificates and independently replayed exact products.

## Replay without the large raw archive

The verifier reconstructs all nine original inputs from the hexadecimal
curve fixture, compares them with the frozen target list, checks every
prime certificate, and checks the exact products and reported metadata.
The individually named prime certificates stay loose under the AGENTS.md
section 13a verifier-path exception.

```bash
round=research/nist_skipped_curve_dissection_20261021/factorization/rounds/R02-20261022
GP_BIN=gp bash "$round/replay_results.sh" "$round/certificates" /tmp/nist-r02-replay
```

Choose a fresh output directory. PARI/GP 2.17.2 was used. The
[product proof map](proof-map.md) links nine `identity.certificate/v1`
records to [Lean 4.35.0 proofs](ProductIdentities.lean). The repository proof
gate checked nine theorems, zero failing, zero trusted native proofs; all
axiom lists are empty. **Lean proves the products, not primality.**

```bash
bash tools/lean/proof-gate.sh "$round/ProductIdentities.lean"
```

The native identity issuer is the study-root `identity_records.rs`, compiled
with `NIST_FACTOR_STUDY` set to this round's repository-relative path. Its
default remains the unchanged R01 proof path. Issue and check are separate
invocations; see `runs/identity-{issue,check}.log` after archive extraction.

## Raw evidence and portable checkpoint

The complete deterministic `runs.tar.xz` is larger than GitHub's file limit.
[Archive manifest](archive-manifest.json) and its SHA-256 identify the local
archive. Publication to the required `s3://crypto-autoresearcher/store`
remains blocked because this runtime has no configured AWS identity.
No remote object or `store:` index entry is claimed. All mathematical
results and their replay certificates are available in this PR without
that archive. Once the byte-identical archive is fetched into this directory:

```bash
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
GP_BIN=gp bash replay_results.sh runs /tmp/nist-r02-raw-replay
```

To reproduce the archive, use the canonical command in AGENTS.md section
13a, with mtime `2026-10-22 00:00:00Z`. R01 is not repacked or rewritten.

The small, checksum-verified
[P-256 checkpoint excerpt](checkpoint-skipped_256_876-twist2.tar.xz) is a
derived job input, separate from the complete raw archive. It contains
per-thread polynomial candidates, not a native-recognized master file,
selected polynomial or sieve relations. **R03 discovered that this original
excerpt alone restarts the polynomial search.** Use the
[corrected R03 checkpoint](../R03-20261022/README.md) for continuation;
[resume_job.sh](resume_job.sh) now refuses the original per-thread-only
state before starting compute. The corrected checkpoint appends a bounded attempt;
[verify_nfs_artifact.sh](verify_nfs_artifact.sh) independently checks the
fixed original target. A verified partial result is not completion.
[R03](../R03-20261022/README.md) records the discovery and corrected restore;
its runs must not be attributed to R02.

## Scope and remaining gaps

P−1 and P+1 yielded no split within their fixed allocations. The two PARI
search phases failed to serialize proposals after an alarm; they are
inconclusive, not valid negative searches. Their corrected matrix/alarm
controls passed. One NFS control initially selected SIQS; a later forced
GNFS control and its checkpoint resume both passed. The production P-224
exit-139 cause is unknown; the retained state subsequently yielded certified
factors. All errors and failed attempts remain in the raw archive.

The remaining [factorization obstruction](../../../../../problems/nist-extension-twist-composite-cofactors.md)
does not imply a lower bound on its unknown prime factors. The forty
generator-dependent null cases, upstream torsion/trace defects and the
historical skip reason remain unresolved by this supplement. No ECDLP
speed, security, or historical-selection conclusion is inferred.
