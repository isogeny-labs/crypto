# Round22: exact norm-zero recovery and proper source geometry

The exact recovery calculation succeeds with **d=1** on the actual norm-zero
factors. The full8-by32 integer system has eight unit Smith invariants and a
24-column integral kernel basis. A concise recovery uses only M1 and M3=7I;
all22 both-sign order11 fixtures pass. See [RECOVERY.md](RECOVERY.md).

The original SS5 incidence fiber is also certified:four degree20 closed
points over F2,all registered units invertible,with native ideal-lift and
Frobenius certificates. See [CM.md](CM.md).

The [report](REPORT.md),[PDF](report.pdf),[visual](round22.svg),
[proper-geometry notes](CM.md),[coupled-divisor argument](SATURATION.md),
[proof map](PROOF_MAP.md),[validation](VALIDATION.md),[sources](SOURCES.md)
and [ledger](ledger.json) retain successes,failed attempts and exact limits.
Scientific input69f6740;publication parent3c87193;Conductor T-409.

Proper level77 normalization,lifted divisors,complete prescribed RR,the
order11 modular-factor map,explicit node187 polynomial membership,full
quotient saturation and arbitrary-divisor reverse evaluation remain open.
The coupled node187 exclusion has explicit written proper/pole hypotheses.
No IC efficiency assessment or new relation rank is supplied.

Hydrate and verify the immutable raw evidence,then replay the native results:

```bash
python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/22
cd research/koblitz_modular_ic_20261018/rounds/22
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
./REPLAY.sh
```

The archive uses sorted GNU tar,mtime2026-10-26,uid/gid0 and the prescribed
normalized mode,then xz-6 with one thread. Repacking the extracted tree with
those settings reproduces the bytes. CAS failures are retained;REPLAY.sh
repeats exact native checks and does not silently rerun normalization jobs.
The complete Lean project is checked by tools/lean/proof-gate.sh. Artifact
objects live under the user-specified setec-astronomy-artifacts/store prefix;
the directory manifest pins their hashes and sizes.

With the pinned native Singular4.4.1 backend installed,the exceptional-fiber
certificate can be replayed separately:

```bash
SINGULAR=/path/to/Singular ./REPLAY_SS5.sh
```

This does not repeat any incomplete normalization or place computation.
