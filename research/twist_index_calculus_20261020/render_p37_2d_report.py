"""Lay out the frozen p=37 full-2D oracle pilot and its A/A record."""
import hashlib
import json
import shutil
from pathlib import Path

import typst

ROOT = Path(__file__).resolve().parent
REPO = ROOT.parent.parent
RAW = ROOT / "quartic_twist_2d_p37_v1.json"
REPLAY = ROOT / "quartic_twist_2d_p37_native_audit_v1.json"
PRODUCER = REPO / "examples/quartic_twist_2d_p37.rs"
AUDITOR = REPO / "examples/quartic_twist_2d_p37_audit.rs"
AUDIT = json.loads(REPLAY.read_text())
assert AUDIT["status"] == "pass"
assert (ROOT / "quartic_twist_2d_p37_aa.diff").read_text() == ""


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


for suffix, aa in (("v1", False), ("aa_v1", True)):
    raw_name = f"quartic_twist_2d_p37_{suffix}.json"
    raw = json.loads((ROOT / raw_name).read_text())
    assert raw["status"] == "pass" and raw["rank"] == 53 and raw["recovered"] == 271
    run = "run-20261022T0717Z-quartic-2d-p37" + ("-aa" if aa else "")
    directory = ROOT / "reports" / run
    directory.mkdir(parents=True, exist_ok=True)
    phases = [
        ("model", raw["model_fp_muls"], 0),
        ("base including axis relifts", raw["base_fp_muls"], 0),
        ("input", raw["input_fp_muls"], raw["input_group_ops"]),
        ("eigenvalues", raw["eigen_fp_muls"], raw["eigen_group_ops"]),
        ("orbit map", raw["map_fp_muls"], raw["map_group_ops"]),
        ("full pair table", raw["table_fp_muls"], raw["table_group_ops"]),
        ("53-query stream", raw["stream_fp_muls"], raw["stream_group_ops"]),
        ("answer checks", raw["verification_fp_muls"], raw["verification_group_ops"]),
    ]
    rows_md = "\n".join(f"| {name} | {m:,} | {g:,} |" for name, m, g in phases)
    rows_typ = "\n".join(f"  [{name}], [{m:,}], [{g:,}]," for name, m, g in phases)
    command = (
        "ulimit -v 8388608; timeout 600s target/release/examples/quartic_twist_2d_p37 "
        "research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson"
    )
    audit_command = (
        "ulimit -v 8388608; timeout 600s target/release/examples/quartic_twist_2d_p37_audit "
        "research/twist_index_calculus_20261020/quartic_twist_2d_p37_v1.json"
    )
    markdown = f"""# p=37 full two-dimensional exact-oracle IC pilot{' · A/A' if aa else ''}

Run ID `{run}`; 22 October 2026; Intel Xeon 6975P-C, 32 logical CPUs, shared-host L0. One public known-answer target `[271]G`, seed zero, prime subgroup order 1,873,633. [Frozen protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md), [raw record](../../{raw_name}), [stderr log](../../{raw_name.replace('.json', '.log')}), [independent native audit](../../quartic_twist_2d_p37_native_audit_v1.json), [empty A/A non-timing diff](../../quartic_twist_2d_p37_aa.diff), and [graph](p37_relations.svg). Producer SHA-256 `{digest(PRODUCER)}`; auditor SHA-256 `{digest(AUDITOR)}`.

Command: `{command}`. Auditor command: `{audit_command}`. Completed before the 256-residual and 600-second caps; no failures, timeouts or censored queries. The exact signed group oracle used 624 physical points in `Fp+t²Fp`, folded to 52 factor-log columns. **53** residuals produced **{raw['decompositions']:,}** signed four-factor decompositions; 53 selected group-verified relations reached rank **53**, recovering scalar **271** and all 52 factor logs. The first three `(a,b)` values match the frozen single-line stream. The auditor independently enumerated all {AUDIT['point_logs_enumerated']:,} subgroup logs, recomputed every residual count with a fully signed point-keyed pair hash, checked each chosen relation and row, reconstructed rank and verified the scalar.

| Phase | Counted `Fp` multiplications | Counted group operations |
| --- | ---: | ---: |
{rows_md}

Modular matrix row operations: {raw['matrix_ops']:,}, a separate unit. L0 total wall time: {raw['wall_ms_l0']:.2f} ms, a shared-host diagnostic and not isolated speed evidence. The field and group counts include model, re-lifted axis and interior base, input, eigen/map checks, full pair table, all 53 queries, and answer checks. File parsing, hash probes and table allocation are excluded. This is an exact **group-oracle** pilot; polynomial S5 relation finding, matched rho and 0/0.1/1 GAE conversion are absent. It does not establish a complete cold polynomial IC or rho speedup. No external same-computation tool was compared.

![Exact signed decomposition count and rank growth](p37_relations.svg)

Graph source: [plot_quartic_followups.py](plot_quartic_followups.py). [PDF report](README.pdf).
"""
    (directory / "README.md").write_text(markdown)
    body = f"""#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= p=37 full two-dimensional exact-oracle IC{' · A/A' if aa else ''}
Run ID: `{run}`. One target `[271]G`, 624 physical points, 52 folded factor columns. Fifty-three residual queries yielded {raw['decompositions']:,} complete signed decompositions, rank 53, and verified scalar 271. An independent point-keyed signed-pair oracle replayed all queries and selected rows; the A/A non-timing diff is empty.

#table(columns: (1.5fr,1.4fr,1.4fr), inset: 4pt,
  [phase], [$F_p$ multiplications], [group operations],
{rows_typ}
)

Matrix: {raw['matrix_ops']:,} modular row operations. Total L0 time {raw['wall_ms_l0']:.2f} ms is shared-host diagnostic only. Hash and allocator cost, polynomial S5 relation finding, matched rho and GAE pricing remain open. Full commands, source hashes, raw trace and independent audit are linked in `README.md`.

#image("p37_relations.png", width: 100%)
"""
    (directory / "README.typ").write_text(body)
    shutil.copyfile(ROOT / "plot_quartic_followups.py", directory / "plot_quartic_followups.py")
    typst.compile(str(directory / "README.typ"), output=str(directory / "README.pdf"))
