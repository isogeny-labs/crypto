# Frozen full D18 direct-subgroup orbit archive

This candidate uses **all 32,643** full-length signed-Frobenius orbit classes
from the independently archived dimension-18 direct-subgroup census of the
primary Koblitz `a=0` GF(2^83) curve
`icv1-f2m83-tm6151469093347-debefd74`. The earlier matched U/D comparison
selected only the first 16,627 classes. This new candidate tests a different
base size and therefore must not be called a matched U/D comparison. The
source list is the exact prior `orbit-classes.csv` from branch
`codex/n83-d18-subgroup-orbit-census-20261018` at
`124ab188cc39d52ee99e979da95b501a3ca34f88`, copied into this directory.
Its complete-file SHA-256 is
`7bd1a338d0ac0b0faedc4f5e2fb0e735cc1047dcc3666ca87704c2d4b23379b4`;
the BLAKE3 of its data rows is
`6d230ffc20624b8e7ae466210175a6f0ac6dbe45906ac723a1242177586048f0`.
The frozen census summary SHA-256 is
`bedf8f59acf5fbed1bf2d2afdf38e992e08fb4f695d6645e5f3c83735b40ba4f`.
The exporter rejects a changed source list, count, curve, summary or source
order before constructing any point rows.

The [native exporter](../../../examples/n83_d18_full_orbit_export.rs)
uses the packed field backend to lift each recorded source x directly into
the prime-order subgroup. It emits all 83 Frobenius phases and both signs
with exact `±lambda^phase mod r` labels, totaling **5,418,738 distinct point
rows** and 32,643 orbit columns. It rejects infinity, short or duplicate
orbits, changed x lifts and wrong subgroup membership. The archive is
content-addressed by compressed BLAKE3. A separate generic multi-limb
replay checks every representative's curve, subgroup and Frobenius identity,
every point and label, orbit closure, global point uniqueness and the whole
point-set hash. Preserve a failed export or replay and classify any process
cap as `UNKNOWN_budget`. Export and replay each have a 600-second wall cap.

Only after full replay passes, write the exact object once with
`If-None-Match: *`, SHA-256 checksum and AES256 to
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0/objects/`.
Then fetch it afresh; require matching compressed SHA-256, BLAKE3 and byte
length plus the expected S3 metadata. Publish exact provider responses,
manifest, replay, host/cap receipts and independent rejection controls. A
collision or existing key is acceptable only after byte-for-byte identity
has been verified; no object may be overwritten.

For the prime subgroup order `2417851639230796216685689`, the unordered
four-summand uniform-target counting mean is
`C(5,418,738 + 3, 4) / r ≈ 14.85772113`; the previous 2,760,082-point
base's mean is about `1.00011051`. These are support counts under a uniform
target model, not observed relation probabilities, search rates or runtime
measurements. The larger base also doubles the column count and increases
import, domain and linear-algebra costs. It can only be ranked after
identical fresh natural-query targets, independent relation and rank replay,
full cold one-target cost including failed searches and S3 transfer, an
unseen-target descent, and matched same-target rho. The selected minimum
remains `null` until those measurements exist.
