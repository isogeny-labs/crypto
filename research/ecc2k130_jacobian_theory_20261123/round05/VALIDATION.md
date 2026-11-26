# Producer replay

Run from this directory:

    g++ -std=c++17 -O2 -Wall -Wextra -Werror check.cpp -lgmpxx -lgmp -o /tmp/ecc2k130-hyperelliptic-check
    /tmp/ecc2k130-hyperelliptic-check > check.txt
    g++ -std=c++17 -O1 -g -fsanitize=undefined -fno-sanitize-recover=all -Wall -Wextra -Werror check.cpp -lgmpxx -lgmp -o /tmp/ecc2k130-hyperelliptic-ubsan
    /tmp/ecc2k130-hyperelliptic-ubsan > check_ubsan.txt
    cmp check.txt check_ubsan.txt
    /tmp/pari-2.19.1/gp -q -f replay.gp > replay.txt 2>&1

The native checks cover exact Weil quotient, coefficient-based2-ranks,
point counts and parity, the two degree2 fiber bounds, prime-degree
Rabin irreducibility, all eight fixture extension counts, direct-y and
trace-based counts, and original E0 Weierstrass/trace controls.
PARI independently replays the source counts, polynomial irreducibility
and exact combinatorial presentation count. It does not claim an
independent geometric proof. No arithmetic mismatch or sanitizer issue.

No adaptive family search, class-group retry, paid cloud job, timing
comparison or DLP solve occurred. These are fixed mathematical checks.
The elementary hyperelliptic and ramification arguments remain written
and unsigned; Lean is not installed and no signed verdict is claimed.
The original general genus130–300 curve/map goal remains unresolved.

Primary-paper full texts are read in scratch and cited, not copied into
Git. The existing GHS/MMT correction was consulted before selecting the
new fiber/branch scope. An inaccessible IACR PDF and missing arXiv html
version are documented in the session log; neither supplies evidence.
