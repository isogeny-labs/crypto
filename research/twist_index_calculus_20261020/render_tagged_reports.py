"""Lay out frozen action-tagged pair-index records; do not run experiments."""
import hashlib
import json
import shutil
from pathlib import Path

import typst

ROOT = Path(__file__).resolve().parent
REPO = ROOT.parent.parent
HOST = "Intel Xeon 6975P-C, 32 logical CPUs; shared-host L0"
RUNS = [
    ("ordered", 13, "20261022T0703Z", "20261022T0704Z"),
    ("ordered", 37, "20261022T0704Z", "20261022T0704Z"),
    ("hash", 13, "20261022T0705Z", "20261022T0705Z"),
    ("hash", 37, "20261022T0705Z", "20261022T0705Z"),
    ("direct", 13, "20261022T0707Z", "20261022T0707Z"),
    ("direct", 37, "20261022T0707Z", "20261022T0707Z"),
    ("direct_full", 37, "20261022T0720Z", "20261022T0723Z"),
]


for backend, prime, started, aa_started in RUNS:
    source = REPO / f"examples/quartic_twist_pair_tagged{'_' + backend if backend != 'ordered' else ''}.rs"
    source_hash = hashlib.sha256(source.read_bytes()).hexdigest()
    for suffix, aa in (("v1", False), ("aa_v1", True)):
        timestamp = aa_started if aa else started
        run = f"run-{timestamp}-quartic-tagged-{backend}-p{prime}" + ("-aa" if aa else "")
        raw_name = f"quartic_twist_pair_tagged{'_' + backend if backend != 'ordered' else ''}_p{prime}_{suffix}.json"
        raw = json.loads((ROOT / raw_name).read_text())
        directory = ROOT / "reports" / run
        directory.mkdir(parents=True, exist_ok=True)
        plot = directory / "tagged_pair_index.svg"
        if not plot.exists():
            raise RuntimeError(f"graph missing: {plot}")
        diff_name = f"quartic_twist_pair_tagged{'_' + backend if backend != 'ordered' else ''}_p{prime}_aa.diff"
        assert (ROOT / diff_name).read_text() == ""
        assert all(q["sets_equal"] for q in raw["queries"])
        phase = [
            ("full setup", raw["full_setup_fp_muls"], raw["full_setup_group_ops"]),
            ("tagged setup", raw["tagged_setup_fp_muls"], raw["tagged_setup_group_ops"]),
            ("full queries", raw["full_query_fp_muls"], raw["full_query_group_ops"]),
            ("tagged queries", raw["tagged_query_fp_muls"], raw["tagged_query_group_ops"]),
        ]
        rows = "\n".join(f"| {label} | {m:,} | {g:,} |" for label, m, g in phase)
        qrows = "\n".join(
            f"| {r['query_index']} | {r['a']} | {r['b']} | {r['decompositions']:,} |"
            for r in raw["queries"]
        )
        command = (
            f"ulimit -v 8388608; timeout 600s target/release/examples/{source.stem} "
            f"--p {prime} research/twist_index_calculus_20261020/quartic_twist_2d_p13_v1.json "
            "research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson"
            + (" research/twist_index_calculus_20261020/quartic_twist_2d_p37_v1.json"
               if backend == "direct_full" else "")
        )
        markdown = f"""# Action-tagged pair index · {backend} · p={prime}{' · A/A' if aa else ''}

Run ID `{run}`; 22 October 2026; {HOST}. [Protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md), [raw record](../../{raw_name}), [stderr](../../{raw_name.replace('.json', '.log')}), [empty A/A non-timing diff](../../{diff_name}), and [graph](tagged_pair_index.svg). Source SHA-256: `{source_hash}`.

Command: `{command}`. One public target `[271]G`; {len(raw['queries'])} frozen residual queries. Completed {len(raw['queries'])}/{len(raw['queries'])}; no timeout, failure or censored query. The full and tagged tables produced exactly equal **complete signed four-factor decomposition sets** for every residual; an independent group addition checked all {raw['tagged_postings']:,} reconstructed postings. The table contains {raw['tagged_pair_representatives']:,} orbit representatives, {raw['tagged_postings']:,} postings and {raw['tagged_keys']:,} abscissa keys, against {raw['full_pair_entries']:,} full signed pair entries. Pair-orbit histogram: `{raw['pair_orbit_hist']}`.

| Phase | Counted `Fp` multiplications | Counted group operations |
| --- | ---: | ---: |
{rows}

| Table | Retained allocator-requested bytes | Peak allocator-requested bytes |
| --- | ---: | ---: |
| full | {raw['full_table_live_requested_bytes']:,} | {raw['full_table_peak_requested_bytes']:,} |
| tagged | {raw['tagged_table_live_requested_bytes']:,} | {raw['tagged_table_peak_requested_bytes']:,} |

Tagged lookup calls: {raw['tagged_lookup_calls']:,}; candidate postings inspected: {raw['tagged_candidate_postings']:,}. Setup/queries L0 wall ms, full: {raw['full_setup_ms_l0']:.3f}/{raw['full_query_ms_l0']:.3f}; tagged: {raw['tagged_setup_ms_l0']:.3f}/{raw['tagged_query_ms_l0']:.3f}. These shared-host observations are not isolated performance results. Posting audit cost is excluded from the candidate's charged work: {raw['posting_audit_fp_muls_excluded']:,} `Fp M` and {raw['posting_audit_group_ops_excluded']:,} group operations. Model/base/target construction is common and excluded; full cold IC and matched rho were not run in this component comparison. Requested bytes are allocator payload sizes, not RSS.

| Query | a | b | Exact signed decompositions |
| ---: | ---: | ---: | ---: |
{qrows}

![Recorded operation and requested-byte comparison](tagged_pair_index.svg)

Graph source: [plot_quartic_followups.py](plot_quartic_followups.py). [PDF report](README.pdf). External same-computation pair-index tool: not available or compared; no fastest-tool claim.
"""
        (directory / "README.md").write_text(markdown)
        typ_rows = "\n".join(
            f"  [{label}], [{m:,}], [{g:,}]," for label, m, g in phase
        )
        body = f"""#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= Action-tagged pair index · {backend} · p={prime}{' · A/A' if aa else ''}
Run ID: `{run}`. {HOST}. One known-answer target and {len(raw['queries'])} registered residual queries. Full and tagged tables returned identical complete signed decomposition sets on every query; {raw['tagged_postings']:,} postings passed an independent group-addition audit. The non-timing A/A diff is empty.

#table(columns: (1.3fr,1.4fr,1.3fr), inset: 5pt,
  [phase], [$F_p$ multiplications], [group operations],
{typ_rows}
)

Retained requested bytes: full {raw['full_table_live_requested_bytes']:,}; tagged {raw['tagged_table_live_requested_bytes']:,}. Setup and query counts exclude common input preparation and the audit. L0 wall times are shared-host diagnostics, not isolated speed evidence. Whole IC, polynomial relation finding, rho and external tools are not measured here. Source, command and raw data are linked in `README.md`.

#image("tagged_pair_index.png", width: 100%)
"""
        (directory / "README.typ").write_text(body)
        shutil.copyfile(ROOT / "plot_quartic_followups.py", directory / "plot_quartic_followups.py")
        typst.compile(str(directory / "README.typ"), output=str(directory / "README.pdf"))
