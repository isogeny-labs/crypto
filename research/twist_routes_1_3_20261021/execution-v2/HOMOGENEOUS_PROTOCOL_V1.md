# E25 correct-field homogeneous pilot, version 1

Freeze date: 2026-10-23. Commit source, exact external adapter, this protocol
and supporting checks before execution. Retain every engine/checker failure,
timeout, incomplete basis, unsupported order, exceptional affine branch and
component at infinity. This is a correctness/applicability pilot, not a
complete solver-performance comparison or completion of E25.

Use the frozen execution-v1 coordinate-box h=8 bases, their first eight
canonical points, the original public solver target seed 0, and p=59,211,251.
Cross arities 2,3 with product, chain, bits and symmetric encodings and
grevlex/lex: 48 requested cells. Preserve the original encoding's affine
task, its diagonal/identity exceptions and the native exact assignment
census. Homogenize every generator with one last variable hh; setting hh=1
must recover the original polynomial, and every actual affine assignment
must satisfy the transformed system. Do not confuse generator homogenization
with the saturated homogeneous ideal. Check hh=0 and saturation separately.

The msolve 0.7.5 source audit corrects an earlier interpretation of its
diagnostic. The signature initializer prints a placeholder characteristic
1073741827, but resets the actual characteristic to the input field before
the signature core. That diagnostic alone does not show a wrong-field run.
Stock 0.7.5's export_results_from_gba exits for signature mode, including
homogeneous input. Use the pinned local output adapter below to serialize
the already computed signature basis before that unsupported export stub.
No field assignment, selection, reduction, termination or GB algorithm is
modified. Treat this as a distinct local fork, not stock msolve success.

Source archive: Debian msolve_0.7.5.orig.tar.gz, SHA256
e88368abfd1e1918329ff9444164ca0e304835794fec68d192a63c845ae63128,
https://deb.debian.org/debian/pool/main/m/msolve/msolve_0.7.5.orig.tar.gz.
The GPL-2.0-or-later source retains its authors and license. The exact local
change is solver-adapter/msolve-signature-output.patch; build with the
recorded GCC command, linked library versions, source/patch/binary hashes.
Keep build output. Version the binary 0.7.5+twist-signature-output.

On each grevlex cell run five arms: stock Debian q0 on the affine system;
local-adapter q0 on that identical affine system; local q0 on the generator
homogenization; local q1 on that identical homogenization; and stock q1 on
the homogeneous input as an export-path control. Use one thread, 4-GiB
address space and 30 seconds per engine. Retain the actual input field and
signature field-correction note. Both local q0 arms are controls on the
adapter/build, not a new algorithm. Repeated cold input/rendering work and
all checker/extraction work are outside any unmeasured performance claim.

The pinned signature path provides grevlex only. Preserve all 24 lex cells
as unsupported signature orders, with an independent Singular lex affine
reference and exact assignment/dimension check. Do not relabel a grevlex
basis as a lex computation or silently add unpriced FGLM.

Parse complete output without truncation. In Singular independently
reconstruct each claimed homogeneous basis and each dehomogenized affine
basis in the exact field/order. Check every required S-pair reduces to zero,
every input generator reduces by the candidate, and every candidate lies
in an independently computed reference ideal. Ideal equality alone is not
a complete-basis certificate. Replay every native affine assignment, and
check the affine reference dimension equals their exact count. Missing
output, parser errors, timeouts or a failed S-pair leaves the basis incomplete.
Before the ordinary grid, run both local q0/q1 modes on the homogeneous
linear positive control x−hh,y−2hh over F59. Its affine chart has exactly
one solution (1,2). Independently reject an intentionally truncated basis,
a wrong-characteristic negative polynomial set, and mismatched characteristic
metadata. These are separate control rows, not extra ordinary-grid cells.
Run homogeneous, affine and infinity/saturation checker stages under separate
30-second/one-thread/4-GiB caps; retain every stage's inputs and raw output.
Use at most three independent cells concurrently. The 3,600-second global
batch cap is an additional fail-safe with per-cell checkpoints.

Magma/FGb/OpenF4 are not installed in this runtime. Stock msolve q0 and
Singular are applicable available references; no claim of beating the
fastest external solver is admitted. Complete operation pricing, all
original solver-grid cells and formal encoding bridges remain open.
