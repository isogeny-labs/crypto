# Native CLI correction before the valid NFS control

The first setup control produced no arithmetic result: with noninteractive
stdin, pinned YAFU reads expressions from stdin rather than the positional
command argument. It immediately reached EOF. Preserve this failed setup
attempt and feed one exact `nfs(N)` expression through stdin thereafter.

The pinned CLI accepts one 64-bit seed, unlike the older two-word syntax
in its bundled documentation. The original two-word argument was parsed
as the integer `20261022`. Explicitly use **20261022** for both the valid
control and every production/resume attempt. Record the actual seed and
exact stdin alongside the command. No prior NFS factorization was performed.

Inputs, allocations, thread counts, acceptance criteria and proof obligations
from PROTOCOL-NFS.md are unchanged. The runner additionally uses a 4 GiB
address-space limit per native process. Setup failures are not production
negative results. The `,+native` text in that protocol is a typographical
error meaning `, native`; it changes no requirement.
