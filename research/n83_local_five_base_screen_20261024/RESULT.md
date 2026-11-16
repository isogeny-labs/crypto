# N83 five-base local screen: replay passes, timing order remains noisy

On one new public N83 point, the sealed five finalist arms each completed two
fresh cold IC processes in forward/reverse order. Both same-point rho controls
also completed. Independent audit passed every raw file hash, exact candidate
and base identity, full-rank relation count, group-scalar replay, and returned
answer. All five constructed bases matched fresh downloads from the requested
S3 factor-base prefix byte for byte. `selected_best_total_runtime` remains
null: this is one local macOS point, several IC pairs vary by more than 10%,
and the two rho controls used distinct fixed seeds and vary substantially.

| Base | Usable points | A process (s) | B process (s) | Median (s) | Pair spread |
| --- | ---: | ---: | ---: | ---: | ---: |
| K992/root11 | 164,672 | 72.478 | 59.822 | 66.150 | 19.132% |
| K1024/root11 | 169,984 | 66.877 | 58.458 | 62.668 | 13.435% |
| K3300/wide3 | 547,800 | 416.578 | 350.521 | 383.550 | 17.223% |
| K2304/root11 | 382,464 | 116.943 | 122.542 | 119.742 | 4.676% |
| K1107/root11 | 183,762 | 67.647 | 63.902 | 65.774 | 5.693% |

Rho-before took 2.015 seconds and rho-after 0.989 seconds; their median was
1.502 seconds and spread 68.274%. Because those controls use different seeds,
that spread cannot be attributed solely to host drift. K1024 has the lowest
descriptive IC median, but its 3.107-second gap to K1107 is smaller than the
observed within-arm variability. The much larger K2304/K3300 medians show a
local cost penalty; they do not establish a population-wide or all-grid result.
In K3300, setup alone took 390.706 and 328.972 seconds in the two children.

The fixture was generated before timing from the domain in `generate.py`, and
its target SHA-256 is
`b1a9b1bc398b1c73867b8b0c19ef8a7438225917989e08b1c15aef2c7640c0bf`.
The point is distinct from the frozen 16-point panel and the earlier local
root-layout point. `audit.py` recomputed its scalar multiplication, checked
all ten IC results and both rho results, and verified every recorded raw file.
Changing one byte in the K1024 B target result caused the audit to reject the
copy at its file-hash gate. The exact audit is in `audit.json`.

The five S3 objects named in the frozen custody receipts were downloaded
again. Their gzip SHA-256 values were
`115391d004768be1108d02176bceef2c9bad982decd74697cfa77a4f2055f3cd`
(K992), `fd7a79674ed74e12f70de75f702fe5371f692347d5042541a29f80683bdccce2`
(K1024), `a04f4559cc796ae147517bcc7f7483e4deaac189f62cdc939bc7a8bc6d6f668e`
(K3300), `d5711731d01b4061c4feaa7c5ec6654b073b40856355ec78fc5fb284b41f266e`
(K2304), and `ccdbf619478990570bfba982c7d1bdfe361ed4530c32407ece4a7ad1be5c42a5`
(K1107). Each decompressed SHA-256 equaled both the prior custody receipt and
the new process dump, so no new factor-base object was necessary.

The raw 225 MiB directory is at
`/Volumes/SSD990/crypto/n83-five-base-evidence-20261024/raw`. Its local archive
is `/Volumes/SSD990/crypto/n83-five-base-evidence-20261024/raw.tar.gz` with
SHA-256 `5a798946ae0e28c42a7fb2d3b2a56b0bbf2bb1510be880d8ef67ae145d68d79a`.
The previous automatic approval review rejected exporting a broader raw N83
diagnostic archive to S3 under the factor-base-only authorization, so this
archive remains local pending specific approval. This committed digest-bound
summary does not replace the raw archive for independent replay elsewhere.

Validation before publication: `cargo test --release --lib --locked` passed
4,162 tests with 117 ignored; the relevant IC and rho examples passed 21 and
4 tests. Four frozen-finalist Python checks, the 52-test boundary-autolab
suite, fixture regeneration, independent raw audit, and one-byte tamper
rejection passed.
