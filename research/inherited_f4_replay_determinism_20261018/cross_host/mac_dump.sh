#!/usr/bin/env bash
# mac_dump.sh -- per-node inherited-F4 word-count dump, for cross-host diffing.
#
# Runs the committed ecbench spec docs/ecbench/specs/claim-koblitz-rootfind-n13.json
# (the spec of archived session ECBS1hfd5bf1885227, Apple M4 Pro / aarch64) at
# the archived commit, restricted to its inherited-F4 arms, with
# RAYON_NUM_THREADS=1, and records for every solver node the process-wide
# word-op counter at node entry plus a hash of the node system. Works on macOS
# (aarch64 or x86-64) and Linux (aarch64 or x86-64); the same script is the
# x86-64 side.
#
# Usage:
#   bash mac_dump.sh [--work DIR] [--out DIR] [--install-toolchain] [--subset SPEC]
#                    [--sha FULLSHA] [--toolchain 1.93.1] [--keep-build] [--list-subset]
#
#   --work DIR            checkout + build dir (default: ./f4-xhost-work)
#   --out DIR             where the result files go (default: ./f4-xhost-out)
#   --install-toolchain   allow `rustup toolchain install <toolchain>` if missing
#   --subset SPEC         which inherited-F4 runs: "all" (every seq of arms ic-f4-m2
#                         and ic-f4-m3, 64 runs incl. warm-ups) or a comma list of
#                         seq numbers. Default: the value of DEFAULT_SUBSET below.
#   --keep-build          keep the cargo target dir (default: deleted at the end)
#
# Output (in --out):
#   node-dump-<arch>-<os>-<shortsha>.jsonl.gz   the dump (one file)
#   host-<arch>-<os>.json                       host/toolchain/commit/sha256/line count
# Copy both into research/inherited_f4_replay_determinism_20261018/cross_host/
# and commit them.
set -euo pipefail

PIN_SHA="2245b7596f3fe04586c33b53a7cac97185fadba2"
REPO_URL="https://github.com/aburan28/crypto"
SPEC_REL="docs/ecbench/specs/claim-koblitz-rootfind-n13.json"
SPEC_SHA256="62fdb5aec2f6313000bf1ba67d7ca9385f9385225aef1a631ca45d42cdfa1b4d"
TOOLCHAIN="1.93.1"
DEFAULT_SUBSET="all"

WORK="./f4-xhost-work"; OUT="./f4-xhost-out"; INSTALL_TC=0; SUBSET="$DEFAULT_SUBSET"; KEEP=0; LIST=0
while [ $# -gt 0 ]; do
  case "$1" in
    --work) WORK="$2"; shift 2;;
    --out) OUT="$2"; shift 2;;
    --install-toolchain) INSTALL_TC=1; shift;;
    --subset) SUBSET="$2"; shift 2;;
    --sha) PIN_SHA="$2"; shift 2;;
    --toolchain) TOOLCHAIN="$2"; shift 2;;
    --keep-build) KEEP=1; shift;;
    --list-subset) LIST=1; shift;;
    -h|--help) sed -n 2,32p "$0"; exit 0;;
    *) echo "unknown option: $1" >&2; exit 2;;
  esac
done

die() { echo "mac_dump.sh: FAIL: $*" >&2; exit 1; }
say() { echo "[mac_dump] $*" >&2; }
sha256_of() { if command -v sha256sum >/dev/null; then sha256sum "$1" | awk '{print $1}'; else shasum -a 256 "$1" | awk '{print $1}'; fi; }

# ---- prerequisites --------------------------------------------------------
missing=0
need() { command -v "$1" >/dev/null 2>&1 || { echo "missing: $1 -- $2" >&2; missing=1; }; }
need git "macOS: xcode-select --install   Linux: apt install git"
need cargo "install rustup: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   (then restart the shell)"
need gzip "ships with macOS/Linux"
need awk "ships with macOS/Linux"
command -v sha256sum >/dev/null 2>&1 || command -v shasum >/dev/null 2>&1 || { echo "missing: sha256sum or shasum" >&2; missing=1; }
if [ "$(uname -s)" = Darwin ]; then
  xcode-select -p >/dev/null 2>&1 || { echo "missing: Xcode command line tools (linker) -- run: xcode-select --install" >&2; missing=1; }
fi
[ "$missing" = 0 ] || die "install the prerequisites above and re-run"

