# Faster checkouts: blobless clones and sparse profiles

On 2026-10-16 (`main` at `f25d5e520`) a full checkout is 6.65 GB in 100,218
files; 6.11 GB of it is `research/`. The 829 files over 1 MiB hold 4.82 GB
(72.5% of the bytes). Building ecbench, running `cargo check`, or measuring a
new session reads almost none of it. Two git features avoid it without
rewriting anything:

- a **blobless clone** (`--filter=blob:none`) downloads the commits and trees
  of the whole history, and file contents only for the paths checked out;
  git fetches any other blob on demand when a command reads it;
- a **sparse checkout** writes only the paths a profile names. An excluded file
  stays in `HEAD` and in the index; `git ls-files` lists it.

Measured on this host (one clone each, from github.com through the session
proxy, `main` at `f25d5e520`; report and graph in
[docs/checkout/20261016-checkout-size/](checkout/20261016-checkout-size/)):

| recipe | files on disk | working tree | `.git` | clone time | `cargo check --all-targets` |
|:--|--:|--:|--:|--:|:--|
| `git clone` | 100,218 | 6.65 GB | 4.06 GB | 190 s | not run |
| blobless, profile `harness` | 81,435 | 2.52 GB | 0.75 GB | +46 s on the minimal clone | passes |
| blobless, profile `minimal` | 26,078 | 0.46 GB | 0.14 GB | 33 s | passes |
| blobless, bare cone (7 dirs) | 3,434 | 0.22 GB | 0.06 GB | 9 s, 8 s | fails (`cargo check` passes) |

## Start here: blobless clone with the minimal profile

```sh
scripts/sparse-checkout.sh clone https://github.com/aburan28/crypto crypto --profile minimal
cd crypto
cargo check --all-targets
scripts/sparse-checkout.sh add --path research/notes/ecc2k130/   # a study the task needs
```

The `minimal` profile is the root files, the dot-directories (`.github/`,
`.claude/`, `.cargo/`) and `src/`, `docs/`, `scripts/`, `tools/`, `benches/`,
`examples/`, `tests/`, `store/`, plus, recomputed from the sources on every
`apply`:

- the directories of the Cargo targets declared outside those directories
  (`experiments/koblitz-*`, whose sibling modules sit beside them);
- every `include_str!`/`include_bytes!` target and every `#[path = "…"]`
  module, with its directory (they must exist to compile);
- every `research/` path a string literal in `src/` or `tests/` names
  (`cargo test` reads them).

Every other directory, the rest of `research/` included, stays off disk until
named with `add --path` (end a directory with `/`). Use it for code, docs and
CI work and for any study whose directory you name.

## The equivalent with plain git

```sh
git clone --filter=blob:none --sparse https://github.com/aburan28/crypto crypto
cd crypto
git sparse-checkout set src docs scripts tools benches examples tests
git sparse-checkout add research/<the study the task needs>
```

This cone-mode set is the smallest that builds the library and binaries
(`cargo check` passes), but `cargo check --all-targets` and `cargo test` do
not: four Cargo targets live in `experiments/`, and examples and tests compile
in about 200 files from `research/` and `docs/`. Add
`experiments/koblitz-n53-compact-cold-20261009` and
`experiments/koblitz-s3-pair-query-20261007-v4` for the target errors, or
replace the `set` line with the script's rules, which a cone list cannot
express file by file:

```sh
scripts/sparse-checkout.sh patterns --profile minimal | git sparse-checkout set --no-cone --stdin
```

## The harness profile

`harness` (the default for `apply`) is everything except the `research/`
directories holding 20 MB or more at HEAD (`HEAVY_MB`), plus the same Rust
references inside them. The list is computed from HEAD on every `apply`, so a
new large study drops out without a code change (39 directories on
2026-10-20, up from the 18 the hand-written list of 2026-10-09 named). A
partial clone cannot size a blob without downloading it, so there the script
reads `scripts/sparse-heavy-dirs.txt`, a snapshot `heavy --write` refreshes
from a full clone; `heavy` prints the list in use. On a cold page cache the
computation reads the pack headers once (81 s on a 3 GB pack here, 0.4 s
warm). It keeps
all other studies and new `research/<topic>_<date>/` directories, where ecbench
sessions land, so `git add` accepts them without naming them first.

```sh
scripts/sparse-checkout.sh apply [--profile minimal|harness] # rewrite the rules
scripts/sparse-checkout.sh status                    # what is on disk
scripts/sparse-checkout.sh add --path research/X/    # materialize more, same profile
scripts/sparse-checkout.sh patterns [--profile P]    # print the rules, change nothing
scripts/sparse-checkout.sh disable                   # full checkout again
```

Applying a profile to an existing full clone frees disk only; the history was
already downloaded. For less network, clone blobless.

## Claude Code cloud sessions

A cloud session clones the repository itself, before any hook runs. Two
session settings shrink that clone (they are parameters of a new session, for
example `create_session` when one session starts another):

- `blob_limit_kb`: files larger than this are left out and fetched when a
  command reads them. `1024` alone skips the 829 files over 1 MiB, 72.5% of
  the bytes, and leaves every path in place.
