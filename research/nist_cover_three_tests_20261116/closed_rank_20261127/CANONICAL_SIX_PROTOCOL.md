# Six-symbol bound for the pole divisor H+2R

Frozen before implementation or execution. Continue the fixed original
225-column P-192 search after the complete 2E_ref shifted-six test retained
only the formal zero row and closed rank six. [pcc:unverified]

Let H be the first retained split canonical line section on the actual
smooth plane quartic. Let R be original point 0, whose exceptional-map
evaluation gives the elliptic identity. Verify H's four actual support
points and line restriction, and evaluate all four images plus R freshly.
Use D=H+2R, a degree-six divisor with image sum kappa=f_*(H). By
Riemann–Roch in genus three, l(D)=4. Zeros of a function in L(D), padded
by cancelled poles, have degree six and image sum kappa. The prior
kappa-target bound used four symbols; it does not exhaust this six-symbol
family. [pcc:unverified]

Reuse the checked native shifted triple enumeration and two exact joins
from shifted-six-v2. Compute rho=[(r+1)/2]kappa and verify [2]rho=kappa
with GMP and named OpenSSL. The half is a public group operation, not a
discrete-log calculation or a rational 2-torsion assumption. Subtract the
coefficient word for H. Derive its six-slot formal witness from H's four
signed image labels and two identity labels; check it before enumeration.

Check every unordered triple on O and both signs of all 225 original
elliptic columns. Both group implementations must agree on every shifted
sum and ordered stream hash; grouped and indexed-merge joins must agree
on every sign-normalized coefficient row and lexicographically minimal
witness. Verify all retained words/rows in both group implementations.
Require all seven original four-symbol section rows (up to row sign),
including zero, padded by identity symbols. Reuse the nonzero public G/2G
fixture and exhaustive small six-word comparison. Mutated row, missing
candidate/triple and altered orientation must reject.

Keep the original six-row span and 108x429 tangent matrix unchanged.
Check ranks and all actual row sums. Add only independent original-column
candidate rows to exact capped bases and compare old-column rank with
full minus auxiliary rank. Every positive candidate still needs actual
RR/KM realization and separate elliptic/elimination replay. Fixture rows
never enter the actual matrix. A null bounds this pole family only when
each individual zero image belongs to the listed signed rational domain;
it also covers repeated slots and closed source points with those images.

Retain one CPU, 1 GiB virtual address space, 300-second strict native build,
600 seconds per worker, dense helpers <=256 rows and <=1024 columns,
and <=4096 retained rows. Native C++ performs all mathematical work. Freeze
source/input/binary/output hashes, argv, exits and costs; preserve failures.
Do not rerun completed zero-target or 2E_ref searches. Run locally with
Conductor ignored per the explicit user instruction.

Sources checked: current `REPORT.md`, `SIX_PROTOCOL.md`,
`SHIFTED_SIX_PROTOCOL.md`, frozen shifted-six-v2 source/result, parent
projective/KM exceptional-point reports, and frozen baseline split-section
and original point-0 records. Standard principal-divisor pushforward and
Riemann–Roch give the condition. The existing report links the primary
Diem and double-large-prime papers. KB MCP tools and wrapper executable
are unavailable as recorded in the journal; no novelty claim.
