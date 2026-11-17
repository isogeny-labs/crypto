#set page(paper: "us-letter", margin: 0.62in)
#set text(size: 9pt)
#set par(leading: 0.45em)
#text(size: 16pt, weight: "bold")[ECCp-131 auxiliary tower: transfer audit]
#linebreak()
_23 October 2026 | exact field and rational-point groups_

The certified eight-step 2-isogeny tower lives on $E': y^2=x^3+x+301$ and its successive auxiliary quotients over the published field. Its 256 coset abscissae supply 115 lifted coordinates on the published ECCp-131 curve. This assessment asks whether that tower itself transports the published discrete-log subgroup.

#figure(image("transfer_path.svg", width: 7.0in), caption: [Verified auxiliary degree-2 arrows; dashed coordinate selection into the published curve. The nontrivial group-map path is excluded by exact rational-point orders.])

The independent published-curve PARI/GP check gives prime $n=1550031797834347859219047037805205710577$. The auxiliary order output gives $m=1550031797834347859229712325794928046336$, divisible by 256. The replay certificate binds both to the same prime field; `check_transfer_obstruction.py` derives $gcd(n,m)=1$ from it. A direct GP recount returned `TRANSFER_ORDER_OK` under a 30 s wall timeout. Sandbox restrictions prevented a memory cap for that extra check; all attempts are retained in `TRANSFER_EXECUTION.md`.

For any group homomorphism $f:E(F_p) arrow E'(F_p)$, the image order divides both $n$ and $m$, hence is one. This also holds in reverse. In particular these exact curves cannot be $F_p$-isogenous: isogenous curves over $F_p$ have equal point counts. Every curve in the published isogeny class has odd order $n$, so a rational 2-isogeny with a rational order-2 kernel cannot originate there.

The eight auxiliary maps remain valid. Lifting a shared integer $x$ onto $E$ is a partial coordinate-selection operation, with no subgroup-preserving homomorphism or relation transport. The resulting 115-point base is still a legitimate published-curve factor-base candidate; its fixed screen found zero natural setup or target rows and rank zero. Higher arity, a larger base, or another relation mechanism needs an explicit support and cost calculation before a further exact-curve run.

*Evidence:* `runs/search.stdout`, `runs/identity_check.stdout`, `runs/replay.json`, `runs/transfer-order-screen.json`, `TRANSFER_AUDIT.md` and `draw_transfer.py`. Measured auxiliary search and replay/screen wall were 83.927 s and 8.276 s; these are component costs only. Group-order obstruction is derived; the full relation, independent rank, target descent and cold cost remain open.
