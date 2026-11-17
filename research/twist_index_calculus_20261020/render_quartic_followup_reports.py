"""Lay out frozen raw records and existing plots as write-once run reports."""
import hashlib
import json
import shutil
from pathlib import Path

import typst


ROOT = Path(__file__).resolve().parent
REPO = ROOT.parent.parent
HOST = "Intel Xeon 6975P-C, 32 logical CPUs; shared-host L0"
PLOT_SOURCE = ROOT / "plot_quartic_followups.py"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(run, markdown, body, graph):
    directory = ROOT / "reports" / run
    directory.mkdir(parents=True, exist_ok=True)
    (directory / "README.md").write_text(markdown)
    (directory / "README.typ").write_text(
        '#set page(paper: "a4", margin: 18mm)\n'
        '#set text(font: "Liberation Sans", size: 9pt)\n'
        + body
        + f'\n#image("{graph}.png", width: 100%)\n'
    )
    shutil.copyfile(PLOT_SOURCE, directory / "plot_quartic_followups.py")
    typst.compile(str(directory / "README.typ"), output=str(directory / "README.pdf"))


for prime, run in [
    (37, "run-20261022T0616Z-quartic-s5-p37"),
    (73, "run-20261022T0618Z-quartic-s5-p73"),
    (97, "run-20261022T0620Z-quartic-s5-p97"),
]:
    raw_name = f"quartic_twist_s5_p{prime}.json"
    raw = json.loads((ROOT / raw_name).read_text())
    rows = raw["rows"]
    source = REPO / "examples/quartic_twist_s5_pilot.rs"
    command = (
        f"RAYON_NUM_THREADS=16 TMPDIR=/home/ec2-user/.cache/gls-report-tmp "
        f"timeout 1800s target/release/examples/quartic_twist_s5_pilot --p {prime} "
        "research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson"
    )
    table = "\n".join(
        f"| {row['kind']} | {row['query_index']} | {row['recorded_decompositions']} | "
        f"{row['solve']['total_muls']:,} | {row['solve']['total_ms']:,.1f} | "
        f"{'yes' if row['agree'] else 'NO'} |"
        for row in rows
    )
    markdown = f"""# Native S5 pilot, p={prime}

Run ID `{run}`; 22 October 2026; {HOST}. One public known-answer target `[271]G`. Registered [protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md), [raw record](../../{raw_name}), [stderr log](../../quartic_twist_s5_p{prime}.log), and [graph](s5_cost.svg). Source SHA-256: `{digest(source)}`.

Command: `{command}`. The process watchdog was 1,800 seconds; both registered queries completed. The exact signed meet-in-the-middle oracle and group addition checked all returned quadruples. This is a two-query oracle pilot, not a full IC solve.

| Query | Index in frozen L0 stream | Exact decompositions | Native S5 solve `Fp M` | L0 solve ms | Oracle agreement |
| --- | ---: | ---: | ---: | ---: | :---: |
{table}

Symmetrisation precomputation: {raw['precompute_muls']:,} `Fp M`, {raw['precompute_ms_l0']:.1f} L0 ms. Each solve used a degree-{rows[0]['solve']['macaulay_degree']} Macaulay matrix with {rows[0]['solve']['rows']:,} rows and {rows[0]['solve']['cols']:,} columns. L0 timing is a shared-host observation and is not an isolated performance comparison. The S5 system is the single `x∈Fp` line; a tagged two-line or two-dimensional polynomial system was not run. msolve/Magma were unavailable in this environment and were not compared.

![Counted field multiplications for both registered queries](s5_cost.svg)

Graph source: [plot_quartic_followups.py](plot_quartic_followups.py). [PDF report](README.pdf). No failed, extraneous or censored outcome occurred.
"""
    typ_table = "\n".join(
        f"  [{row['kind']}], [{row['query_index']}], [{row['recorded_decompositions']}], "
        f"[{row['solve']['total_muls']:,}], [{row['agree']}],"
        for row in rows
    )
    body = f"""= Native S5 pilot · p={prime}
Run ID: `{run}`. {HOST}. The frozen seed-zero L0 trace supplies one hit and one miss on `[271]G`. Both solver outputs match the exact signed group oracle.

#table(columns: (1.2fr,.6fr,.8fr,1.6fr,.7fr), inset: 5pt,
  [query], [index], [decompositions], [solve $F_p$ multiplications], [agree],
{typ_table}
)

Symmetrisation: {raw['precompute_muls']:,} $F_p$ multiplications. Each solve: degree {rows[0]['solve']['macaulay_degree']}, {rows[0]['solve']['rows']:,} Macaulay rows, {rows[0]['solve']['cols']:,} columns. Timings are shared-host L0; no whole-method speed claim. Source, command, raw data and audit details are in `README.md`.
"""
    write(run, markdown, body, "s5_cost")


