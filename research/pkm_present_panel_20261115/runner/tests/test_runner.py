import json, subprocess, sys
from pathlib import Path
import pytest
from runner_helpers import HERE, RUNNER, STUDY, write_protocol, make_cells, make_systems
import build_plan, run_plan

FI = str(HERE / "fake_instrument.py")
FM = str(HERE / "fake_msolve.py")
PARSE = STUDY.parent / "pkm_m8_sparse_panel_20261114/pod/parse_msolve_log.py"


def cell(i, m, t, curves):
    return {"id": i, "m": m, "t": t, "curves": curves}


def build(tmp, specs, cells, wall=1, **kw):
    cdir = make_cells(tmp, specs)
    write_protocol(tmp / "protocol.json", cells, wall)
    plan = build_plan.build(tmp, cdir, tmp / "protocol.json", **kw)
    (tmp / "plan.json").write_text(json.dumps(plan))
    return plan


def run(tmp, res, *extra):
    return run_plan.main(["--plan", str(tmp / "plan.json"), "--out", str(res), "--instrument", FI, "--msolve", FM,
                          "--parse-log", str(PARSE)] + list(extra))


def rows_of(res):
    return [json.loads(l) for l in (res / "rows.jsonl").read_text().splitlines()]


# ---------------------------------------------------------------- build_plan
def test_plan_order_not_applicable_and_m4t4_last(tmp_path):
    cells = [cell("m3t2", 3, 2, ["c1", "c2"]), cell("m4t4", 4, 4, ["c1"]) | {"runs_last": True},
             cell("m4t2", 4, 2, ["c1"]), cell("m3t3", 3, 3, ["c1"])]  # deliberately scrambled input order
    specs = [("c1", 2, 3, 2), ("c2", 2, 3, 4), ("c1", 2, 4, 2), ("c1", 3, 3, 5), ("c1", 4, 4, 5)]
    plan = build(tmp_path, specs, cells)
    rows = plan["rows"]
    # runs_last block is moved to the end; the rest keeps protocol order
    assert [c["cell"] for c in plan["cells"]] == ["m3t2", "m4t2", "m3t3", "m4t4"]
    assert rows[-1]["cell"] == "m4t4"
    # m3t2 / c1 (B_real = 2 < 4): instances in draw order, planted last (file order was planted first)
    blk = [r for r in rows if r["cell"] == "m3t2" and r["curve"] == "c1"]
    inst = []
    for r in blk:
        if r["instance"] not in inst:
            inst.append(r["instance"])
    assert inst == ["d000000", "d000001", "dplant"]
    first = [(r["arm"], r["engine"]) for r in blk if r["instance"] == "d000000"]
    assert first == [("chain-tower", "native"), ("chain-tower", "msolve"), ("chain-direct", "native"),
                     ("chain-direct", "msolve"), ("mono-direct", "native"), ("mono-direct", "msolve"),
                     ("sym-tower", "native"), ("sym-tower", "msolve"), ("sym-direct", "native"),
                     ("sym-direct", "msolve"), ("aa", "native"), ("aa", "msolve")]
    na = [r for r in blk if r["arm"].startswith("sym")]
    assert na and all(r["censored"] == "not_applicable" and r["system"] is None for r in na)
    # c2 has B_real = 4 >= m+1: the symmetric arms run there
    assert all("censored" not in r and r["system"] for r in rows if r["cell"] == "m3t2" and r["curve"] == "c2"
               and r["arm"].startswith("sym"))
    # arity-dependent arm sets; aa points at chain-tower files
    assert not any(r["arm"] == "tree-tower" for r in rows if r["m"] == 3)
    assert not any(r["arm"] == "mono-direct" for r in rows if r["m"] == 4)
    assert any(r["arm"] == "tree-tower" for r in rows if r["m"] == 4)
    aa = next(r for r in rows if r["arm"] == "aa" and r["engine"] == "native")
    assert aa["system"].endswith("chain-tower.system.json") and aa["out"].endswith("aa.native.json")
    assert all(r["cap_wall_s"] == 1 and r["as_gb"] == 10 for r in rows)
    assert [r["seq"] for r in rows] == list(range(len(rows)))


def test_plan_sha_and_missing_cell_is_loud(tmp_path):
    specs = [("c1", 2, 3, 4)]
    make_cells(tmp_path, specs)
    write_protocol(tmp_path / "protocol.json", [cell("m3t2", 3, 2, ["c1", "c2"])])
    with pytest.raises(build_plan.PlanError):
        build_plan.build(tmp_path, tmp_path / "cells", tmp_path / "protocol.json")
    plan = build_plan.build(tmp_path, tmp_path / "cells", tmp_path / "protocol.json", allow_missing=True)
    assert plan["dropped_curves"][0]["curve"] == "c2"
    rc = build_plan.main(["--study", str(tmp_path), "--allow-missing-curves"])
    assert rc == 0 and (tmp_path / "plan.sha256").read_text().split()[0] == run_plan.sha256_file(tmp_path / "plan.json")


