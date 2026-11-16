# Primary sources and frozen inputs

| Source | Used statement | SHA-256 of downloaded PDF |
| --- | --- | --- |
| [Müller, *Explicit Kummer surface formulas for arbitrary characteristic*, p. 5](https://www.math.uni-hamburg.de/home/js.mueller/general_kummer.pdf) | Classical genus-2 Kummer embedding and the dimension-four linear system | `5980f6d2162fb4cdedb68f96b2259fb0e778fc1291243fc39f3545e444fbae77` |
| [Arnth-Jensen and Flynn, *Non-trivial Sha in the Jacobian of an infinite family of curves of genus 2*, Eq. (4.2)](https://jtnb.centre-mersenne.org/item/10.5802/jtnb.654.pdf) | Independently printed `F0` with the `f5*s*t²` term | `ad1ad3b40c6faea8315e4be045188c2556701cb50d57e053d34333e7bab5d7dd` |
| [Chung, Costello and Smith, *Fast, uniform, and compact scalar multiplication for elliptic curves and genus 2 Jacobians with applications to signature schemes*, §§6.3, 7.1–7.4](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf) | General Kummer Mumford map, Rosenhain/theta assumptions, direct fast projection, theta xADD, and exact linear transformation `tau` | `c140ab40dba1f22ccb8de3ed25a0930b6298593b7f084ac9832310f8d8c6c83a` |

Müller's p. 5 displayed `F0` prints `f5*s*t`, repeating the `f3`
monomial. Arnth-Jensen–Flynn Eq. (4.2), CCS's general Kummer quartic, and
the symmetric bidegree-five construction give `f5*s*t²`. The N18 exact
certificate uses that corrected term and retains the discrepancy in its
report. No source PDF is redistributed in this packet.

The frozen model source is
`../n16-action-certificate/results/exceptional-ideal.json` (SHA-256
`01643d22415da15452efe3f880efa545527c8a276e9f6fac3499e9c6d125d29c`).
The selected theta constants come from
`../../kummer_theta_20261022/results/theta-models.jsonl`.
Its SHA-256 is `b76424d8195335452c679c69b7c759a22e5721ce4811ff005304a61ab5bbab87`.