- `sparse_checkout_paths`: a cone-mode list, the directories plus the files
  directly in their parent folders and at the root. Give it the plain-git
  list above plus the research directories the task needs, and add the two
  `experiments/` directories when the task runs `cargo test` or builds
  examples. When it is set, `blob_limit_kb` is ignored; prefer it when the
  task's directories are known.

On 2026-10-20 a session started without either setting downloaded the whole
of HEAD: a 3.17 GB pack for 7.5 GB on disk, of which 42 MB was blobs that only
older commits hold. History is not the cost; the files at HEAD are. A local
depth-1 clone of the same commit fetched 3.9 MB with `blob:none`, 167 MB with
`blob:limit=100k` and 488 MB with `blob:limit=1m`, but a full checkout of the
`blob:limit=1m` clone then fetched every skipped blob back (`.git` 3.0 GB). A
filter only saves the download when the skipped files also stay off disk: use
`blob_limit_kb` for sessions that read no large research file, and
`sparse_checkout_paths` (or a profile) otherwise. Whether a session started
from the claude.ai web UI can set either is an environment setting this
document has not checked.

`CRYPTO_SPARSE_PROFILE=harness` (or `minimal`) in the environment's variables
makes `.claude/hooks/session-start.sh` apply that profile after the clone.
That frees disk, not download time.

## Sibling repositories

`scripts/setup-workstation.sh` (run by the session hook) clones the missing
sibling repositories, `cryptanalysis` by default, as partial clones: files over
`CLONE_BLOB_LIMIT` (1m) stay on the server, and a sparse rule per file keeps
them off disk so checkout does not fetch them back. `git ls-files` still lists
them; `git sparse-checkout disable` fetches them all. `CLONE_FULL=1` clones
everything.

## Off disk is not absent

An excluded file stays in `HEAD` and in the index; `git ls-files` lists it.
Before replaying, comparing against, or citing an existing session or note
under an excluded directory, `add --path` it. A path off disk is never
evidence that a record does not exist.

Examples that enumerate whole archives (`examples/n37_shared_rank_*` read
`research/notes/ecc2k130/`) need `add --path research/notes/ecc2k130/` before
they run, and studies whose verifiers hash their files in place need their
whole directory.

## CI

CI jobs keep their own per-job `sparse-checkout:` lists in
`.github/workflows/` and are unaffected by the profiles. On 2026-10-16 the
1,133 pattern lines in 63 workflows were checked against the tracked tree:
every one that names a tracked path still matches; the 97 that match nothing
are optional root files that do not exist (`/build.rs`, `/rustfmt.toml`,
`/clippy.toml`, `/rust-toolchain*`, `/.cargo/`, `/LICENSE*`, `/*.rs`) and
`/docs/ic/measurement/reports/`, which `reports-s3.yml` fills at run time.

Every checkout that asks for the full history (`fetch-depth: 0`) without a
sparse list also sets `filter: blob:none` (86 checkouts in 76 workflows on
2026-10-20): commits and trees come in full, file contents only for HEAD, and
any older blob a step reads (`git show <rev>:<path>`) is fetched on demand.
Their history uses were `git merge-base`, `git cat-file -t/-e` and diffs
against the working tree, which need no old blobs.

The `large-files` workflow runs on every pull request with a blobless,
two-file sparse checkout and fails when the PR adds or rewrites a file over
5 MB that `.github/large-files-allowed.txt` does not name (AGENTS.md §13).
Replayed over the last 60 merges to `main` before it landed: 55 passed, 4
passed only through the allow list (catalog regenerations), and 1 would have
failed (`papers/ecdlp-six-month-portfolio/evidence/catalogue.tar.xz`, 13.3 MB,
and `reading-coverage.tar.xz`, 7.4 MB).

## New evidence: archives, and a proposal for Git LFS

New raw run trees land as one deterministic `runs.tar.xz` with its `.sha256`
unless claims or verifiers resolve their files by path (AGENTS.md §13a), and
files too big for Git go through the store (AGENTS.md §13). Committed
evidence is never re-packed, moved, or rewritten out of history.

**Proposal, not implemented (needs the owner's yes): Git LFS for new large
evidence.** Git LFS is installed in the cloud image and no file uses it. A
`.gitattributes` rule such as
`research/**/*.jsonl filter=lfs diff=lfs merge=lfs -text`, scoped to new
directories (dated after the switch), would keep large new blobs out of the
pack: clones download pointers, and `git lfs pull --include=<path>` fetches
contents on demand. Trade-offs to decide first:

- LFS storage and bandwidth are metered on GitHub (and need a data pack past
  the free quota), where the store's S3 bucket is already paid for;
- CI jobs that read LFS files need `lfs: true` on checkout, and offline or
  third-party replays need LFS access;
- existing files must stay as they are: converting them (`git lfs migrate
  import`) rewrites history, which AGENTS.md forbids;
- a blobless clone with a sparse profile already avoids downloading large
  blobs a task does not read, so LFS mainly helps full clones.

The alternative that needs no new service is now in force: AGENTS.md §13 sets
the threshold at 5 MB and the `large-files` check enforces it (see CI above).
