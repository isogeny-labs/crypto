"""Lay out recorded dense same-residual exact-oracle results and controls."""
import hashlib
import json
import shutil
from pathlib import Path

import typst

ROOT = Path(__file__).resolve().parent
REPO = ROOT.parent.parent
PHASES = ["model", "base", "input", "eigen", "map", "table", "stream", "verification"]


def total(record, unit):
    return sum(record.get(f"{phase}_{unit}", 0) for phase in PHASES)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


for prime in (13, 37):
    prior_name = f"quartic_twist_2d_p{prime}_v1.json"
    prior = json.loads((ROOT / prior_name).read_text())
    audit_name = f"quartic_twist_2d_dense_p{prime}_native_audit_v1.json"
    audit = json.loads((ROOT / audit_name).read_text())
    assert audit["status"] == "pass"
    diff_name = f"quartic_twist_2d_dense_p{prime}_aa.diff"
    assert (ROOT / diff_name).read_text() == ""
    source = REPO / f"examples/quartic_twist_2d_dense_p{prime}.rs"
    auditor = REPO / "examples/quartic_twist_2d_dense_audit.rs"
    command = (
        f"ulimit -v 8388608; timeout 600s target/release/examples/quartic_twist_2d_dense_p{prime}"
        + (" research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson" if prime == 37 else "")
    )
    for suffix, aa in (("v1", False), ("aa_v1", True)):
        raw_name = f"quartic_twist_2d_dense_p{prime}_{suffix}.json"
        raw = json.loads((ROOT / raw_name).read_text())
        assert raw["status"] == "pass" and raw["residuals"] == 1 and raw["recovered"] == 271
        stamp = "20261022T0730Z" if aa else "20261022T0729Z"
        run = f"run-{stamp}-quartic-2d-dense-p{prime}" + ("-aa" if aa else "")
        directory = ROOT / "reports" / run
        directory.mkdir(parents=True, exist_ok=True)
        phase_rows = [
            ("model", raw["model_fp_muls"], 0),
            ("base", raw["base_fp_muls"], 0),
            ("input", raw["input_fp_muls"], raw["input_group_ops"]),
            ("eigenvalues", raw["eigen_fp_muls"], raw["eigen_group_ops"]),
            ("orbit map", raw["map_fp_muls"], raw["map_group_ops"]),
            ("full pair table", raw["table_fp_muls"], raw["table_group_ops"]),
            ("one-query stream", raw["stream_fp_muls"], raw["stream_group_ops"]),
            ("answer checks", raw["verification_fp_muls"], raw["verification_group_ops"]),
        ]
        rows_md = "\n".join(f"| {name} | {m:,} | {g:,} |" for name, m, g in phase_rows)
        rows_typ = "\n".join(f"  [{name}], [{m:,}], [{g:,}]," for name, m, g in phase_rows)
        old_m, new_m = total(prior, "fp_muls"), total(raw, "fp_muls")
        old_g, new_g = total(prior, "group_ops"), total(raw, "group_ops")
        markdown = f"""# Dense same-residual 2D exact-oracle IC · p={prime}{' · A/A' if aa else ''}

Run ID `{run}`; 22 October 2026; Intel Xeon 6975P-C, 32 logical CPUs, shared-host L0. One public known-answer target `[271]G`. [Frozen protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md), [raw record](../../{raw_name}), [stderr log](../../{raw_name.replace('.json', '.log')}), [independent replay](../../{audit_name}), [empty A/A non-timing diff](../../{diff_name}), [prior one-row-per-query pilot](../../{prior_name}) and [graph](dense_cost.svg). Producer SHA-256 `{digest(source)}`; auditor SHA-256 `{digest(auditor)}`.

Command: `{command}`. Completed 1/1 residual before the 64-query and 600-second caps; no failure, timeout or censored query. The exact signed oracle enumerated **{raw['decompositions']:,}** four-distinct-factor decompositions of the *same* residual. Its unanchored matrix rank was **{raw['unanchored_rank_first_query']}** of {raw['folded_columns']+1}; setting the public generator's factor log to 1 left **{raw['anchored_unknowns']}** unknowns. A deterministic scan accepted **{len(raw['selected_relations'])}** rank-raising rows within the first **{raw['decompositions_examined_to_rank']}** decompositions and reached anchored rank **{raw['rank']}**, recovering 271. The independent point-keyed signed-pair oracle reproduced the full first-query hit count, checked every selected signed pair and row, enumerated all **{audit['point_logs_enumerated']:,}** subgroup logs, independently solved the anchored matrix and verified 271. The one-residual unanchored rank ceiling follows because differences between rows annihilate the nonzero factor-log vector; the anchored full rank supplies the matching lower bound.

| Phase | Counted `Fp` multiplications | Counted group operations |
| --- | ---: | ---: |
{rows_md}

Total counted `Fp M`: previous {old_m:,}; dense anchored {new_m:,}; previous/dense **{old_m/new_m:.3f}**. Total group operations: previous {old_g:,}; dense anchored {new_g:,}; previous/dense **{old_g/new_g:.3f}**. These totals include model, base, input, eigen/map verification, full pair table, relation stream and answer checks in each arm. The reference used one selected row per residual and kept the generator log as an unknown; the candidate changes **both** row retention and the public known-log anchor. Modular row operations are separate: anchored selection {raw['anchored_matrix_ops']:,}, final solve {raw['solve_matrix_ops']:,}, and unanchored-rank diagnostic {raw['unanchored_matrix_ops']:,} (excluded from the anchored algorithmic path but included in this run's wall time). Shared-host L0 total wall time {raw['wall_ms_l0']:.3f} ms is not an isolated speed claim. Hash/allocation CPU, polynomial S5, 0/0.1/1 GAE and an integrated matched rho are absent; this is a counted **exact group-oracle** one-target comparison.

![Old one-row-per-query and dense anchored operation counts](dense_cost.svg)

Graph source: [plot_quartic_followups.py](plot_quartic_followups.py). [PDF report](README.pdf).
"""
        (directory / "README.md").write_text(markdown)
        body = f"""#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= Dense same-residual 2D oracle IC · p={prime}{' · A/A' if aa else ''}
Run ID: `{run}`. One target `[271]G`. {raw['decompositions']:,} exact signed decompositions of the first residual; unanchored rank {raw['unanchored_rank_first_query']} of {raw['folded_columns']+1}. The public generator log fixes one column, leaving {raw['anchored_unknowns']} unknowns. {len(raw['selected_relations'])} accepted rows from the first {raw['decompositions_examined_to_rank']} decompositions gave full anchored rank and scalar 271. Independent signed-pair and all-log replay passed; non-timing A/A diff empty.

#table(columns: (1.5fr,1.4fr,1.4fr), inset: 4pt,
  [phase], [$F_p$ multiplications], [group operations],
{rows_typ}
)

Counted totals, previous → dense: {old_m:,} → {new_m:,} $F_p$ multiplications; {old_g:,} → {new_g:,} group operations; {prior['residuals']} → 1 residuals. Matrix row operations are a separate unit. This exact group-oracle result does not price polynomial relation finding, hash CPU or a matched rho band. Raw records, commands and source hashes are in `README.md`.

#image("dense_cost.png", width: 100%)
"""
        (directory / "README.typ").write_text(body)
        shutil.copyfile(ROOT / "plot_quartic_followups.py", directory / "plot_quartic_followups.py")
        typst.compile(str(directory / "README.typ"), output=str(directory / "README.pdf"))
