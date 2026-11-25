# Replay and scope

Run from this directory, using the official SHA-pinned PARI2.19.1 build
documented in round03. GP commands are native arithmetic checks.

    /tmp/pari-2.19.1/gp -q -f interface.gp > interface.txt 2>&1
    /tmp/pari-2.19.1/gp -q -f basis.gp > basis.txt 2>&1
    g++ -std=c++17 -O2 -Wall -Wextra -Werror gram.cpp -lgmpxx -lgmp -o /tmp/ecc2k130-gram
    /tmp/ecc2k130-gram > gram.txt

Optimized and UBSan output compare byte-identical. No separate automated
unit tests were added: the discriminant computations and checked division
conditions are the meaningful exact proof checks for this change.

Each bounded class/witness process uses the following wrapper, replacing
INPUT and OUTPUT with the named retained .gp/.txt files:

    ulimit -v 1048576
    timeout --signal=TERM 30s /tmp/pari-2.19.1/gp -q -f INPUT > OUTPUT 2>&1

Explicit-basis attempt exit124, subfields exit124, corrected subfield_generators exit0,
corrected degree65 exit124. Positive witness checks are exact; provisional
class numbers are not asserted as unconditional.

    awk -f extract_witnesses.awk subfield_generators.txt > witnesses.gp
    /tmp/pari-2.19.1/gp -q -f replay_witnesses.gp > replay_witnesses.txt 2>&1

This last replay uses no bnfinit or class-group data. It checks the
explicit expressions by irreducibility, characteristic polynomial,
norm and exact Sturm positivity. All five retained witnesses pass.

No Lean installation is available, and no full geometric formalization
or signed verifier receipt is claimed. No curve equation, gluing kernel,
Torelli realization, DLP input or evaluated subgroup map is supplied.

The generated472468-byte basis object is local/gitignored, reproducible
from the committed small source, and not represented as archived to S3.
The existing unavailable AWS identity is documented in round03. PDF and
editable SVG are descriptive research artifacts, with no performance data.
