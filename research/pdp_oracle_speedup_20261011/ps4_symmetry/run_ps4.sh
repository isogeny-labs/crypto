#!/usr/bin/env bash
# PS-4: candidate reduction by Frobenius-orbit representatives and the 2-torsion symmetry
# (protocols/PS-4.md).  Every build and every experiment goes through the benchmark lock
# (`tools/isolated_bench.py busy --wait`); one compute command at a time.  All counts are
# deterministic; every wall figure in the records is `ms_unisolated` (shared container, no
# pinning) and is a note, not a result.
#
# STAGES (space separated, default: all):
#   build checks panel run torsion control summary figures pdf
# A stage skips what its marker file in raw/done/ says is finished, so an interrupted run is
# resumed without overwriting any record; a re-run of a finished stage needs its marker removed.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
S=research/pdp_oracle_speedup_20261011
D=$S/ps4_symmetry
OUT=$D/raw
C=$S/baseline_v1/corpus
BIN=target/release/examples/pdp_ps4
OPC=target/opcount/release/examples/pdp_ps4
PANEL=target/release/examples/hamming_nb_sat_panel
STAGES=${STAGES:-"build checks panel run torsion control summary figures pdf"}
SEED=20261011
BUSY="python3 tools/isolated_bench.py busy --wait --"
mkdir -p "$OUT/done" "$OUT/panel_work"

# The seven type-II degrees of the protocol and the two weights.
DEGREES="11 23 29 33 35 39 41"
# The panel's own exact oracle (pair table on the library's inversion) runs in bounded time for
# every w = 2 setting and for w = 3 at n = 11, 23 and 29; at the larger w = 3 bases it needs
# hours per target (see README), and the pair-table oracle of the example stands alone there.
PANEL_SETTINGS="11:2 11:3 23:2 23:3 29:2 29:3 33:2 33:3 35:2 35:3 39:2 41:2"
# Torsion test targets: all eight, except at the three largest w = 3 bases (time): targets 0
# (uniform) and 6 (planted).
torsion_only() { if [ "$2" = 3 ] && [ "$1" -ge 35 ]; then echo "--only 0,6"; fi; }

# A resumed run on another host class records that host (the build stage records the first one,
# in manifest.txt).  Counts do not depend on the host; the wall notes (`ms_unisolated`) are never
# pooled across hosts (the summary attributes each record to a host by its stage marker's time).
HOSTTAG=$(printf '%s-%s' "$(uname -r)" "$(grep -m1 'model name' /proc/cpuinfo | sed 's/.*: //')" | tr -c 'A-Za-z0-9.\n' '_')
if [ -f "$OUT/manifest.txt" ] && ! grep -q "$(grep -m1 'model name' /proc/cpuinfo | sed 's/.*: //' | sed 's/[][()*.]/\\&/g')" "$OUT/manifest.txt" \
   && [ ! -f "$OUT/host_$HOSTTAG.txt" ]; then
  {
    echo "date_utc $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "note resumed run on a different host class; the binaries are the ones hashed in manifest.txt"
    echo "rustc $(rustc --version)"
    echo "cargo $(cargo --version)"
    echo "git_head $(git rev-parse HEAD) branch $(git branch --show-current)"
    echo "uname $(uname -srm)"
    echo "cpu $(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2- | sed 's/^ //')"
    echo "cpu_flags $(grep -m1 '^flags' /proc/cpuinfo | tr ' ' '\n' | grep -E '^(popcnt|pclmulqdq|avx2|avx512f|avx512bw|bmi2)$' | tr '\n' ' ')"
    echo "logical_cpus $(nproc)"
    echo "mem_kb $(grep MemTotal /proc/meminfo | awk '{print $2}')"
    sha256sum "$BIN" "$OPC" "$PANEL"
  } > "$OUT/host_$HOSTTAG.txt"
fi

stage() { case " $STAGES " in *" $1 "*) return 0 ;; *) return 1 ;; esac; }
done_marker() { [ -f "$OUT/done/$1" ]; }
mark() { date -u +%Y%m%dT%H%M%SZ > "$OUT/done/$1"; }