ARCH="$(uname -m)"; [ "$ARCH" = arm64 ] && ARCH=aarch64
OS="$(uname -s | tr 'A-Z' 'a-z')"; [ "$OS" = darwin ] && OS=macos
SHORT="${PIN_SHA:0:7}"

# ---- toolchain ------------------------------------------------------------
TC_NOTE=""
if command -v rustup >/dev/null 2>&1; then
  if ! rustup run "$TOOLCHAIN" rustc -V >/dev/null 2>&1; then
    if [ "$INSTALL_TC" = 1 ]; then
      say "installing rust $TOOLCHAIN (minimal profile)"
      rustup toolchain install "$TOOLCHAIN" --profile minimal >&2 || die "rustup toolchain install $TOOLCHAIN failed"
    fi
  fi
  if rustup run "$TOOLCHAIN" rustc -V >/dev/null 2>&1; then
    export RUSTUP_TOOLCHAIN="$TOOLCHAIN"; TC_NOTE="pinned $TOOLCHAIN via rustup"
  else
    TC_NOTE="NOT PINNED: rust $TOOLCHAIN is not installed (pass --install-toolchain); using the default toolchain"
    say "WARNING: $TC_NOTE"
  fi
else
  TC_NOTE="NOT PINNED: no rustup; using $(command -v cargo)"
  say "WARNING: $TC_NOTE"
fi
RUSTC_V="$(rustc -V)"; CARGO_V="$(cargo -V)"
say "toolchain: $RUSTC_V / $CARGO_V ($TC_NOTE)"

# ---- checkout at the pinned commit, with the dump patch --------------------
mkdir -p "$WORK" "$OUT"
WORK="$(cd "$WORK" && pwd)"; OUT="$(cd "$OUT" && pwd)"
SRC="$WORK/src"
PATCH="$WORK/node-dump-compact.patch"
emit_patch() { sed -n "/^: <<'__PATCH_END__'$/,/^__PATCH_END__$/p" "$0" | sed '1d;$d'; }
emit_patch > "$PATCH"
[ -s "$PATCH" ] || die "embedded patch missing"
if [ ! -d "$SRC/.git" ]; then
  mkdir -p "$SRC"; git -C "$SRC" init -q
  git -C "$SRC" remote add origin "$REPO_URL"
fi
if [ "$(git -C "$SRC" rev-parse -q --verify HEAD 2>/dev/null || true)" != "$PIN_SHA" ]; then
  say "fetching exactly $PIN_SHA"
  git -C "$SRC" fetch --depth=1 origin "$PIN_SHA" >&2 || die "cannot fetch $PIN_SHA from $REPO_URL"
  git -C "$SRC" checkout -q --force "$PIN_SHA"
fi
# fresh tree, then apply the patch exactly once
git -C "$SRC" reset -q --hard "$PIN_SHA"
git -C "$SRC" apply --check "$PATCH" || die "patch does not apply to $PIN_SHA"
git -C "$SRC" apply "$PATCH"
PATCH_SHA="$(sha256_of "$PATCH")"
[ "$(git -C "$SRC" rev-parse HEAD)" = "$PIN_SHA" ] || die "checkout is not at $PIN_SHA"
[ "$(sha256_of "$SRC/$SPEC_REL")" = "$SPEC_SHA256" ] || die "spec file differs from the archived spec sha256"

# ---- build ----------------------------------------------------------------
export CARGO_TARGET_DIR="$WORK/target"
say "building ecbench (release); a first build takes several minutes"
( cd "$SRC" && cargo build --release --bin ecbench >&2 ) || die "cargo build failed"
ECB="$CARGO_TARGET_DIR/release/ecbench"
[ -x "$ECB" ] || die "no binary at $ECB"

# ---- select the runs ---------------------------------------------------------
SPEC="$SRC/$SPEC_REL"
"$ECB" plan --spec "$SPEC" > "$WORK/plan.txt" || die "ecbench plan failed"
NEXEC="$(head -1 "$WORK/plan.txt" | sed -n 's/.*: \([0-9][0-9]*\) executions.*/\1/p')"
[ -n "$NEXEC" ] || die "cannot read the execution count from 'ecbench plan'"
SEQS=""
for ((s=0; s<NEXEC; s++)); do
  "$ECB" profile-input --spec "$SPEC" --seq "$s" > "$WORK/input-$s.json" || die "profile-input $s failed"
  if grep -q 'engine=inherited-f4' "$WORK/input-$s.json"; then
    case "$SUBSET" in
      all) SEQS="$SEQS $s";;
      *) case ",$SUBSET," in *",$s,"*) SEQS="$SEQS $s";; esac;;
    esac
  fi
