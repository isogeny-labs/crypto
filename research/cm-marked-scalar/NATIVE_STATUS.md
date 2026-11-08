# Native execution status

The C++/OpenSSL evaluator passes 50 curve/input/window cases on P-384 and P-521. Both full polynomial map identities, the closed dual point actions, primitive seventh roots and scalar correction congruences pass. Direct signed-window multiplication, the relation route and both OpenSSL configurations agree. Altered scalar labels are rejected on nonzero controls.

This is `demonstrated_on_cases` software qualification plus the stated polynomial checks. No native timing, end-to-end gain, fixed-base gain or amortized gain has been measured. The executable has only a correctness mode, so an accidental shared-host speed measurement cannot be presented as admitted evidence.

The first native build succeeded with warnings; the second is warning-free and explicitly rejects empty JSON tokens. Sources and binary hashes are retained before optimization. The report and receipts are in `qualification/native-v2/`. Build with `make -C research/cm-marked-scalar -j1`; replay with `/tmp/cm-marked-scalar-build/native --qualify research/cm-marked-scalar/fixtures`.

Next: implement a matched native timing and isolation path, freeze source/inputs/protocol before measurements, and improve the complete relation route against the fastest reproduced public-scalar reference. Existing root Cargo tests were not run for this standalone C++ change; the touched native path was built and replayed as recorded.