run = "run-20261022T0621Z-quartic-2d-census"
records = [json.loads(line) for line in (ROOT / "quartic_twist_2d_census_v1.ndjson").read_text().splitlines()]
census_source = REPO / "examples/quartic_twist_2d_census.rs"
audit_source = REPO / "examples/quartic_twist_2d_audit.rs"
rows_md = "\n".join(
    f"| {r['p']} | {r['line0']} | {r['line2']} | {r['interior']:,} | "
    f"{r['physical_base']:,} | {r['folded_columns']:,} | {r['signed_pair_entries']:,} |"
    for r in records
)
markdown = f"""# Two-dimensional invariant-space census

Run ID `{run}`; 22 October 2026; {HOST}. [Frozen protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md), [raw census](../../quartic_twist_2d_census_v1.ndjson), [independent native norm/point audit](../../quartic_twist_2d_native_audit_v1.ndjson), and [graph](space_census.svg). The producer and audit returned `pass` on all eight curves.

Commands: `target/release/examples/quartic_twist_2d_census` and `target/release/examples/quartic_twist_2d_audit research/twist_index_calculus_20261020/quartic_twist_2d_census_v1.ndjson`. Source SHA-256: producer `{digest(census_source)}`; audit `{digest(audit_source)}`. The test enumerated every nonzero `a+b t²` in `Fp+t²Fp`; this is a structural census, not a target relation stream.

| p | line 0 | line 2 | interior | full physical base | 24-fold columns | signed pair entries |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
{rows_md}

The independent audit applied the `Fp²` norm character and checked signed point orbits on every abscissa orbit. All 265,008 candidate abscissae agreed and every signed orbit had size 24. The pair-entry count is `B(B−1)` for the current explicit oracle before collisions; it is a derived work size, not allocated memory. No target or rho was run in this census.

![Invariant-space base size and pair-entry count](space_census.svg)

Graph source: [plot_quartic_followups.py](plot_quartic_followups.py). [PDF report](README.pdf).
"""
typ_rows = "\n".join(
    f"  [{r['p']}], [{r['line0']+r['line2']}], [{r['interior']:,}], "
    f"[{r['physical_base']:,}], [{r['signed_pair_entries']:,}],"
    for r in records
)
body = f"""= Two-dimensional invariant-space census
Run ID: `{run}`. {HOST}. Every nonzero `a+b t²` was tested on eight canonical quartic twists. The independent norm and point-orbit audit passed on all 265,008 candidates.

#table(columns: (.5fr,.8fr,.9fr,.9fr,1.2fr), inset: 4pt,
  [$p$], [two axes], [interior], [full base], [signed pair entries],
{typ_rows}
)

Every nonempty abscissa orbit has size 12 and signed point orbit size 24. The p=13 empty-line control has 72 interior abscissae. Pair entries are a derived construction count, not measured memory. Source, commands and raw files are linked in `README.md`.
"""
write(run, markdown, body, "space_census")