done
SEQS="${SEQS# }"
[ -n "$SEQS" ] || die "no inherited-F4 runs selected"
say "inherited-F4 runs: $(echo $SEQS | wc -w | tr -d ' ') (subset=$SUBSET): $SEQS"
[ "$LIST" = 1 ] && { echo "$SEQS"; exit 0; }

# ---- run: one fresh process per run, exactly the ecbench child environment --
RAW="$WORK/dump.jsonl"; : > "$RAW"
T0=$(date +%s)
for s in $SEQS; do
  nodes="$WORK/nodes-$s.jsonl"; rm -f "$nodes"
  env -i PATH="$PATH" HOME="${HOME:-/tmp}" LC_ALL=C RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 \
      RUST_BACKTRACE=0 RUST_MIN_STACK=8388608 \
      KIC_F4_NODE_DUMP="$nodes" KIC_F4_NODE_DUMP_COMPACT=1 \
      "$ECB" exec < "$WORK/input-$s.json" > "$WORK/out-$s.json" 2> "$WORK/err-$s.txt" \
      || die "exec seq $s failed (see $WORK/err-$s.txt)"
  [ -s "$nodes" ] || : > "$nodes"
  ops="$(tr -d ' \n' < "$WORK/out-$s.json" | sed -n 's/.*"solver_ops":\([0-9][0-9]*\).*/\1/p')"
  [ -n "$ops" ] || die "no relations.solver_ops in output of seq $s"
  oracle="$(tr -d ' \n' < "$WORK/input-$s.json" | sed -n 's/.*"oracle":"\([^"]*\)".*/\1/p')"
  tidx="$(tr -d ' \n' < "$WORK/input-$s.json" | sed -n 's/.*"target_index":\([0-9][0-9]*\).*/\1/p')"
  aseed="$(tr -d ' \n' < "$WORK/input-$s.json" | sed -n 's/.*"algorithm_seed":"\{0,1\}\([0-9][0-9]*\).*/\1/p')"
  nn="$(wc -l < "$nodes" | tr -d ' ')"
  printf '{"type":"run","seq":%s,"oracle":"%s","target_index":%s,"algorithm_seed":"%s","solver_ops":%s,"nodes":%s}\n' \
    "$s" "$oracle" "$tidx" "$aseed" "$ops" "$nn" >> "$RAW"
  # node lines: add type and seq, drop pid (not comparable across hosts)
  sed -e 's/"pid":[0-9]*,\{0,1\}//' -e "s/^{/{\"type\":\"node\",\"seq\":$s,/" "$nodes" >> "$RAW"
  say "seq $s done: solver_ops=$ops nodes=$nn"
done
WALL=$(( $(date +%s) - T0 ))

# ---- assemble the single output file ---------------------------------------
DUMP="$OUT/node-dump-$ARCH-$OS-$SHORT.jsonl.gz"
{
  printf '{"type":"meta","schema":"f4-node-dump/v1","arch":"%s","os":"%s","commit":"%s","patch_sha256":"%s","spec_sha256":"%s","subset":"%s","rustc":"%s","toolchain_note":"%s"}\n' \
    "$ARCH" "$OS" "$PIN_SHA" "$PATCH_SHA" "$SPEC_SHA256" "$SUBSET" "$RUSTC_V" "$TC_NOTE"
  cat "$RAW"
} | gzip -9 -n > "$DUMP"
LINES="$(gzip -dc "$DUMP" | wc -l | tr -d ' ')"
DSHA="$(sha256_of "$DUMP")"
CPU="$( { sysctl -n machdep.cpu.brand_string 2>/dev/null || lscpu 2>/dev/null | sed -n 's/^Model name:[ \t]*//p' | head -1; } || true )"
jstr() { printf '%s' "$1" | sed -e 's/\\/\\\\/g' -e 's/"/\\"/g' | tr '\n' ' '; }
HOSTJ="$OUT/host-$ARCH-$OS.json"
cat > "$HOSTJ" <<JSON
{
  "schema": "f4-node-dump-host/v1",
  "arch": "$ARCH",
  "os": "$OS",
  "uname": "$(jstr "$(uname -a)")",
  "cpu": "$(jstr "$CPU")",
  "rustc": "$(jstr "$RUSTC_V")",
  "cargo": "$(jstr "$CARGO_V")",
  "toolchain_note": "$(jstr "$TC_NOTE")",
  "commit": "$PIN_SHA",
  "patch_sha256": "$PATCH_SHA",
  "spec_sha256": "$SPEC_SHA256",
  "subset": "$SUBSET",
  "runs": $(echo $SEQS | wc -w | tr -d ' '),
  "dump_file": "$(basename "$DUMP")",
  "dump_sha256": "$DSHA",
  "dump_bytes": $(wc -c < "$DUMP" | tr -d ' '),
  "dump_lines": $LINES,
  "run_wall_seconds": $WALL
}
JSON
[ "$KEEP" = 1 ] || { rm -rf "$CARGO_TARGET_DIR"; say "removed build dir $CARGO_TARGET_DIR"; }
say "wrote $DUMP ($LINES lines, sha256 $DSHA) and $HOSTJ; run wall ${WALL}s"
say "copy both files into research/inherited_f4_replay_determinism_20261018/cross_host/ and commit"
exit 0
: <<'__PATCH_END__'
diff --git a/src/bin/ecbench/pdp3_koblitz.rs b/src/bin/ecbench/pdp3_koblitz.rs
index 53a9bbf6d..b75367de1 100644
--- a/src/bin/ecbench/pdp3_koblitz.rs
+++ b/src/bin/ecbench/pdp3_koblitz.rs
@@ -562,9 +562,11 @@ impl<'a> DecompositionOracle<BinaryGroup<'a>> for Pdp3KoblitzOracle<'_> {
         params: &Params,
         ops: &mut GroupOps,
     ) -> Result<(), String> {
-        if let Some((k, _)) =
-            std::env::vars().find(|(k, _)| TUNING_PREFIXES.iter().any(|p| k.starts_with(p)))
-        {
+        // `KIC_F4_NODE_DUMP*` only appends diagnostics to a file; it does not
+        // change the search (cross-host probe, research/inherited_f4_replay_determinism_20261018).
+        if let Some((k, _)) = std::env::vars().find(|(k, _)| {
+            !k.starts_with("KIC_F4_NODE_DUMP") && TUNING_PREFIXES.iter().any(|p| k.starts_with(p))
+        }) {
             return Err(format!(
                 "pdp3-koblitz refuses to run with the solver override `{k}` set: it would \
                  change the search without changing the method's identity"
diff --git a/src/cryptanalysis/koblitz_groebner.rs b/src/cryptanalysis/koblitz_groebner.rs
index 613266cba..4e584ab83 100644
--- a/src/cryptanalysis/koblitz_groebner.rs
+++ b/src/cryptanalysis/koblitz_groebner.rs
@@ -5195,6 +5195,40 @@ fn dump_node_system(system: &[F2BoolPoly], n_vars: usize, engine: SolverEngine)
     else {
         return;
     };
+    // Cross-host probe (research/inherited_f4_replay_determinism_20261018):
+    // `KIC_F4_NODE_DUMP_COMPACT=1` writes a node index, the word-op counters
+    // at node entry, and a hash of the system instead of the system, so two
+    // hosts' dumps can be diffed for the first node whose preceding work
+    // charged a different number of words.
+    if std::env::var("KIC_F4_NODE_DUMP_COMPACT").as_deref() == Ok("1") {
+        thread_local! {
+            static NODE_INDEX: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
+        }
+        let index = NODE_INDEX.with(|c| {
+            let i = c.get();
+            c.set(i + 1);
+            i
+        });
+        let text = serde_json::to_string(system).unwrap_or_default();
+        let mut h: u64 = 0xcbf29ce484222325;
+        for b in text.bytes() {
+            h = (h ^ u64::from(b)).wrapping_mul(0x100000001b3);
+        }
+        let terms: usize = system.iter().map(|p| p.terms.len()).sum();
+        let line = serde_json::json!({
+            "i": index,
+            "pid": std::process::id(),
+            "n_vars": n_vars,
+            "engine": format!("{engine:?}"),
+            "polys": system.len(),
+            "terms": terms,
+            "h": format!("{h:016x}"),
+            "w_thread": f4_word_ops_thread(),
+            "w_total": f4_word_ops_total(),
+        });
+        let _ = writeln!(file, "{line}");
+        return;
+    }
     let line = serde_json::json!({
         "n_vars": n_vars,
         "engine": format!("{engine:?}"),
__PATCH_END__
