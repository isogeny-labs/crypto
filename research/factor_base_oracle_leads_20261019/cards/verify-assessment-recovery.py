"""Read-only receipt/relay metadata verification; never executes the card."""
import copy
import hashlib
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
CARDS = Path(__file__).resolve().parent
CARD_REL = CARDS.relative_to(ROOT) / "CARD-oracle-leads-20261019-1.yaml"

spec = importlib.util.spec_from_file_location("canonical_task_card", ROOT / "tools/task_card.py")
tc = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(tc)

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

card_rel, card_sha, card = tc.load_card(CARD_REL, ROOT)
receipt_path = ROOT / card["receipt"]
receipt = json.loads(receipt_path.read_text())
relay_bytes = (CARDS / "executor-relay.txt").read_bytes()
relay = relay_bytes.decode()
assert receipt["card_path"] == card_rel
assert receipt["card_sha256"] == card_sha
assert receipt["card_id"] == card["card_id"]
assert receipt["commit"] == card["commit"]
assert receipt["status"] == "completed"
assert len(receipt["commands"]) == len(card["commands"]) == receipt["commands_total"]

for binding in card["binds"]:
    assert digest(ROOT / binding["path"]) == binding["sha256"], binding["path"]

runtime_order = copy.deepcopy(receipt)
streams = 0
output_checks = 0
for declared, recorded, reordered in zip(card["commands"], receipt["commands"], runtime_order["commands"]):
    assert recorded["argv"] == declared["argv"]
    assert recorded["cwd"] == declared.get("cwd", ".")
    assert recorded["id"] == declared["id"]
    assert recorded["attempts"] == 1 and len(recorded["attempt_log"]) == 1
    assert recorded["exit_code"] in declared["success_exit_codes"]
    assert recorded["timed_out"] is False and recorded["succeeded"] is True
    assert recorded["seconds"] < declared["timeout_seconds"]
    assert set(recorded["outputs"]) == set(declared["outputs"])
    for stream in ("stdout", "stderr"):
        path = ROOT / declared[stream]
        assert digest(path) == recorded[stream + "_sha256"]
        assert tc._tail(path.read_bytes()) == recorded[stream + "_tail"]
        streams += 1
    for path, expected in recorded["outputs"].items():
        assert digest(ROOT / path) == expected, path
        output_checks += 1
    # run_card constructs this dict in card order. Its serialized receipt
    # sorts keys; brief() itself consumes insertion order without sorting.
    reordered["outputs"] = {path: recorded["outputs"][path] for path in declared["outputs"]}

canonical_reloaded = tc.brief(receipt, card["receipt"], digest(receipt_path)) + "\n"
canonical_runtime = tc.brief(runtime_order, card["receipt"], digest(receipt_path)) + "\n"
assert relay_bytes == canonical_runtime.encode()
assert relay_bytes != canonical_reloaded.encode()
assert (CARDS / "canonical-runtime-order-brief.txt").read_bytes() == canonical_runtime.encode()
assert (CARDS / "canonical-reloaded-brief.txt").read_bytes() == canonical_reloaded.encode()

canonical_check = tc.check_relay(card, relay, ROOT)
assert canonical_check["relay_verbatim"] is False
assert canonical_check["verify_findings"] == []
canonical_facts = tc.assessment_facts(card, relay, ROOT)
assert canonical_facts["outputs_changed_since_run"] == []
assert canonical_facts["writes_outside_scope"] == []
before = set(receipt["dirty_before"])
new_paths = [path for path in receipt["dirty_after"] if path not in before]
assert all(tc._scope_covers(card["write_scope"], path) for path in new_paths)
declared_paths = [card["receipt"]]
for command in card["commands"]:
    declared_paths += [command["stdout"], command["stderr"], *command["outputs"]]
assert all(tc._scope_covers(card["write_scope"], path) for path in declared_paths)

experiment = ROOT / "research/factor_base_oracle_leads_20261019/runs/experiment"
manifest_path = experiment / "leads-manifest.json"
manifest = json.loads(manifest_path.read_text())
for path, expected in manifest["members"].items():
    tc.safe_relative(path)
    assert digest(experiment / path) == expected, path
actual_members = {path.relative_to(experiment).as_posix() for path in experiment.rglob("*") if path.is_file() and path != manifest_path}
assert actual_members == set(manifest["members"])
assert all(tc._scope_covers(card["write_scope"], (experiment / path).relative_to(ROOT).as_posix()) for path in actual_members)

result = {
    "scope": "read-only execution/relay/digest assessment, no research rerun or scientific interpretation",
    "card": card_rel,
    "card_sha256": card_sha,
    "source_commit": receipt["commit"],
    "receipt_sha256": digest(receipt_path),
    "bindings_match": len(card["binds"]),
    "commands_single_attempt_exit_zero": len(receipt["commands"]),
    "streams_verified": streams,
    "declared_output_digest_checks": output_checks,
    "manifest_members_verified": len(actual_members),
    "relay_byte_exact_to_runtime_insertion_order_brief": True,
    "relay_sha256": hashlib.sha256(relay_bytes).hexdigest(),
    "canonical_cli_pass": False,
    "canonical_check_relay": canonical_check,
    "canonical_assessment_outcome": canonical_facts["outcome"],
    "outputs_changed_since_run": [],
    "recorded_writes_outside_scope": [],
    "limitation": "Git dirty snapshots omit ignored paths; all declared files and sealed manifest members were separately checked within scope. No process tracing or independent-host certification.",
}
print(json.dumps(result, indent=2, sort_keys=True))