# ---------------------------------------------------------------- runner
SMALL = [cell("m3t2", 3, 2, ["c1"]), cell("m3t3", 3, 3, ["c1"]), cell("m4t2", 4, 2, ["c1"])]
SPECS = [("c1", 2, 3, 4), ("c1", 3, 3, 4), ("c1", 2, 4, 4)]


def test_full_run_fields_censoring_and_resume(tmp_path):
    plan = build(tmp_path, SPECS, SMALL)

    def fake(r):
        # d000001 (unsat): chain-tower native sleeps past the 1 s cap -> timeout with w_lo from the progress file
        if r["arm"] == "chain-tower" and r["instance"] == "d000001" and r["cell"] == "m3t3":
            return {"sleep": 30, "ops": 4242, "msleep": 30}
        if r["instance"] == "d000000":
            return {"decomps": [[[1, 1], [2, -1], [3, 1]]], "solve": 300, "extract": 30}
        return {}
    make_systems(tmp_path, plan, fake)
    res = tmp_path / "results/t1"
    assert run(tmp_path, res, "--max-calls", "7") == 0       # interrupted run
    n1 = len(rows_of(res))
    assert 7 <= n1 < len(plan["rows"])
    assert run(tmp_path, res) == 0                            # resume
    rows = rows_of(res)
    keys = [run_plan.key_of(r) for r in rows]
    assert len(keys) == len(set(keys)) == len(plan["rows"])  # nothing run twice, nothing lost
    assert [run_plan.key_of(r) for r in rows[:n1]] == keys[:n1]
    ok = next(r for r in rows if r["cell"] == "m3t2" and r["arm"] == "chain-tower" and r["engine"] == "native"
              and r["instance"] == "d000000")
    assert ok["censored"] == "no" and ok["status"] == "ok" and ok["solve_field_multiplications"] == 300
    assert ok["extract_field_multiplications"] == 30 and ok["construction_field_multiplications"] == 5
    assert ok["verified_real_decompositions"] == [[[1, 1], [2, -1], [3, 1]]] and ok["rerun"] is False
    assert ok["threads_used"] == 1 and ok["max_rss_kb"] > 0 and ok["wall_s"] >= 0 and ok["real_count"] == 0
    cen = next(r for r in rows if r["cell"] == "m3t3" and r["arm"] == "chain-tower" and r["engine"] == "native"
               and r["instance"] == "d000001")
    assert cen["censored"] == "timeout" and cen["w_lo"] == 4242 and cen["progress_last"]["ops"] == 4242
    assert cen["status"] == "censored" and "solve_field_multiplications" not in cen
    cm = next(r for r in rows if r["cell"] == "m3t3" and r["arm"] == "chain-tower" and r["engine"] == "msolve"
              and r["instance"] == "d000001")
    assert cm["censored"] == "timeout" and cm["work_proxy"] is None and cm["work_proxy_lo"] > 0
    okm = next(r for r in rows if r["cell"] == "m3t2" and r["arm"] == "chain-tower" and r["engine"] == "msolve")
    assert okm["censored"] == "no" and okm["work_proxy"] == pytest.approx(10 * 20 * 0.5 * 2) and okm["rounds"] == 2
    assert okm["max_round_degree"] == 4 and okm["cpu_s"] is not None
    for f in ("lscpu.txt", "hashes.json", "env_dump.json", "versions.json", "field_sources.json", "resumes.jsonl"):
        assert (res / "env" / f).is_file()
    h = json.loads((res / "env/hashes.json").read_text())
    assert h["instrument_sha256"] == run_plan.sha256_file(FI) and h["plan_sha256"]
    assert json.loads((res / "progress.json").read_text())["done"] == len(rows)
    assert len((res / "env/resumes.jsonl").read_text().splitlines()) == 2


def test_not_applicable_rows_are_written_and_not_run(tmp_path):
    plan = build(tmp_path, [("c1", 2, 3, 2)], [cell("m3t2", 3, 2, ["c1"])])
    make_systems(tmp_path, plan)
    res = tmp_path / "results/na"
    assert run(tmp_path, res) == 0
    rows = rows_of(res)
    na = [r for r in rows if r["arm"].startswith("sym")]
    assert len(na) == 3 * 2 * 2 and all(r["censored"] == "not_applicable" and r["status"] == "not_applicable" for r in na)
    assert not any((res / r["out"]).exists() for r in na)


def test_stop_rule_skips_later_cells_of_same_m_only(tmp_path):
    plan = build(tmp_path, SPECS, SMALL)

    def fake(r):  # chain-tower native censored on 2 of the 3 instances of m3t2 -> 2*2 > 3
        if r["arm"] == "chain-tower" and r["cell"] == "m3t2" and r["instance"] in ("d000000", "d000001"):
            return {"sleep": 30}
        return {}
    make_systems(tmp_path, plan, fake)
    res = tmp_path / "results/stop"
    assert run(tmp_path, res) == 0
    rows = rows_of(res)
    m3t3 = [r for r in rows if r["cell"] == "m3t3"]
    assert m3t3 and all(r["status"] == "not_run" and "stop rule" in r["not_run_reason"] for r in m3t3)
    assert all(r["censored"] == "not_run" for r in m3t3)
    m4 = [r for r in rows if r["cell"] == "m4t2"]
    assert m4 and all(r["status"] in ("ok", "not_applicable") for r in m4)       # other m unaffected
    assert not any((res / r["out"]).exists() for r in m3t3)