if stage build; then
  if ! done_marker build; then
    $BUSY cargo build --release --example pdp_ps4
    $BUSY cargo build --release --example hamming_nb_sat_panel
    CARGO_TARGET_DIR=target/opcount $BUSY cargo build --release --example pdp_ps4 --features pdp-opcount
    {
      echo "date_utc $(date -u +%Y-%m-%dT%H:%M:%SZ)"
      echo "rustc $(rustc --version)"
      echo "cargo $(cargo --version)"
      echo "git_head $(git rev-parse HEAD)"
      echo "uname $(uname -srm)"
      echo "cpu $(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2- | sed 's/^ //')"
      echo "cpu_flags $(grep -m1 '^flags' /proc/cpuinfo | tr ' ' '\n' | grep -E '^(popcnt|pclmulqdq|avx2|avx512f|avx512bw|bmi2)$' | tr '\n' ' ')"
      echo "logical_cpus $(nproc)"
      echo "mem_kb $(grep MemTotal /proc/meminfo | awk '{print $2}')"
      sha256sum "$BIN" "$OPC" "$PANEL" examples/pdp_ps4/*.rs examples/hamming_nb_sat_panel.rs \
        "$C"/corpus_l8.txt "$C"/corpus_l9.txt "$C"/corpus_l10.txt "$C"/corpus_l11.txt "$C"/corpus_l12.txt
    } > "$OUT/manifest.txt"
    mark build
  fi
fi

if stage checks; then
  # Field vs the library, curve constants (a = 0), point counts, translation by T, S3, and the
  # exactness theorem (exhaustive at n = 3, 5, 9; weight bases at n = 11).
  for c in field curve theorem; do
    if ! done_marker "check_$c"; then
      $BUSY "$BIN" "check-$c" --out "$OUT/${c}_checks.jsonl"
      mark "check_$c"
    fi
  done
fi

if stage panel; then
  # The panel itself, on the targets and the oracle we compare with: --m 3, 6 uniform + 2 planted,
  # seed 20261011, the SAT arm cut to one second so that only the exact reference matters.
  for nw in $PANEL_SETTINGS; do
    n=${nw%%:*}; w=${nw##*:}
    if ! done_marker "panel_n${n}_w${w}"; then
      $BUSY "$PANEL" --n "$n" --w "$w" --m 3 --targets 6 --planted 2 --seed $SEED \
        --solvers cadical --timeout-s 1 --max-models 1 --work "$OUT/panel_work" \
        --out "$OUT/panel_n${n}_w${w}.jsonl"
      mark "panel_n${n}_w${w}"
    fi
  done
fi

if stage run; then
  for w in 2 3; do
    for n in $DEGREES; do
      if ! done_marker "run_n${n}_w${w}"; then
        extra=()
        [ -f "$OUT/panel_n${n}_w${w}.jsonl" ] && extra=(--panel "$OUT/panel_n${n}_w${w}.jsonl")
        $BUSY "$BIN" run --n "$n" --w "$w" --seed $SEED "${extra[@]}" --out "$OUT/runs.jsonl"
        mark "run_n${n}_w${w}"
      fi
    done
  done
fi

if stage torsion; then
  for w in 2 3; do
    for n in $DEGREES; do
      if ! done_marker "torsion_n${n}_w${w}"; then
        # shellcheck disable=SC2046
        $BUSY "$BIN" torsion --n "$n" --w "$w" --seed $SEED $(torsion_only "$n" "$w") --out "$OUT/torsion.jsonl"
        mark "torsion_n${n}_w${w}"
      fi
    done
  done
fi

if stage control; then
  # The subspace-leaf control: the PS-2 group-law kernel on the PS-0 corpus, library counters.
  for l in 8 9 10 11 12; do
    if ! done_marker "control_l${l}"; then
      $BUSY "$OPC" control --l "$l" --corpus "$C/corpus_l$l.txt" --targets 6 --planted 2 --out "$OUT/control.jsonl"
      mark "control_l${l}"
    fi
  done
fi

if stage summary; then
  # The summary subcommand was extended after the measurements (host attribution of the wall notes,
  # totals, the plotting data); manifest_summary.txt hashes the sources it was built from.  The
  # measurement subcommands (check-*, run, torsion, control) are byte-identical to the build stage's.
  SUMBIN=target/ps4_summary/release/examples/pdp_ps4
  CARGO_TARGET_DIR=target/ps4_summary $BUSY cargo build --release --example pdp_ps4
  sha256sum "$SUMBIN" examples/pdp_ps4/*.rs > "$OUT/manifest_summary.txt"
  "$SUMBIN" summary --raw "$OUT" --data "$OUT/plot_data.json" > "$OUT/summary.md"
  echo "wrote $OUT/summary.md and $OUT/plot_data.json"
fi

# The run report (research-visuals): figures from the recorded values, then the Typst PDF.
REPORT=$S/reports/run-20261011T2346Z
if stage figures; then
  mkdir -p "$REPORT"
  /tmp/report-venv/bin/python "$D/plot_ps4.py" "$OUT/plot_data.json" "$REPORT"
fi
if stage pdf; then
  /tmp/report-venv/bin/python -c "import typst; typst.compile('$REPORT/ps4_report.typ', output='$REPORT/ps4_report.pdf')"
fi
