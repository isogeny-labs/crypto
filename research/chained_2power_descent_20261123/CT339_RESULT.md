# CT339 — projective infinity bridge and complete F103 public toy-DLP transfer

**Executed 2026-11-23 UTC on #4997; [pcc:unverified].** Preregistered [CT339_PROTOCOL.md](CT339_PROTOCOL.md) at commit `90573ac243fd71659d61937b8faac45070a3e960` before the standalone source or this outcome. The predecessor source revision was `f4b4e7d53d277a1091417426793554063e7b06e3`. One coordinator authored/executed the independent-language Python oracle, not a separate native worker or a signed PCC validation.

## Genuine end-to-end result (deliberately bielliptic toy instance)

On `C/F103: y²=∏_{j=1}^{6}(x-j)`, the fresh standalone Python implementation uses exact prime/quadratic-extension arithmetic, odd-degree genus-2 **Cantor composition/reduction**, and two separately implemented (nonmonic-capable) cubic elliptic group laws. It supports Mumford degree 0, 1 and 2, split/repeated/nonsplit quadratic support, and special source `z=0` (the original two infinities). The computation includes the required degree-one subtraction of the image of `W1=(1,0)`.

- Enumerated `#E+(F103)=92=4·23` and `#E-(F103)=100=4·25`, checked point equations and the three two-torsion kernel pairs.
- First eight nonbranch rational source points with x≥2 were `(10,15),(17,51),(19,39),(20,36),(22,29),(25,5),(26,36),(29,31)`.
- **168/168** fixed `Φ([k]D_i)=[k]Φ(D_i)` checked with actual genus-2 Cantor source scalar multiplications for `0≤k≤20`. Output classes were 8 identity, 8 degree-one, 47 split, **89 genuine nonsplit F103² supports** and 16 repeated degree-two supports. All were supported, none silently excluded.
- First eligible projection: `G=[100](P_{x=10}-W1)`, represented on the transformed odd model by reduced `U(z)=z²+42z+74`, `V(z)=61z+85`, with `Φ(G)=((51,25),O)`. **Both** `[23]G=0` via source Cantor and `[23]Φ(G)=0` via elliptic law passed; `Φ(G)≠0`.
- For **all 23 public scalars** `k=0..22`, constructed `Q=[k]G` on the genus-2 Jacobian, evaluated `Φ(Q)` from its actual reduced Mumford representation (including nonsplit support), solved `[k]Φ(G)=Φ(Q)` using a separate small-order product-elliptic BSGS, recovered the correct integer and verified the source identity `[k]G=Q`. **23/23 complete toy scalar recoveries**; not a new index-calculus solver.

## Projective/exceptional map qualification

For each `p∈{103,2147483647}`, in `z=(x-1)^(-1), η=yz³`, the two original rational infinities are the *finite* odd-model supports `(z,η)=(0,±1)`. Their quotient images are `π+=(O,O)` on `E+` and `π-=(0,±1)` on `E-`. The basepoint `W1` maps to nonzero two-torsion `T+=(25/4,0)`, `T-=(4/25,0)`. The independent Cantor test verifies `[Q∞+ - W1]+[Q∞- - W1]=0` and verifies the corresponding quotient pair sums vanish. All six branch-point images have the required two-torsion behavior and each of the three paired kernel classes maps to zero.

For Prime31, `Φ([Q∞+ - W1])=((536870918,0),(1431655758,1431655722))` in the two cubic charts; opposite infinity has the negative Eminus ordinate `715827925`. All these are field elements mod `2147483647`. This qualifies this **two-infinity exceptional family**, not projective completeness for every Jacobian divisor or general `C0=0` and ramified/extension exception.

## Reproducibility, negative controls, provenance

Source: [gzip-base64 text custody](ct339_f103_toy_dlp.py.gz.b64), remote source commit `343c74411e3719a520e7dae74e1b4fb967a85db5`; decoded `ct339.py` SHA256 `c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695`. Source bytes 12,121; canonical gzip-base64 payload bytes 6,484 excluding final newline. Frozen compact stdout: [CT339_RESULT.json](CT339_RESULT.json), commit `7350a3b4ca9fad5a2842b52f07b672722325e69e`, exact JSON SHA256 `eb986754a030062db0aa629989edf1ac2705695f5d8b1863fc19dab28c085492` (954 bytes including newline).

```sh
base64 -d ct339_f103_toy_dlp.py.gz.b64 | gzip -d > ct339.py
printf '%s  %s\n' c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695 ct339.py | sha256sum -c -
python3 ct339.py > ct339_normal.json
python3 -O ct339.py > ct339_optimized.json
cmp ct339_normal.json ct339_optimized.json
sha256sum ct339_normal.json
```

**Actually executed in the local session**: `python3 /mnt/data/ct339.py`, `python3 -O /mnt/data/ct339.py`, `cmp`, `sha256sum`, `python3 -m py_compile`, all passed. Standard and optimized output byte-identical. Source-level mutation controls independently rejected wrong basepoint (kernel pair failure), altered source ordinate (Mumford relation failure) and changed elliptic tangent slope (curve-membership failure); each deliberate mutated Python run exited nonzero. Two built-in malformed/bad-basepoint controls passed. No CI/source-exact GitHub hosted run, native Rust compile, third-party PCC signature, EKS/purchased compute, benchmark or merge.

The raw field-group operation instrumentation totaled 1,006 Jacobian additions, 2,888 elliptic additions and 202 image evaluations, across *the entire panel* (includes naive diagnostic repeated scalar multiplications and BSGS), **not** a timed single-target cost. Do not use them to infer speedup. The two elliptic factors are intentionally accessible by explicit degree-two quotient maps. This experiment proves **finite public toy recovery**, not ordinary Prime31 ECDLP speedup.

## Ownership and next gate

The original 695/701 fixture custody, actual norm-point reuse `C169/C13a/C13b/N2_actual/D` physical pricing and fully charged same-host strongest optimized rho/BSGS/IC remain reserved by their existing owners. #5007 retains native F3/E7 source transfers. All shared EKS paid STOP, review, workflow budget and PCC guards unchanged.

Next unclaimed bridge: independently replay the exact published CT339 code/results in an admitted Rust or separate independent native implementation, extend full odd-Cantor infinity/`C0=0` and ramified nonsplit exception coverage, verify `Φ^\u005e∨ Φ=[2]` as actual Jacobian maps on an exhaustive small-field panel, then integrate a non-weak source ECDLP and complete cost baseline only after approved benchmark custody.

Sources: preceding CT336–338 reports in this branch; [Magma Richelot documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1615); [Tanja Lange, Formulae for Arithmetic on Genus 2 Hyperelliptic Curves](https://hyperelliptic.org/tanja/preprints/expl.pdf); [Ernst Kani, elliptic subcovers and isogeny defect](https://doi.org/10.1007/s40316-018-0105-6). Research-KB MCP/CLI and native worker launcher unavailable in this runtime; no novelty claim.
