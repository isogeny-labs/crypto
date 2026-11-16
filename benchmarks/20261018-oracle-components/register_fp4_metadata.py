"""Metadata-only EC1/UID registration; native arithmetic is in registration-export.rs."""
import copy
import hashlib
import json
from pathlib import Path
import yaml
from tools.curve_identity import curve_identity

class RegistryDumper(yaml.SafeDumper):
    def increase_indent(self, flow=False, indentless=False):
        return super().increase_indent(flow, False)

root = Path(__file__).resolve().parents[2]
here = Path(__file__).resolve().parent
registry = root / "docs/curves/ic/curves.yaml"
raw = registry.read_bytes()
before = yaml.safe_load(raw)
native = json.loads((here / "registration-native.json").read_text())
main_registry = json.loads((root / "docs/curves/registry.json").read_text())
main_slugs = {entry["slug"] for entry in main_registry["curves"]}
additions, verified = {}, []
assert len(native["records"]) == 4
for row in native["records"]:
    identity = curve_identity(row["field"], row["curve"], "tau2")
    assert all(old.get("curve_id") != identity["curve_id"] for old in before["curves"].values())
    alias = f"ideas20261018_fp4p{row['p']}_seed{row['seed']}_h{identity['curve_sha256'][:12]}"
    assert alias not in before["curves"]
    additions[alias] = {
        "label": f"2026-10-18 factorial translation fixture over F_{row['p']}^4, seed {row['seed']}",
        "curve_tag": "tau2", "curve_ref": f"fp4_translation/{row['p']}/seed{row['seed']}",
        **identity, "field": copy.deepcopy(row["field"]), "curve": copy.deepcopy(row["curve"]),
        "icv1_identity": copy.deepcopy(row["icv1_identity"]),
        "legacy_constructor_identity": copy.deepcopy(row["legacy_constructor_identity"]),
        "endomorphism": {"endomorphism_order_conductor": None, "frobenius_order_conductor": None,
                         "volcano_levels": None, "status": "unproved_not_used"},
        "isogeny_routes": {"incoming": [], "outgoing": []},
        "factor_base_refs": [], "factor_base_link_status": "not_reconciled",
        "evidence_ref": "research/ideas_execution_20261018/oracle_integration/runs/factorial-current-session/plan.json",
        "registration_evidence": "benchmarks/20261018-oracle-components/registration-native.json",
        "registration_scope": "Exact EC1 crosswalk alias only; ecbench Fp4 runtime flags and trait arithmetic are not promoted.",
    }
    verified.append({"alias": alias, **identity,
                     "main_model_registry_present": row["icv1_identity"]["slug"] in main_slugs,
                     "canonical_model_slug": row["icv1_identity"]["slug"],
                     "legacy_constructor_slug": row["legacy_constructor_identity"]["slug"],
                     "native_checks": row["checks"]})

snapshot = here / "crosswalk-before-registration.yaml"
with snapshot.open("xb") as handle:
    handle.write(raw)
appendix = "\n# Exact Fp4 factorial-constructor aliases; arithmetic and native checks are separate evidence.\n"
appendix += "".join("  " + line + "\n" for line in yaml.dump(additions, Dumper=RegistryDumper, sort_keys=False, allow_unicode=True, width=1000000).splitlines())
registry.write_bytes(raw + appendix.encode())
after = yaml.safe_load(registry.read_bytes())
assert all(after["curves"][key] == value for key, value in before["curves"].items())
for alias, row in additions.items():
    assert curve_identity(after["curves"][alias]["field"], after["curves"][alias]["curve"], "tau2")["curve_uid"] == row["curve_uid"]
receipt = {
    "schema": "campaign-fp4-registry-verification/v1", "records_added": len(additions),
    "old_records_unchanged": True, "registry_before_sha256": hashlib.sha256(raw).hexdigest(),
    "registry_after_sha256": hashlib.sha256(registry.read_bytes()).hexdigest(),
    "native_export_sha256": hashlib.sha256((here / "registration-native.json").read_bytes()).hexdigest(),
    "records": verified,
    "scope": "Exact metadata hashes plus separate native field/order/constructor checks. No prior session, candidate result or runtime registered/ec1 flag is rewritten.",
}
with (here / "registration-verification.json").open("x") as handle:
    json.dump(receipt, handle, indent=2)
    handle.write("\n")
print(json.dumps({"added": len(additions), "old_records_unchanged": True,
                  "registry_sha256": receipt["registry_after_sha256"]}))
