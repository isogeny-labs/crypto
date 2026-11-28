# Decision record: `gpu/btcpuzzle`

Status: **options and a recommendation, 2026-11-22. The Principal
Cryptographer decides** (action plan 2026-11-21, row F5; workspace
`docs/strategy/2026-11-21-action-plan.md`). Nothing has been removed or
moved. Whatever is chosen is recorded in [`SECURITY.md`](../../SECURITY.md),
"Decisions under this policy".

## Why this needs a decision

The action plan ties the licence of this repository to this directory. Its
decision D5 reads: "`crypto` gets MIT, or goes private if `gpu/btcpuzzle`
and the other offensive kernels stay in it". The plan's first customer
segment (D2) is digital-asset custody, exchanges and wallet
infrastructure: the same people a Bitcoin key-search tool in the company's
public repository is most likely to worry.

## What the directory is

From [`gpu/btcpuzzle/README.md`](../../gpu/btcpuzzle/README.md) and
[`gpu/btcpuzzle/puzzles.txt`](../../gpu/btcpuzzle/puzzles.txt), read on
2026-11-22:

| Fact | Where it says so |
|:--|:--|
| A Pollard kangaroo solver for the discrete log **inside a known interval** on secp256k1, about 2·√W group operations for an interval of width W | README, opening and "How the walk works" |
| Motivated by the Bitcoin puzzle series, a public challenge created in 2015: puzzle #n's key lies in `[2^(n-1), 2^n)`, so it costs about 2^(n/2) when its public key is known | README, opening |
| "**No kernel here has been run on a GPU.**" The solver is checked end to end on the CPU, where it recovers planted keys | README, opening |
| Interval ECDLP only: "There is no address scanner and no hash160 kernel". Unspent puzzles, whose public key is hidden behind HASH160, are out of reach | README, "The two kinds of puzzle" |
| "It does not scan address lists, derive addresses, or touch a wallet or the Bitcoin network in any way" | README, "Scope" |
| The real puzzle public keys are **not shipped**; the registry holds only synthetic, locally generated keys used by `make test` | README, "The registry"; `puzzles.txt` |
| A recovered key that was not already known is written to a mode-0600 file and is not printed by default; device buffers are zeroed before release | README, "Recovered keys and GPU memory" |
| Multi-GPU in one process (one host thread per device, one distinguished-point table), modelled on the public JeanLucPons Kangaroo design | README, "Multi-GPU" |
| Measured constant on the CPU: C ≈ 2.8 ± 0.14 in `steps = C·√W` on a 2^31 interval, 120 trials, about a third above the best published implementations | README, "Measured cost" |
| Size: 10 files, 2,451 lines (CUDA headers, host code, tests, registry, README) | `wc -l gpu/btcpuzzle/*` |
| The README's own examples include `./bench solve --puzzle 66 --key-out /secure/p66.key` | README, "Build and test" |

What depends on it inside this repository:

- `.github/workflows/release.yml`: the `native` job runs `make -C gpu/btcpuzzle test`
  (line 795) and the `cuda` job's sparse checkout lists `/gpu/btcpuzzle/` (line 851);
- `docs/RELEASES.md` line 129, `docs/POLLARD_COLLAB_DESIGN.md` line 282;
- `README.md` lines 1097, 1125, 1170–1176, 1222 and 1255;
- `SECURITY.md`, "Components with dual-use capability".

Nothing in the Rust crate imports it; it reuses `gpu/ecc` headers, not the
other way round (README, opening).

## What the directory is not

- It is not a new capability. Interval kangaroo is textbook (Pollard 1978;
  van Oorschot–Wiener 1999), and the README itself points at public GPU
  solvers that implement it and have been used on the puzzle series. On the
  lab's own measurement it is slower per step than those (constant 2.8
  against about 2.1), and its README says no kernel has been run on a GPU.
- It cannot reach any address whose public key is unknown, which is every
  unspent puzzle and every ordinary unused address.

So the case for acting is about **posture and audience**, not about
denying anyone a capability. The code has also been public for as long as
it has existed: removing or moving it changes what the repository presents
from now on, not what is in its public history. Rewriting history is not
proposed.

## Options

| Option | What happens | For | Against |
|:--|:--|:--|:--|
| **A. Remove** | Delete `gpu/btcpuzzle/`; drop it from `release.yml`, `docs/RELEASES.md`, `README.md` and `docs/POLLARD_COLLAB_DESIGN.md` | Simplest public story; resolves D5's condition for this directory | Loses an interval-ECDLP solver the lab could use on authorised small-range-key and biased-nonce work (plan P8, pack 1); the code stays in history anyway |
| **B. Move to a private repository** | Copy the directory with its history (`git filter-repo --path gpu/btcpuzzle/`) to a private org repository; leave a short stub README here saying where it went and why; update the same four references | Keeps the code for authorised engagements; removes the Bitcoin-puzzle framing from the public flagship repository; resolves D5's condition for this directory; reversible | One more private repository to maintain; the private copy still needs the `gpu/ecc` headers it includes, so it pins a `gpu/ecc` revision or vendors those headers |
| **C. Keep, with a written rationale** | Keep the directory; add the rationale to `SECURITY.md`; optionally rename it to `gpu/kangaroo/`, make the generic interval solver the headline and move the puzzle registry and `--puzzle` CLI path behind a build flag | No work, no loss of a working (CPU-verified) tool; the technique is public and the safeguards above are real | Under D5 this keeps the "or go private" branch open for the whole repository; the puzzle framing reads as a key-search tool to a custody buyer, whatever the safeguards |

## Recommendation (proposed, not decided)

**Option B: move it to a private repository**, with a stub here. It is the
only option that removes the puzzle framing from the public repository
without discarding a tool the lab can use under the engagement rules in
`SECURITY.md`, and it takes this directory out of D5's "or go private"
condition, so the MIT licence in [`LICENSE`](../../LICENSE) can stand for the
rest of the repository if the owners also accept the other dual-use
components (next section).

If the owners prefer to keep the code public, option C with the rename and
the puzzle path behind a flag is the better form of keeping it.

## The other offensive kernels D5 mentions

D5 says "`gpu/btcpuzzle` **and the other offensive kernels**". The other
components are listed, with what they do and their safeguards, in
`SECURITY.md`, "Components with dual-use capability". On reading them, all
but one implement published techniques against public challenge curves, toy
parameters, simulated oracles or the library's own implementations, or
solvers whose full-size targets (P-256, secp256k1) are out of reach at about
2^128 group operations. Keeping them public under MIT with that table is
proposed. The exception to look at is `src/cryptanalysis/multi_key_hnp.rs`
(joint biased-nonce key recovery aimed at BIP32 HD-wallet master secrets),
which is a real-world wallet target framed the same way as this directory:
it should get the same decision.

## What the decision unblocks

- F3: whether `LICENSE` (MIT) merges as is, or the repository goes private.
- F5 "done when": the decision written in `SECURITY.md` and the code change
  merged.
- The follow-up PR that carries out A, B or C (not started; this record
  changes no code).
