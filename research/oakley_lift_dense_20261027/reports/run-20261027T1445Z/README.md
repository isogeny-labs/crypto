# Run run-20261027T1445Z: twisted-line applicability, Oakley groups 3 and 4

Binary built from `src/main.rs` (sha256 in `sha256.txt`), `cargo build --release`,
rustc 1.97.0, 4-core Xeon 2.10 GHz VM (`host.txt`), run locally (see Limits).
Raw output: `rows.json` (one object per curve), `stderr.log`, `stdout.log`.
Seed `0xAC120261027`. Samples 200; max 2000 lines per curve.
Protocol: `../../PROTOCOL.md`. Parameters: RFC 2409 6.3/6.4 (b, field polynomial,
order, generator x); c2pnb from the copied params file.

## Results, copied from `rows.json`

All curves: field polynomial irreducible (x^(2^N)=x and every gcd is 1:
true); [order*cofactor]P = O for generator and random points: true on every
curve (Oakley cofactor 1, so the RFC order annihilates the sampled points);
P(t) vs direct G cross-check and direct root verification: true.

| curve | N | d | n | Tr_N(a) | lines tried | first u at line | u found, T(u)=0, T(c/u)=0 | twisted lift (first u) | twisted lift (u2, u3) | standard base lift | control random u in ker T |
|:--|--:|--:|--:|--:|--:|--:|:--|--:|:--|--:|--:|
| Oakley 3 | 155 | 31 | 5 | 0 | 6 | 1 | true, true, true | 200/200 | 200, 200 | 104/200 | 102/200 |
| Oakley 4 | 185 | 37 | 5 | 0 | 3 | 1 | true, true, true | 200/200 | 200, 200 | 99/200 | 111/200 |
| c2pnb176v1 | 176 | 44 | 4 | 1 | 3 | 1 | true, true, true | 0/200 | 0, 0 | 122/200 | 106/200 |
| c2pnb208w1 | 208 | 52 | 4 | 0 | 1 | 1 | true, true, true | 200/200 | 200, 200 | 108/200 | 91/200 |
| c2pnb272w1 | 272 | 68 | 4 | 1 | 2 | 2 | true, true, true | 0/200 | 0, 0 | 103/200 | 93/200 |
| c2pnb304w1 | 304 | 76 | 4 | 1 | 5 | 1 | true, true, true | 0/200 | 0, 0 | 98/200 | 97/200 |
| c2pnb368w1 | 368 | 92 | 4 | 0 | 3 | 3 | true, true, true | 200/200 | 200, 200 | 109/200 | 97/200 |

"Lines tried" is the number of lines drawn until 3 verified u were collected
(roots found in total: 4, 4, 5, 3, 3, 3, 3 in table order).

Found u (hex, first of three, from `rows.json`):
- Oakley 3: `74c82bcfcaedc7af1110968da27a25afb61a9df`
- Oakley 4: `17ade0c11af2e7083b1daf986dcd3b9804665e58ce36786`
- c2pnb208w1: `b9458681d1de15fc2537629c81bca06029d18bd11412f0adfa43`
- c2pnb368w1: `83d63fcff6b92c5b42560886369de240c5d46c40b4d7e6d5b62ba0a49f2f4f7557d92380357fe978dd725db5d08c`
- c2pnb176v1 / 272w1 / 304w1 (u valid, lift 0/200 as Tr_N(a)=1): see `rows.json`.

Timing (seconds, this host): search to first u 0.046 (Oakley 3), 0.051
(Oakley 4), 0.055 / 0.138 / 0.491 / 0.306 / 1.899 (c2pnb 176v1 / 208w1 /
272w1 / 304w1 / 368w1); search for all three u 0.275 and 0.159 on Oakley 3 and
4; whole-curve time (including order checks and sampling) 2.63 and 3.95.
Total run: real 1m4.5s (tail of `stderr.log`; `stdout.log` is empty).

## Reading

- Positive control passes exactly as predicted: 208w1 and 368w1 give 200/200;
  176v1, 272w1, 304w1 (Tr_N(a)=1) give 0/200 for all three valid u. The standard base
  gives 98 to 122 of 200 on every curve.
- Both Oakley curves have Tr_N(a)=0 (a=0). A u with T(u)=T(c/u)=0 exists on each
  (4 roots on the first lines), is verified directly in F_{2^N}, and 200/200
  sampled line elements lift on Oakley 3 and on Oakley 4, for all three u per
  curve, against 104/200 and 99/200 for the standard base x in F_q.
  By the decision rule the twisted line is **applicable** to both.
- Oakley n = 5 lane: a decomposition over the twisted line has all nonzero
  x lifting, so 2^5 = 32 times as many relations per decomposition trial as the
  standard base (where about half the x lift). This is a per-trial
  relation-yield statement for the prediction 2^n; it is not a cost
  measurement. The factor-base size, the cost of the Weil-descent polynomial
  system, linear algebra and the unpriced native work (AGENTS.md 5a band) are
  not measured here, and the u-line is not F_{2^p}-rational like the subfield
  base V = F_{2^p} that lane plans, so whether it composes with that lane's
  descent (the line is an F_q-subspace, q = 2^31 or 2^37) is open. No
  comparison to Pollard rho is made or implied. The atlas also prices a GHS
  genus-16 transfer for these groups
  (`research/ghs_oakley_extended_20261023/REPORT.md`); this run does not touch it.

## Limits

- 200 samples per line check a proved theorem; they are not a proof.
- The group orders were checked by [order]P = O on the RFC generator and 3
  random points per curve (consistency of the transcription), not by point
  counting.
- The G polynomial is obtained by expanding the product, not by 5-point
  interpolation as first proposed; the two are equivalent and the expansion was
  checked against direct evaluation at 3 random t per line.
- The run was local (about one minute, seconds per curve): a feasibility
  check of a proved identity, not a benchmark or leaderboard row, so it did
  not go through isolab-eks and has no VALIDATION.json.