for suffix, run in [
    ("v1", "run-20261022T0626Z-quartic-2d-p13"),
    ("aa_v1", "run-20261022T0626Z-quartic-2d-p13-aa"),
]:
    raw_name = f"quartic_twist_2d_p13_{suffix}.json"
    raw = json.loads((ROOT / raw_name).read_text())
    source = REPO / "examples/quartic_twist_2d_p13.rs"
    audit = json.loads((ROOT / "quartic_twist_2d_p13_native_audit_v1.json").read_text())
    phase_rows = [
        ("model", raw["model_fp_muls"], 0),
        ("base", raw["base_fp_muls"], 0),
        ("input", raw["input_fp_muls"], raw["input_group_ops"]),
        ("eigenvalues", raw["eigen_fp_muls"], raw["eigen_group_ops"]),
        ("orbit map", raw["map_fp_muls"], raw["map_group_ops"]),
        ("pair table", raw["table_fp_muls"], raw["table_group_ops"]),
        ("relation stream", raw["stream_fp_muls"], raw["stream_group_ops"]),
        ("answer check", raw["verification_fp_muls"], raw["verification_group_ops"]),
    ]
    phase_md = "\n".join(f"| {name} | {m:,} | {g:,} |" for name,m,g in phase_rows)
    q_md = "\n".join(
        f"| {i+1} | {item[2]} | {raw['selected_relations'][i]['rank']} |"
        for i,item in enumerate(raw["query_trace"])
    )
    aa_note = "Full A/A non-timing diff: [empty](../../quartic_twist_2d_p13_aa.diff)."
    markdown = f"""# p=13 two-dimensional exact-oracle IC pilot{' (A/A)' if suffix != 'v1' else ''}

Run ID `{run}`; 22 October 2026; {HOST}. One public target `[271]G`, seed zero, exact signed four-factor oracle, 72 physical base points and six 24-fold log columns. [Frozen protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md), [raw record](../../{raw_name}), [independent replay](../../quartic_twist_2d_p13_native_audit_v1.json), [graph](p13_relations.svg). Source SHA-256: `{digest(source)}`.

Command: `TMPDIR=/home/ec2-user/.cache/gls-report-tmp timeout 180s target/release/examples/quartic_twist_2d_p13`. The 10,000-target cap was not reached. Seven targets produced 4,041 signed decompositions and seven selected group-verified relations; rank seven recovered and independently verified the target log 271. The separate auditor enumerated all {audit['point_logs_enumerated']:,} group logs, recomputed all seven signed query hit counts and rows, and verified rank and answer. {aa_note}

| Phase | Counted `Fp` multiplications | Counted group operations |
| --- | ---: | ---: |
{phase_md}

Matrix arithmetic: {raw['matrix_ops']:,} modular row operations, separately priced. Total shared-host L0 process time: {raw['wall_ms_l0']:.2f} ms; this is not an isolated speed result. Hash probes, table allocation, a matched rho band and an S5 polynomial system are not measured in this pilot.

| Processed target | Exact decompositions | Matrix rank |
| ---: | ---: | ---: |
{q_md}

![Decomposition count and rank for each target](p13_relations.svg)

Graph source: [plot_quartic_followups.py](plot_quartic_followups.py). [PDF report](README.pdf).
"""
    typ_phase = "\n".join(
        f"  [{name}], [{m:,}], [{g:,}]," for name,m,g in phase_rows
    )
    body = f"""= p=13 two-dimensional IC pilot{' · A/A' if suffix != 'v1' else ''}
Run ID: `{run}`. {HOST}. One target `[271]G`; 72 base points, six 24-fold columns. Seven queries yielded {raw['decompositions']:,} exact signed decompositions, seven selected relations, rank seven and verified scalar 271. Independent replay checked all queries, rows and factor logs. A/A non-timing diff is empty.

#table(columns: (1fr,1.2fr,1.1fr), inset: 4pt,
  [phase], [$F_p$ multiplications], [group operations],
{typ_phase}
)

Matrix: {raw['matrix_ops']} modular row operations. Wall: {raw['wall_ms_l0']:.2f} ms, shared-host L0. Hash work, calibrated rho and the polynomial relation system remain unpriced. Raw data, commands and source hashes are in `README.md`.
"""
    write(run, markdown, body, "p13_relations")


