"""R9: result-affecting switches must reach the run records.

Run: python3 -m pytest tools/test_result_switches.py
"""
import ast
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
AUTOLAB = ROOT / "research/sat_factor_base_review_20260908/autolab"
for p in (HERE, AUTOLAB):
    if str(p) not in sys.path:
        sys.path.insert(0, str(p))

import result_switches as rs  # noqa: E402

RUST_SCOPES = ("src/cryptanalysis", "src/bin/ic", "src/bin/ic.rs", "src/bin/icx.rs",
               "src/bin/icprog.rs", "src/bin/icprog", "src/bin/ecbench.rs")


def test_every_rust_env_switch_is_recorded_or_exempt():
    """A new std::env::var("X") in the producers fails here until X is either
    matched by a recorded pattern or listed as unable to change a result."""
    names = rs.rust_env_names(ROOT, RUST_SCOPES)
    assert len(names) > 100, "scan found suspiciously few variables"
    missing = sorted(n for n in names if not rs.is_recorded(n) and n not in rs.NOT_RESULT_AFFECTING)
    assert not missing, f"unrecorded environment switches: {missing}"


def test_record_captures_switches_not_noise():
    env = {"KIC_FREE_RELATIONS": "1", "F4_FP_PACKED": "0", "RAYON_NUM_THREADS": "1",
           "GB_ENGINE": "f4", "PATH": "/bin", "HOME": "/h"}
    doc = rs.record(env, {"k": 125, "cpu": None, "func": print})
    assert doc["env"] == {"F4_FP_PACKED": "0", "GB_ENGINE": "f4",
                          "KIC_FREE_RELATIONS": "1", "RAYON_NUM_THREADS": "1"}
    assert doc["args"] == {"cpu": None, "k": 125}
    assert rs.record(dict(env, KIC_X="1"))["env_sha256"] != doc["env_sha256"]


def test_second_write_with_other_switches_is_kept_beside_the_first(tmp_path):
    path = tmp_path / "artifacts" / "switches.json"
    rs.write(path, {"KIC_A": "1"})
    rs.write(path, {"KIC_A": "1"})  # identical resume: nothing new
    rs.write(path, {"KIC_A": "2"})  # resumed under another environment
    assert json.loads(path.read_text())["env"] == {"KIC_A": "1"}
    assert json.loads(path.with_name("switches.1.json").read_text())["env"] == {"KIC_A": "2"}
    assert not path.with_name("switches.2.json").exists()


def _functions_with(path: Path, attr_chain: str):
    tree = ast.parse(path.read_text())
    found = []
    for fn in ast.walk(tree):
        if isinstance(fn, (ast.FunctionDef, ast.AsyncFunctionDef)):
            src = ast.get_source_segment(path.read_text(), fn) or ""
            found.append((fn.name, src))
    return [(n, s) for n, s in found if attr_chain in s]


def test_every_autolab_environment_copy_is_followed_by_a_switch_record():
    """Each function that builds a producer environment from os.environ also
    writes switches.json (write_switches)."""
    offenders = []
    for name in ("boundary_autolab.py", "single_target_panel.py", "equal_advice_pair.py"):
        for fn, src in _functions_with(AUTOLAB / name, "os.environ.copy()"):
            if "write_switches" not in src:
                offenders.append(f"{name}:{fn}")
    assert not offenders, f"producer environments with no switch record: {offenders}"


def test_autolab_launch_functions_record_every_parsed_argument():
    """launch, launch-panel, launch-single and launch-equal-advice all write the
    whole argparse namespace, so a new flag cannot be left out."""
    import boundary_autolab as lab
    parser = lab.parser()
    subs = next(a for a in parser._actions if a.choices and hasattr(a, "_name_parser_map"))
    checked = 0
    for name, sub in subs.choices.items():
        if not name.startswith("launch"):
            continue
        argv = [name]
        for action in sub._actions:
            if action.required and action.option_strings:
                argv += [action.option_strings[0], "x"]
        ns = parser.parse_args(argv)
        recorded = rs.record({}, ns)["args"]
        dests = {a.dest for a in sub._actions if a.dest not in ("help", "func")}
        assert dests <= set(recorded), f"{name}: {sorted(dests - set(recorded))}"
        checked += 1
    assert checked >= 4
    for fn in ("launch", "launch_panel", "launch_equal_advice"):
        src = dict(_functions_with(AUTOLAB / "boundary_autolab.py", "write_switches"))
        assert fn in src and "write_switches(run" in src[fn] and "arguments" in src[fn], fn
    single = dict(_functions_with(AUTOLAB / "single_target_panel.py", "write_switches"))
    assert "launch_single" in single and "arguments" in single["launch_single"]


def test_ic_tool_program_records_carry_the_switches():
    """bench.host_manifest (every ic_tool_program round) and isolated_bench's
    per-run record (every timed `ic price`) both write result_switches."""
    manifest = (ROOT / "research/ic_tool_program/harness/bench.py").read_text()
    assert '"result_switches": result_switches.record(ENV)' in manifest
    iso = (ROOT / "tools/isolated_bench.py").read_text()
    assert "record['result_switches'] = result_switches.record(os.environ)" in iso
