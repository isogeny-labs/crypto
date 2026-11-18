# Executed: a twist-derived degree17 source endomorphism

Native and independent PARI verify the concrete source endomorphism
α=(π+[1])/[7], both generic norm compositions with1−α equal[17], and
[7]α=π+[1]. This supplies a direct twist-derived connection to generator
arithmetic on the source. It uses standard isogeny/endomorphism theory;
unknown-log recovery, cheaper IC, novelty and NIST benefit remain unestablished.

[Report/figure](../reports/endomorphism-20261027/RESULTS.md) ·
[PDF](../reports/endomorphism-20261027/RESULTS.pdf) · [admission](ADMISSION.json) ·
[pre-data protocol](PROTOCOL.md) · [structured freeze](protocol.json) ·
[source/binary/core pins](FROZEN.json).

Source E:y²=x³+x+15/F827, order823,G=(2,5),public Q=37G. The d2 twist has
order833 and maximal-order H=(2,386),order119. Fixed[7]H=(345,743),order17,
embeds as an anti-Frobenius kernel. Vélu gives y²=x³+31x+77; the fixed
(x,y)↦(49x,343y) isomorphism returns to E. On all823 source points,
α(P)=[353]P and[415]α(P)=P; all822 wrong-scalar354 controls are rejected.
Images of G,Q are(189,620),(451,814). All833 twist points have kernel17/image49
killed by7. The scalar353 has order274 mod823, verified independently.
These are exact finite/generic checks, not a cost measurement or cheap orbit fold.

Source c12290db2fd94b4a929b2e0586ff8973dd8079a1, protocol
1bfb4a24bf342ec370dfaa59b861449fd7b59c422effd85def05139f47df5523,
committed/checkpointed before main data. One native/one GP execution, each
300s/4GiB/CPU0/one thread,L0 managed Xeon Platinum8573C/KVM. Neither stage
fails or caps; strict GP admission accepts only the declared stack warning.
The [unified export](experiment.export.json) binds to the full frozen
protocol/schema/files with zero findings. The EKS access/image fallback is
documented in prior waves; no paid resource or IAM change.

The native program copies the byte-preserved v8 arithmetic/map prefix; see
[snapshot receipt](CORE_SNAPSHOT.json). It independently constructs generic
[7]/[17], the conjugate id−α and Frobenius(x^827,y*f(x)^413). GP reconstructs
the maps with ellisogeny and separate rational-function group arithmetic,
then uses the rational twist map(X,Y)↦(2R(X/2),Y*S(X/2)). No finite scalar
action is substituted for the generic identities.

Three conditional Lean declarations are in ../lean-v6; fresh frozen replay
and native axiom audit pass with propext/Quot.sound only. They prove the norm
polynomial implication, its Frobenius polynomial consequence and scalar inverse
on823-torsion. Concrete field/curve/morphism/endomorphism-ring bridges remain
unformalized. No class-polynomial or full volcano census is claimed.

[Exact EC1 model UIDs](EC1_MODELS.json), [ordered route](ORDERED_ROUTE.json)
and canonical intermediate-model record retain direction, generator and raw
map hashes. The source and returned curve share the geometric model; the
returned generator is α(G), so its generator-bound EC1 UID is distinct. The
same DLP scalar is preserved when both G and Q are transported; using the
original G instead gives scalar353*k, inverted by415 modulo823.
No global ISO1 assignment is inferred. The complete native
canonical checker verifies75 model records/34 extension models, including
restored historical metadata and the new intermediate. Existing curves'
large integer metadata is preserved byte-for-byte by the append-only writer.

## Hydration and replay

From repository root, use the maintained artifact tool:

```sh
python3 tools/artifacts/artifacts.py pull research/twist_routes_1_3_20261021/execution-v10
cd research/twist_routes_1_3_20261021/execution-v10
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

Raw archive:3,988 bytes/8 files, SHA256
725ce0e6d3b12a0437fb3e2cecc2b2c63880913bdaf337d4c560aa90fd15a819,
stored in the user-confirmed `s3://setec-astronomy-artifacts/store`. Member
inventory and deterministic packing/repacking recipe are beside it. The fresh
remote [member/hash/repack check](RAW_VERIFICATION.json) covers every file.

Build `twist-endo-symmetry` from its pinned source with Rust1.90/Cargo.lock and
the standalone runner manifest. Call `twist-endo-symmetry FRESH_OUT`, compare
RESULTS.json/verify.gp to extracted bytes, then native twist-pari-transport
SCRIPT FRESH_GP_RECEIPTS under the pinned GP layout. Call
`twist-endo-symmetry admit RAW GP FRESH_ADMISSION`; it verifies actual exit,
script/output hashes, exact marker and stderr. Existing output paths fail closed.
Lean replay is `bash research/twist_routes_1_3_20261021/lean-v6/replay.sh FRESH_ABSOLUTE_RECEIPTS`.

Separate artifact/receipt publication initially ran before new proof files
were indexed and failed; the staged retry succeeded. PDF math-delimiter/font
layout and earlier render errors remain logged, then corrected without changing
scientific bytes. Neither is a scientific result or hidden failed experiment.
No credential value is saved. Original full campaigns and all cost/formal/
independent-host gaps stay open; new future cards retain their unrun status.