for suffix, run in [
    ("v1", "run-20261022T0644Z-quartic-pair-quotient"),
    ("aa_v1", "run-20261022T0645Z-quartic-pair-quotient-aa"),
]:
    raw_name = f"quartic_twist_pair_quotient_p13_{suffix}.json"
    raw = json.loads((ROOT / raw_name).read_text())
    source = REPO / "examples/quartic_twist_pair_quotient_p13.rs"
    phase_rows = [
        ("full setup", raw["full_setup_fp_muls"], raw["full_setup_group_ops"]),
        ("quotient setup", raw["quotient_setup_fp_muls"], raw["quotient_setup_group_ops"]),
        ("full seven queries", raw["full_query_fp_muls"], raw["full_query_group_ops"]),
        ("quotient seven queries", raw["quotient_query_fp_muls"], raw["quotient_query_group_ops"]),
    ]
    phase_md = "\n".join(f"| {name} | {m:,} | {g:,} |" for name,m,g in phase_rows)
    typ_rows = "\n".join(f"  [{name}], [{m:,}], [{g:,}]," for name,m,g in phase_rows)
    markdown = f"""# p=13 twist-quotient pair-table oracle{' (A/A)' if suffix != 'v1' else ''}

Run ID `{run}`; 22 October 2026; {HOST}. [Frozen protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md), [raw record](../../{raw_name}), [previous exact-oracle target trace](../../quartic_twist_2d_p13_v1.json), and [graph](pair_quotient.svg). Source SHA-256: `{digest(source)}`.

Command: `TMPDIR=/home/ec2-user/.cache/gls-report-tmp timeout 180s target/release/examples/quartic_twist_pair_quotient_p13 research/twist_index_calculus_20261020/quartic_twist_2d_p13_v1.json`. All seven queries completed. The full and quotient oracles returned **identical complete signed decomposition sets** on every query; each previously selected IC relation occurs in both. The full table had {raw['full_pair_entries']:,} entries, while the quotient stored {raw['quotient_representatives']:,} orbit representatives under the 12 abscissa actions and used {raw['quotient_hash_keys']:,} canonical hash keys. Orbit histogram: `{raw['pair_orbit_hist']}`. The [A/A non-timing diff](../../quartic_twist_pair_quotient_p13_aa.diff) is empty.

| Phase | Counted `Fp` multiplications | Counted group operations |
| --- | ---: | ---: |
{phase_md}

Setup and query counts exclude common model, base and target construction. Hash probes, allocator bytes and resident memory were not measured. Shared-host L0 times (full setup/query; quotient setup/query) were {raw['full_setup_ms_l0']:.2f}/{raw['full_query_ms_l0']:.2f} and {raw['quotient_setup_ms_l0']:.2f}/{raw['quotient_query_ms_l0']:.2f} ms; these are diagnostics, not an isolated speed comparison. The quotient exchanges fewer stored pair representatives for more field arithmetic in this implementation.

![Full and quotient pair-table work on identical targets](pair_quotient.svg)

Graph source: [plot_quartic_followups.py](plot_quartic_followups.py). [PDF report](README.pdf).
"""
    body = f"""= p=13 twist-quotient pair-table oracle{' · A/A' if suffix != 'v1' else ''}
Run ID: `{run}`. {HOST}. On the same seven recorded residuals, the full and quotient tables returned identical complete signed decomposition sets. The full table has {raw['full_pair_entries']:,} entries; the quotient stores {raw['quotient_representatives']:,} orbit representatives and {raw['quotient_hash_keys']:,} canonical keys. Every pair-entry orbit has size 12. A/A non-timing diff is empty.

#table(columns: (1.3fr,1.2fr,1.2fr), inset: 5pt,
  [phase], [$F_p$ multiplications], [group operations],
{typ_rows}
)

This is a storage/work-size and operation-count result. Hash probes, allocated bytes and calibrated whole-method costs were not measured. Raw data, command and source hash are in `README.md`.
"""
    write(run, markdown, body, "pair_quotient")
