import importlib.util, json, sys
from pathlib import Path
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("validation_gate", "/home/user/crypto/scripts/validation_gate.py")
vg = importlib.util.module_from_spec(spec); spec.loader.exec_module(vg)
rep = json.loads(Path(sys.argv[1]).read_text())
repo = Path("/home/user/crypto"); base = repo / "research/prime_ksum_pooled_20261029"
errors = []
vg.check_validator_report(rep, base, repo, errors)
print("check_validator_report(base=%s): %s" % (base, "OK, no errors" if not errors else errors))
print("raw_data entries:", len(rep["raw_data"]), "headline entries:", len(rep["headline"]), "status:", rep["status"], "independent:", rep["independent"])