def test_stop_rule_needs_strictly_more_than_half(tmp_path):
    # 3 instances, one censored: 2*1 > 3 is false -> m3t3 runs
    plan = build(tmp_path, SPECS, SMALL)
    make_systems(tmp_path, plan, lambda r: {"sleep": 30} if (r["arm"] == "chain-tower" and r["cell"] == "m3t2"
                                                              and r["instance"] == "d000000") else {})
    res = tmp_path / "results/half"
    assert run(tmp_path, res) == 0
    assert all(r["status"] != "not_run" for r in rows_of(res))


def test_lost_call_is_rerun_once_and_tagged(tmp_path):
    plan = build(tmp_path, SPECS[:1], SMALL[:1])
    make_systems(tmp_path, plan)
    res = tmp_path / "results/lost"
    assert run(tmp_path, res, "--max-calls", "2") == 0
    nxt = next(r for r in plan["rows"] if run_plan.key_of(r) not in run_plan.read_rows(res / "rows.jsonl")
               and r.get("status") != "not_applicable")
    (res / "inflight.json").write_text(json.dumps({"key": list(run_plan.key_of(nxt))}))
    assert run(tmp_path, res, "--max-calls", "1") == 0
    got = run_plan.read_rows(res / "rows.jsonl")[run_plan.key_of(nxt)]
    assert got["rerun"] is True and got["lost_count"] == 1 and got["status"] == "ok"
    # a call lost twice is not run a third time: it becomes a crash row
    nxt2 = next(r for r in plan["rows"] if run_plan.key_of(r) not in run_plan.read_rows(res / "rows.jsonl"))
    (res / "lost.jsonl").write_text((json.dumps({"key": list(run_plan.key_of(nxt2))}) + "\n") * 2)
    assert run(tmp_path, res, "--max-calls", "1") == 0
    g2 = run_plan.read_rows(res / "rows.jsonl")[run_plan.key_of(nxt2)]
    assert g2["censored"] == "crash" and "lost twice" in g2["censor_basis"] and g2["lost_count"] == 2


def test_missing_instrument_field_fails_loudly(tmp_path, capsys):
    plan = build(tmp_path, SPECS[:1], SMALL[:1])
    make_systems(tmp_path, plan, lambda r: {"drop": ["extract_field_multiplications"]})
    res = tmp_path / "results/bad"
    assert run(tmp_path, res) == 3
    assert "extract_field_multiplications" in capsys.readouterr().err
    assert not (res / "inflight.json").exists()


def test_crash_and_as_rlimit_classification(tmp_path):
    assert run_plan.classify(124, 600, 600, "", None, 10 << 30)[0] == "timeout"
    assert run_plan.classify(137, 599.9, 600, "", None, 10 << 30)[0] == "timeout"
    assert run_plan.classify(137, 20, 600, "", 100, 10 << 30)[0] == "crash"
    assert run_plan.classify(134, 20, 600, "memory allocation of 99 bytes failed", None, 10 << 30)[0] == "as_rlimit"
    assert run_plan.classify(139, 20, 600, "", 6 * 1024 ** 2, 10 << 30)[0] == "as_rlimit"
    assert run_plan.classify(1, 1, 600, "boom", 10, 10 << 30)[0] == "crash"
    assert run_plan.classify(0, 1, 600, "", 10, 10 << 30)[0] == "no"
    plan = build(tmp_path, SPECS[:1], SMALL[:1])
    make_systems(tmp_path, plan, lambda r: {"exit": 134, "stderr": "memory allocation of 8 bytes failed"}
                 if r["instance"] == "d000000" and r["arm"] == "chain-tower" else {})
    res = tmp_path / "results/rl"
    assert run(tmp_path, res) == 0
    r = next(r for r in rows_of(res) if r["arm"] == "chain-tower" and r["engine"] == "native" and r["instance"] == "d000000")
    assert r["censored"] == "as_rlimit" and r["rc"] == 134


def test_dry_prints_wrapped_commands_and_touches_nothing(tmp_path, capsys):
    plan = build(tmp_path, SPECS[:1], SMALL[:1])
    res = tmp_path / "results/dry"
    assert run(tmp_path, res, "--dry") == 0
    out = capsys.readouterr().out
    assert "taskset -c 0 /usr/bin/time -v -o" in out and "timeout -k 10 1 prlimit --as=10737418240" in out
    assert "--max-degree 64" in out and "--extract" in out and "-t 1 -v 2 -c 0 -g 2 -f" in out
    assert not res.exists()


def test_env_is_cleaned(tmp_path, monkeypatch):
    monkeypatch.setenv("F4_FP_GM", "1")
    monkeypatch.setenv("RAYON_NUM_THREADS", "8")
    e = run_plan.clean_env()
    assert "F4_FP_GM" not in e and e["RAYON_NUM_THREADS"] == "1"
