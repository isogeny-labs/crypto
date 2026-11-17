# H115 Frobenius--Kummer resolvent moments

H115 tests a new frozen observable, not a result. On a prime-order curve it
maps the `[r]`-division Kummer cocycle through the exact factorization

```text
(V-1)(pi-1)=[r]
```

and studies low-order power sums over the degree-`r` fiber of `V-1`. The
candidate family has 648 fixed radix-six predicates, evaluated on five tiny
curves, 15 generator rows and 366 nonidentity row-targets with matched
random-line, label, rational-line and coordinate-isomorphism controls.

![H115 mechanism and evidence gates](mechanism.png)

The key distinction is cost: compressing an `r^2` torsor to a degree-`r` fiber
does not make it subexponential. The frozen producer must count its explicit
`Theta(r)` fiber enumeration. A speed claim additionally requires a uniform
`r^{o(1)}` construction of the selected moment and complete decoder.

Files:

| Artifact | Role | SHA-256 at freeze |
| --- | --- | --- |
| `PRIOR_ART.md` | Primary-source overlap and scoped novelty screen | `7637f17a6cd0051ae9ea048e6726eb945632a320c591caea96e585d66f258697` |
| `PROTOCOL.md` | Human mechanism, controls, acceptance and cost boundary | `5879cc70605291f295aa286516ec88ca2243e273924f511927bd119807acf5e7` |
| `PROTOCOL.json` | Machine-readable frozen family and run policy | `c8412d94f3e6cc928c13c93572a8796cea3a983b5fcd4e6b043a75bc5b1a2363` |
| `CURVES.json` | Five exact structural toy fixtures | `babd2556951ecfcaee66a5c6966364ca136bf1f62fb0a51511f391768255d295` |
| `PROOF_MAP.md` | Generic identities, score handoff, decoder and cost obligations | `99319afcea8e17b60c668a446535b4c1bb2864f0b4aa877ee193407e08f1a7e0` |
| `mechanism.svg` | Editable algebra/evidence diagram | `7cdc6fb0c6a4f2719e72641119649825b51f81e46f78d5210ee56eb367da3d7e` |
| `mechanism.png` | Inspected 1,200-by-640 rendering | `6dfa6ac9033d15f9424125b290a7546fbb54f2dc394862336ba002567f23ef79` |

No candidate moment scores, selected IDs, decoder outcomes or runtime
measurements exist. External novelty and a generic/subexponential ECDLP
algorithm are not claimed.
