#!/usr/bin/env python3
"""Experiment specs (crypto.experiment/v1): validate, track, and compile into
mechanical task cards.

An experiment spec (experiments/specs/<id>.yaml) is the decided, frozen plan:
question, cells, steps, gates, hosts, budget, outcome rules. This tool is the
only thing that reads it during a run. It never interprets results and never
runs an experiment command itself: each step instance becomes one
tools/task_card.py frozen-command card, which executor-mechanical (Haiku)
runs unchanged. Progress goes to experiments/specs/<id>.state.yaml.

The run loop, driven by the main session (README.md, "The loop"):

    python3 tools/expspec/expspec.py next <spec>      # prints one JSON action
      run_card -> relay `python3 tools/task_card.py prompt <card>` to executor-mechanical
      ask      -> ask the owner; record with `expspec.py answer`
      wait     -> schedule a check-in at `until`; do nothing else
      escalate -> hand `reason` to the reasoning tier
      done / stopped -> report
    (repeat; `next` ingests any receipt that appeared since the last call)

    python3 tools/expspec/expspec.py validate <spec>...
    python3 tools/expspec/expspec.py plan <spec>          # expanded instances, works on drafts
    python3 tools/expspec/expspec.py ls [<dir>]
    python3 tools/expspec/expspec.py answer <spec> --gate <id> (--yes|--no) --by <name>
    python3 tools/expspec/expspec.py facts <spec>         # metrics and artifacts, no verdict

Exit status: 0 ok, 1 findings / refused, 2 unreadable input.
"""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import itertools
import json
import re
import shlex
import subprocess
import sys
from pathlib import Path, PurePosixPath
from typing import Any

import yaml

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
SPEC_SCHEMA = HERE / "experiment.v1.schema.json"
STATE_SCHEMA = HERE / "experiment-state.v1.schema.json"
CARD_SCHEMA = "crypto.autoresearch.mechanical-task-card.v1"
TEMPLATE = re.compile(r"\{\{\s*([A-Za-z_][A-Za-z0-9_.-]*)\s*\}\}")
TERMINAL = {"done", "skipped_done", "failed", "censored", "not_attempted", "void"}
OK_DONE = {"done", "skipped_done"}
LIFECYCLE = ("provision", "poll", "collect", "teardown")
STOP_WHEN = ["python3 tools/task_card.py run exits with status 2."]


class SpecError(Exception):
    pass


def now() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def parse_ts(text: str) -> dt.datetime:
    return dt.datetime.fromisoformat(text.replace("Z", "+00:00"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*args: str) -> tuple[int, str]:
    r = subprocess.run(["git", *args], cwd=REPO, capture_output=True, text=True)
    return r.returncode, r.stdout.strip()


def committed(relative: str) -> bool:
    code, blob = git("rev-parse", f"HEAD:{relative}")
    return code == 0 and git("hash-object", relative) == (0, blob)


# ---------------------------------------------------------------------------
# Loading, templates, expansion
# ---------------------------------------------------------------------------

class _Loader(yaml.SafeLoader):
    """SafeLoader that keeps dates and timestamps as the strings written."""


_Loader.yaml_implicit_resolvers = {k: [r for r in v if r[0] != "tag:yaml.org,2002:timestamp"]
                                   for k, v in yaml.SafeLoader.yaml_implicit_resolvers.items()}


def load_yaml(path: Path) -> Any:
    try:
        return yaml.load(path.read_text(encoding="utf-8"), Loader=_Loader)  # noqa: S506
    except (OSError, yaml.YAMLError) as exc:
        raise SpecError(f"{path}: {exc}") from None


def lookup(ctx: dict[str, Any], dotted: str) -> Any:
    cur: Any = ctx
    for part in dotted.split("."):
        if isinstance(cur, dict) and part in cur:
            cur = cur[part]
        else:
            raise KeyError(dotted)
    return cur


def render(value: Any, ctx: dict[str, Any]) -> Any:
    if isinstance(value, str):
        def sub(m: re.Match) -> str:
            got = lookup(ctx, m.group(1))
            if isinstance(got, (dict, list)):
                raise KeyError(m.group(1))
            return str(got)
        return TEMPLATE.sub(sub, value)
    if isinstance(value, list):
        return [render(v, ctx) for v in value]
    if isinstance(value, dict):
        return {k: render(v, ctx) for k, v in value.items()}
    return value


def resolve_paths(spec: dict[str, Any]) -> dict[str, str]:
    paths = dict(spec["paths"])
    paths.setdefault("cards", "{{paths.root}}/cards")
    paths.setdefault("collected", "{{paths.root}}/collected")
    for _ in range(6):
        ctx = {"paths": paths, "exp": {"id": spec["id"], "version": spec["version"]}}
        new = {k: render(v, ctx) for k, v in paths.items()}
        if new == paths:
            break
        paths = new
    for k, v in paths.items():
        if TEMPLATE.search(v):
            raise SpecError(f"paths.{k}: unresolved template in {v!r}")
        p = PurePosixPath(v)
        if p.is_absolute() or ".." in p.parts:
            raise SpecError(f"paths.{k}: must be repo-relative without '..': {v!r}")
    return paths


def expand_cells(spec: dict[str, Any]) -> list[dict[str, Any]]:
    cells = [dict(c) for c in spec.get("cells") or []]
    m = spec.get("matrix")
    if m:
        keys = list(m["axes"])
        for combo in itertools.product(*(m["axes"][k] for k in keys)):
            params = dict(zip(keys, combo))
            if any(all(params.get(k) == v for k, v in ex.items()) for ex in m.get("exclude") or []):
                continue
            cell = {**(m.get("defaults") or {}), "params": params}
            cell["id"] = render(m["id_template"], params)
            cells.append(cell)
    for c in cells:
        c.setdefault("role", "measure")
        c.setdefault("priority", 0)
        c.setdefault("optional", False)
    return cells


def fits(provides: dict[str, Any], need: dict[str, Any] | None) -> bool:
    if not need:
        return True
    for key in ("vcpus", "memory_gb", "disk_gb", "gpus"):
        if key in need and float(provides.get(key, 0)) < float(need[key]):
            return False
    if need.get("arch", "any") not in ("any", provides.get("arch", "any")) and provides.get("arch", "any") != "any":
        return False
    return set(need.get("cpu_flags") or []) <= set(provides.get("cpu_flags") or [])


def cells_for_step(step: dict[str, Any], cells: list[dict[str, Any]]) -> list[dict[str, Any]]:
    out = cells
    if step.get("cells"):
        out = [c for c in out if c["id"] in step["cells"]]
    if step.get("cell_roles"):
        out = [c for c in out if c["role"] in step["cell_roles"]]
    return sorted(out, key=lambda c: -c["priority"])


def instance_key(step_id: str, cell: str | None = None, host: int | None = None) -> str:
    if cell is not None:
        return f"{step_id}@{cell}"
    if host is not None:
        return f"{step_id}#{host}"
    return step_id


# ---------------------------------------------------------------------------
# Validation
# ---------------------------------------------------------------------------

def schema_findings(doc: Any, schema_path: Path) -> list[str]:
    import jsonschema  # noqa: PLC0415
    validator = jsonschema.Draft202012Validator(json.loads(schema_path.read_text()))
    out = []
    for err in sorted(validator.iter_errors(doc), key=lambda e: list(e.absolute_path)):
        where = "/".join(str(p) for p in err.absolute_path) or "(root)"
        out.append(f"{where}: {err.message}")
    return out


def semantic_findings(spec: dict[str, Any], spec_path: Path) -> tuple[list[str], list[str]]:
    errors: list[str] = []
    warnings: list[str] = []
    if spec_path.stem != spec["id"]:
        errors.append(f"file name {spec_path.name} must be {spec['id']}.yaml")
    try:
        paths = resolve_paths(spec)
    except SpecError as exc:
        return [str(exc)], warnings
    cells = expand_cells(spec)
    cell_ids = [c["id"] for c in cells]
    dup = {c for c in cell_ids if cell_ids.count(c) > 1}
    if dup:
        errors.append(f"cells: duplicate ids {sorted(dup)}")
    slug = re.compile(r"^[a-z][a-z0-9-]{0,47}$")
    for c in cells:
        if not slug.match(c["id"]):
            errors.append(f"cells: generated id {c['id']!r} is not a slug")
    steps = spec["steps"]
    step_ids = [s["id"] for s in steps]
    if len(set(step_ids)) != len(step_ids):
        errors.append("steps: duplicate ids")
    gate_ids = {g["id"] for g in spec.get("gates") or []}
    host_backends = {h["backend"] for h in spec["hosts"]["candidates"]}
    seen: set[str] = set()
    for s in steps:
        at = f"steps.{s['id']}"
        for n in s.get("needs") or []:
            if n not in step_ids:
                errors.append(f"{at}.needs: unknown step {n}")
            elif n not in seen:
                errors.append(f"{at}.needs: {n} must come earlier in `steps`")
        seen.add(s["id"])
        for g in (s.get("gates") or {}).get("before", []) + (s.get("gates") or {}).get("after", []):
            if g not in gate_ids:
                errors.append(f"{at}.gates: unknown gate {g}")
        for cid in s.get("cells") or []:
            if cid not in cell_ids:
                errors.append(f"{at}.cells: unknown cell {cid}")
        if s["scope"] == "per_cell" and not cells_for_step(s, cells):
            errors.append(f"{at}: per_cell step selects no cells")
        if s["runs_on"] == "host" and "batch" in host_backends and "status_done" not in s:
            errors.append(f"{at}: a host step needs status_done when any candidate is a batch host")
        if s["runs_on"] == "host" and "batch" in host_backends and s.get("gates"):
            errors.append(f"{at}.gates: a batch host runs its steps by itself, so expspec cannot gate this one; "
                          "gate a controller step that reads the collected outputs, or rely on the launch ask")
        if s["runs_on"] == "host" and host_backends - {"batch"} and "argv" not in s:
            errors.append(f"{at}: needs argv to run on a local or ssh host")
        if s.get("evidence") and s.get("done_when"):
            warnings.append(f"{at}: evidence step with done_when; make sure done_when reads the committed output, "
                            "not a partial file")
        # Every template must resolve against a representative context.
        sample_cells = cells_for_step(s, cells) if s["scope"] == "per_cell" else [None]
        for cell in sample_cells[:1] or [None]:
            ctx = context(spec, paths, cell, spec["hosts"]["candidates"][0], 0, {"handle": "x"}, probe=True)
            for field in ("argv", "cwd", "env", "outputs", "must_not_exist", "status_done", "done_when", "write_scope"):
                if field in s:
                    try:
                        render(s[field], ctx)
                    except KeyError as exc:
                        errors.append(f"{at}.{field}: unknown template variable {exc.args[0]}")
    used = {g for s in steps for side in (s.get("gates") or {}).values() for g in side}
    for g in sorted(gate_ids - used):
        warnings.append(f"gates.{g}: no step uses it")
    for h in spec["hosts"]["candidates"]:
        for cid in h.get("cells") or []:
            if cid not in cell_ids:
                errors.append(f"hosts.{h['id']}.cells: unknown cell {cid}")
        if h["backend"] == "batch":
            prov = h["lifecycle"]["provision"]
            if "handle" not in (prov.get("capture") or {}):
                errors.append(f"hosts.{h['id']}.lifecycle.provision: must capture `handle`")
            if h["preemptible"] and (spec.get("checkpoint") or {}).get("resume") == "from_ckpt" \
                    and (spec.get("checkpoint") or {}).get("transport", "none") == "none":
                errors.append(f"hosts.{h['id']}: preemptible with resume from_ckpt needs checkpoint.transport")
    for c in cells:
        if not any(fits(h["provides"], c.get("requires")) and (not h.get("cells") or c["id"] in h["cells"])
                   for h in spec["hosts"]["candidates"]):
            (warnings if c["optional"] else errors).append(
                f"cells.{c['id']}: no host candidate fits; it will be recorded not_attempted")
    if any(m["kind"] == "timing" for m in spec.get("metrics") or []) and \
            not any(h["timing_valid"] for h in spec["hosts"]["candidates"]):
        warnings.append("metrics: a timing metric with no timing_valid host; wall times will be descriptive only")
    if spec["status"] == "approved":
        for f in spec["source"]["frozen"]:
            p = REPO / f["path"]
            if not p.is_file():
                errors.append(f"source.frozen: {f['path']} does not exist")
            elif sha256_file(p) != f["sha256"]:
                errors.append(f"source.frozen: {f['path']} does not match its sha256")
        if git("cat-file", "-e", spec["source"]["commit"] + "^{commit}")[0] != 0:
            errors.append(f"source.commit: {spec['source']['commit']} is not a commit here")
        dec = REPO / spec["approval"]["decision"]
        if not dec.is_file():
            errors.append(f"approval.decision: {spec['approval']['decision']} does not exist")
        else:
            rec = (load_yaml(dec) or {}).get("coordinator_decision") or {}
            if rec.get("decision") != "approve" or spec["id"] not in (rec.get("target_ids") or []):
                errors.append(f"approval.decision: {spec['approval']['decision']} does not approve {spec['id']}")
            elif not committed(spec["approval"]["decision"]):
                errors.append(f"approval.decision: {spec['approval']['decision']} is not committed as it is on disk")
    budget = spec["budget"]
    est = sum((c.get("requires") or {}).get("est_hours", 0) for c in cells)
    hourly = max((h.get("est_hourly_usd", 0) for h in spec["hosts"]["candidates"]), default=0)
    if est and hourly and est * hourly > budget["max_cost_usd"]:
        warnings.append(f"budget: cells' est_hours ({est:g} h) x the dearest host ({hourly:g} $/h) "
                        f"exceeds max_cost_usd {budget['max_cost_usd']:g}; optional cells may be dropped")
    if (REPO / paths["root"]).exists() is False and spec["status"] == "approved":
        warnings.append(f"paths.root: {paths['root']} does not exist yet")
    return errors, warnings


def validate(spec_path: Path) -> tuple[dict[str, Any] | None, list[str], list[str]]:
    spec = load_yaml(spec_path)
    if not isinstance(spec, dict):
        return None, ["spec must be a mapping"], []
    errors = schema_findings(spec, SPEC_SCHEMA)
    if errors:
        return spec, errors, []
    e, w = semantic_findings(spec, spec_path)
    return spec, e, w


# ---------------------------------------------------------------------------
# State
# ---------------------------------------------------------------------------

def state_path(spec_path: Path) -> Path:
    return spec_path.with_name(spec_path.stem + ".state.yaml")


def event(state: dict[str, Any], kind: str, text: str) -> None:
    state["events"].append({"at": now(), "kind": kind, "text": text})


def save_state(spec_path: Path, state: dict[str, Any]) -> None:
    state["updated_at"] = now()
    errs = schema_findings(state, STATE_SCHEMA)
    if errs:
        raise SpecError("internal: state would not validate: " + "; ".join(errs[:5]))
    tmp = state_path(spec_path).with_suffix(".tmp")
    tmp.write_text(yaml.safe_dump(state, sort_keys=False, width=100), encoding="utf-8")
    tmp.replace(state_path(spec_path))


def load_state(spec_path: Path, spec: dict[str, Any]) -> dict[str, Any]:
    sp = state_path(spec_path)
    digest = sha256_file(spec_path)
    if not sp.exists():
        state = {"schema": "crypto.experiment-state/v1", "experiment_id": spec["id"],
                 "spec_version": spec["version"], "spec_sha256": digest, "status": "not_started",
                 "blocked_on": None, "created_at": now(), "updated_at": now(), "vars": {},
                 "hosts": [], "instances": {}, "gates": {}, "events": []}
        event(state, "init", f"state created for {spec['id']} v{spec['version']}")
        return state
    state = load_yaml(sp)
    if state.get("spec_sha256") != digest:
        raise SpecError(f"spec_drift: {spec_path.name} changed since the run started "
                        f"(state pins {state.get('spec_sha256')}, file is {digest}). "
                        "An approved spec is frozen; amend with a new version and a new state.")
    return state


def host_index(state: dict[str, Any]) -> int | None:
    for i, h in enumerate(state["hosts"]):
        if h["state"] in ("provisioning", "up"):
            return i
    return None


def spend(spec: dict[str, Any], state: dict[str, Any]) -> float:
    total = 0.0
    cands = {h["id"]: h for h in spec["hosts"]["candidates"]}
    for h in state["hosts"]:
        if not h.get("started_at"):
            continue
        start = parse_ts(h["started_at"])
        end = parse_ts(h["ended_at"]) if h.get("ended_at") else dt.datetime.now(dt.timezone.utc)
        life = (cands.get(h["candidate"], {}).get("lifecycle") or {}).get("lifetime_s")
        if life:
            end = min(end, start + dt.timedelta(seconds=life))
        total += max(0.0, (end - start).total_seconds()) / 3600 * h.get("hourly_usd", 0)
    return round(total, 2)


# ---------------------------------------------------------------------------
# Checks
# ---------------------------------------------------------------------------

def json_value(ref: dict[str, Any], ctx: dict[str, Any]) -> Any:
    path = REPO / render(ref["file"], ctx)
    text = path.read_text(encoding="utf-8")
    if path.suffix == ".jsonl":
        rows = [json.loads(line) for line in text.splitlines() if line.strip()]
        line = ref.get("line", "last")
        docs = rows if line == "all" else [rows[-1] if line == "last" else rows[0]]
    else:
        docs = [json.loads(text)]
    out = []
    for doc in docs:
        cur = doc
        for part in filter(None, re.split(r"\.|(\[\d+\])", ref.get("path") or "")):
            cur = cur[int(part[1:-1])] if part.startswith("[") else cur[part]
        out.append(cur)
    return out if ref.get("line") == "all" else out[0]


def run_check(check: dict[str, Any], ctx: dict[str, Any], paths: dict[str, str]) -> tuple[bool, str]:
    kind = check["kind"]
    try:
        if kind == "file_exists":
            p = REPO / render(check["path"], ctx)
            return p.exists() and (p.is_dir() or p.stat().st_size > 0), str(p.relative_to(REPO))
        if kind == "sha256":
            p = REPO / render(check["path"], ctx)
            got = sha256_file(p) if p.is_file() else None
            return got == check["sha256"], f"{p.relative_to(REPO)} sha256 {got}"
        if kind == "sha256_manifest":
            p = REPO / render(check["path"], ctx)
            if not p.is_file() or p.stat().st_size == 0:
                return False, f"{p.relative_to(REPO)} missing"
            for line in p.read_text().splitlines():
                if not line.strip():
                    continue
                digest, name = line.split(maxsplit=1)
                f = p.parent / name.lstrip("*")
                if not f.is_file() or sha256_file(f) != digest:
                    return False, f"{f.relative_to(REPO)} does not verify"
            return True, f"{p.relative_to(REPO)} verifies"
        if kind == "status_token":
            p = REPO / paths["collected"] / "STATUS"
            token = render(check["token"], ctx)
            lines = p.read_text().splitlines() if p.is_file() else []
            return any(line.rstrip().endswith(token) for line in lines), f"STATUS token {token}"
        if kind == "command":
            r = subprocess.run(render(check["argv"], ctx), cwd=REPO, capture_output=True,
                               timeout=check.get("timeout_s", 120))
            return r.returncode == 0, f"exit {r.returncode}"
        if kind == "json":
            op, want = check["op"], check.get("value")
            try:
                got = json_value(check["json"], ctx)
            except (OSError, KeyError, IndexError, ValueError, TypeError) as exc:
                return (False, f"unreadable: {exc}") if op != "exists" else (False, "absent")
            ok = {"eq": lambda: got == want, "ne": lambda: got != want, "lt": lambda: got < want,
                  "le": lambda: got <= want, "gt": lambda: got > want, "ge": lambda: got >= want,
                  "in": lambda: got in want, "exists": lambda: True, "truthy": lambda: bool(got),
                  "all_true": lambda: bool(got) and all(bool(x) for x in got),
                  "count_ge": lambda: sum(1 for x in got if x) >= want}[op]()
            return bool(ok), f"value {json.dumps(got)[:120]} {op} {json.dumps(want)}"
    except (OSError, subprocess.TimeoutExpired, KeyError) as exc:
        return False, f"{kind}: {exc}"
    return False, f"unknown check {kind}"


def context(spec, paths, cell, host, hidx, vars_, probe=False) -> dict[str, Any]:
    ctx: dict[str, Any] = {"exp": {"id": spec["id"], "version": spec["version"]}, "paths": paths,
                           "vars": dict(vars_), "env": spec.get("env") or {}}
    if cell is not None:
        ctx["cell"] = {"id": cell["id"], "role": cell["role"], **cell["params"]}
    if host is not None:
        ctx["host"] = {"id": host["id"], "index": hidx, **(host.get("config") or {}),
                       **{f"provides_{k}": v for k, v in host["provides"].items() if not isinstance(v, list)}}
    if probe:
        ctx["vars"] = _AnyDict()
    return ctx


class _AnyDict(dict):
    """validate-time stand-in for vars captured at run time."""
    def __contains__(self, key):  # noqa: D401
        return True

    def __getitem__(self, key):
        return "VAR"


# ---------------------------------------------------------------------------
# Cards
# ---------------------------------------------------------------------------

def bus_address(spec_id: str) -> str:
    return "executor-mechanical-" + re.sub(r"[^a-z0-9-]", "-", spec_id.lower())[:40]


def write_card(spec, spec_path, state, paths, key, command: dict[str, Any], extra_scope: list[str]) -> str:
    head = git("rev-parse", "HEAD")[1]
    src = spec["source"]["commit"]
    if head != src:
        if git("merge-base", "--is-ancestor", src, head)[0] != 0:
            raise SpecError(f"spec_drift: HEAD {head} does not descend from source.commit {src}")
        if not spec["source"].get("allow_drift"):
            allowed = ("experiments/specs/", str(PurePosixPath(spec["approval"]["decision"]).parent) + "/",
                       paths["cards"] + "/", paths["collected"] + "/", paths["out"] + "/")
            changed = git("diff", "--name-only", src, head)[1].splitlines()
            stray = [c for c in changed if not c.startswith(allowed)]
            if stray:
                raise SpecError(f"spec_drift: commits after source.commit change {stray[:5]}; "
                                "only specs, the decision and run products may change unless allow_drift")
    binds = []
    for f in spec["source"]["frozen"]:
        p = REPO / f["path"]
        if not p.is_file() or sha256_file(p) != f["sha256"]:
            raise SpecError(f"spec_drift: frozen file {f['path']} is missing or changed")
        binds.append({"path": f["path"], "sha256": f["sha256"]})
    bound = {b["path"] for b in binds}
    for token in command["argv"]:
        if token not in bound and (REPO / token).is_file():
            binds.append({"path": token, "sha256": sha256_file(REPO / token)})
            bound.add(token)
    seq = sum(len(i["attempts"]) for i in state["instances"].values()) + 1
    card_id = f"CARD-{spec['id']}-{seq:03d}"
    cards = paths["cards"]
    command = {"id": re.sub(r"[^a-z0-9-]", "-", key.lower())[:60].strip("-") or "step", **command,
               "stdout": f"{cards}/{card_id}.stdout", "stderr": f"{cards}/{card_id}.stderr"}
    scope = sorted({paths["root"], paths["out"], cards, paths["collected"], *extra_scope,
                    *(p for k, p in paths.items() if k not in ("root",))})
    card = {
        "schema": CARD_SCHEMA, "card_id": card_id, "experiment_id": spec["id"],
        "author_role": "experiment-director", "executor_role": "executor-mechanical",
        "kind": "frozen_command", "commit": head,
        "approval_decision": spec["approval"]["decision"],
        "receipt": f"{cards}/{card_id}.receipt.json",
        "binds": binds, "must_not_exist": command.pop("must_not_exist", []),
        "commands": [command], "bus_address": bus_address(spec["id"]),
        "background": bool(command.pop("background")), "write_scope": scope, "stop_when": STOP_WHEN,
        "notes": f"Compiled by tools/expspec/expspec.py from {spec_path.relative_to(REPO)} "
                 f"v{spec['version']}, instance {key}.",
    }
    for d in paths.values():
        (REPO / d).mkdir(parents=True, exist_ok=True)
    path = REPO / cards / f"{card_id}.yaml"
    if path.exists():
        raise SpecError(f"{path} exists; cards are write-once")
    path.write_text(yaml.safe_dump(card, sort_keys=False, width=200), encoding="utf-8")
    sys.path.insert(0, str(REPO / "tools"))
    import task_card  # noqa: PLC0415
    findings = task_card.lint(card, REPO)
    if findings:
        path.unlink()
        raise SpecError("compiled card does not lint (fix the spec): " + "; ".join(findings))
    rel_card = str(path.relative_to(REPO))
    inst = state["instances"].setdefault(key, {"step": key.split("@")[0].split("#")[0], "cell": None,
                                               "host": None, "status": "pending", "reason": None, "attempts": []})
    inst["status"] = "carded"
    inst["attempts"].append({"card": rel_card, "receipt": card["receipt"], "receipt_status": None,
                             "exit_code": None, "seconds": None, "carded_at": now(), "ingested_at": None,
                             "outputs": {}})
    event(state, "card", f"{key}: {rel_card}")
    return rel_card


def step_command(spec, step, ctx, host) -> tuple[dict[str, Any], list[str]]:
    argv = render(step["argv"], ctx)
    if step["runs_on"] == "host" and host["backend"] == "ssh":
        workdir = host["config"]["workdir"]
        envs = " ".join(f"{k}={shlex.quote(v)}" for k, v in {**(spec.get("env") or {}),
                                                            **(step.get("env") or {})}.items())
        argv = ["ssh", host["config"]["host"],
                f"cd {shlex.quote(workdir)} && {envs + ' ' if envs else ''}{shlex.join(argv)}"]
    cmd = {"argv": argv, "timeout_seconds": step["timeout_s"],
           "success_exit_codes": step.get("success_exit_codes", [0]),
           "outputs": render(step.get("outputs") or [], ctx) if step["runs_on"] == "controller"
           or host["backend"] == "local" else [],
           "env": {k: str(v) for k, v in render({**(spec.get("env") or {}), **(step.get("env") or {})}, ctx).items()},
           "must_not_exist": render(step.get("must_not_exist") or [], ctx),
           "background": step.get("background", step["timeout_s"] > 600)}
    if step.get("cwd"):
        cmd["cwd"] = render(step["cwd"], ctx)
    if step.get("retry_exit_codes"):
        cmd["retry"] = {"exit_codes": step["retry_exit_codes"],
                        "max_attempts": max(2, min(3, spec["budget"].get("max_attempts_per_step", 2)))}
    if not cmd["env"]:
        del cmd["env"]
    return cmd, render(step.get("write_scope") or [], ctx)


def lifecycle_command(spec, host, phase, ctx) -> dict[str, Any]:
    lc = host["lifecycle"][phase]
    cmd = {"argv": render(lc["argv"], ctx), "timeout_seconds": lc["timeout_s"], "success_exit_codes": [0],
           "outputs": [], "must_not_exist": [], "background": lc["timeout_s"] > 600}
    if lc.get("env"):
        cmd["env"] = render(lc["env"], ctx)
    if lc.get("retry_exit_codes"):
        cmd["retry"] = {"exit_codes": lc["retry_exit_codes"], "max_attempts": 3}
    return cmd


# ---------------------------------------------------------------------------
# Ingest
# ---------------------------------------------------------------------------

def capture(spec_capture: dict[str, Any] | None, stdout: str, state: dict[str, Any]) -> None:
    for name, how in (spec_capture or {}).items():
        val = None
        if "json_key" in how:
            for line in stdout.splitlines():
                try:
                    doc = json.loads(line)
                except ValueError:
                    continue
                if isinstance(doc, dict) and how["json_key"] in doc:
                    val = str(doc[how["json_key"]])
                    break
        else:
            m = re.search(how["regex"], stdout)
            val = m.group(1) if m else None
        if val is None:
            raise SpecError(f"capture {name}: not found in stdout")
        state["vars"][name] = val
        event(state, "capture", f"{name} = {val}")


def ingest_pending(spec, spec_path, state, paths, cells_by_id) -> list[str]:
    """Read every receipt that appeared since the last call. Returns escalation reasons."""
    problems: list[str] = []
    for key, inst in state["instances"].items():
        if inst["status"] != "carded":
            continue
        att = inst["attempts"][-1]
        rpath = REPO / att["receipt"]
        if not rpath.is_file():
            continue
        receipt = json.loads(rpath.read_text())
        cmd = receipt["commands"][-1] if receipt["commands"] else {}
        att.update(receipt_status=receipt["status"], exit_code=cmd.get("exit_code"),
                   seconds=cmd.get("seconds"), ingested_at=now(), outputs=cmd.get("outputs") or {})
        ok = receipt["status"] == "completed"
        stdout_p = REPO / paths["cards"] / (Path(att["card"]).stem + ".stdout")
        stdout = stdout_p.read_text(errors="replace") if stdout_p.is_file() else ""
        name = inst["step"]
        if name.startswith("_refresh-"):
            inst["status"] = "done" if ok else "failed"
            event(state, "ingest", f"{key}: {receipt['status']}")
            if not ok:
                problems.append(f"refresh {key} failed ({receipt['status']}, exit {cmd.get('exit_code')})")
            continue
        if name.startswith("_"):
            phase, hidx = name[1:], inst["host"]
            h = state["hosts"][hidx]
            host = next(c for c in spec["hosts"]["candidates"] if c["id"] == h["candidate"])
            if phase == "provision":
                if ok:
                    try:
                        capture(host["lifecycle"]["provision"].get("capture"), stdout, state)
                        h.update(state="up", handle=state["vars"]["handle"])
                    except SpecError as exc:
                        ok = False
                        problems.append(f"provision of {h['candidate']}: {exc}")
                if not ok:
                    h.update(state="unavailable", ended_at=now())
                    problems.append(f"host_unavailable: provision of {h['candidate']} failed "
                                    f"({receipt['status']}, exit {cmd.get('exit_code')})")
            elif phase == "poll":
                h["last_poll_at"] = now()
                if not ok:
                    problems.append(f"poll of {h['candidate']} failed (exit {cmd.get('exit_code')}); "
                                    "the host may be gone")
            elif phase == "teardown" and ok:
                h.update(state="torn_down", ended_at=now())
            elif phase == "teardown":
                problems.append(f"teardown of {h['candidate']} failed: terminate it by hand "
                                f"(handle {h.get('handle')})")
            inst["status"] = "done" if ok else "failed"
            event(state, "ingest", f"{key}: {receipt['status']}")
            continue
        step = next(s for s in spec["steps"] if s["id"] == name)
        code = cmd.get("exit_code")
        if ok:
            try:
                capture(step.get("capture"), stdout, state)
            except SpecError as exc:
                ok = False
                problems.append(f"{key}: {exc}")
        if ok:
            inst["status"] = "done"
        elif cmd.get("timed_out") or code in (step.get("censored_exit_codes") or []):
            inst["status"], inst["reason"] = "censored", f"exit {code}, timed_out {cmd.get('timed_out')}"
        else:
            inst["status"], inst["reason"] = "failed", f"{receipt['status']}, exit {code}"
        event(state, "ingest", f"{key}: {inst['status']} ({receipt['status']})")
        if inst["status"] == "failed":
            problems.append(apply_on_failure(spec, state, step, inst, key))
        elif inst["status"] == "censored" and "step_censored" in (spec["agent"].get("escalate_on") or []):
            problems.append(f"step_censored: {key} ({inst['reason']})")
    return [p for p in problems if p]


def all_checks(gate, ctx, paths) -> tuple[bool, str]:
    details = []
    for check in gate.get("checks") or []:
        ok, detail = run_check(check, ctx, paths)
        details.append(("ok " if ok else "FAIL ") + detail)
        if not ok:
            return False, "; ".join(details)
    return True, "; ".join(details)


def mark_cell(state, cell_id: str, status: str, reason: str) -> None:
    for inst in state["instances"].values():
        if inst["cell"] == cell_id and inst["status"] not in TERMINAL:
            inst["status"], inst["reason"] = status, reason


def apply_fail(spec, state, gate, inst, text) -> str:
    mode = gate["on_fail"]
    if mode in ("skip_cell", "void_cell") and inst.get("cell"):
        mark_cell(state, inst["cell"], "not_attempted" if mode == "skip_cell" else "void", text)
        inst["status"] = "not_attempted" if mode == "skip_cell" else "void"
        event(state, mode, text)
        return ""
    if mode == "stop":
        state["status"], state["blocked_on"] = "stopped", text
        return f"stopped: {text}"
    return f"gate_failed: {text}"


def apply_on_failure(spec, state, step, inst, key) -> str:
    mode = step.get("on_failure", "escalate")
    text = f"step_failed: {key} ({inst['reason']})"
    if mode == "skip_cell" and inst.get("cell"):
        mark_cell(state, inst["cell"], "not_attempted", text)
        return ""
    if mode == "continue":
        return ""
    if mode == "stop":
        state["status"], state["blocked_on"] = "stopped", text
        return f"stopped: {text}"
    return text


# ---------------------------------------------------------------------------
# next
# ---------------------------------------------------------------------------

def act(kind: str, **fields) -> dict[str, Any]:
    return {"action": kind, **fields}


def ensure_instances(spec, state, cells) -> None:
    hidx = host_index(state)
    for s in spec["steps"]:
        if s["scope"] == "once":
            keys = [(instance_key(s["id"]), None, None)]
        elif s["scope"] == "per_cell":
            keys = [(instance_key(s["id"], c["id"]), c["id"], None) for c in cells_for_step(s, cells)]
        else:
            keys = [(instance_key(s["id"], host=hidx), None, hidx)] if hidx is not None else []
        for key, cell, host in keys:
            state["instances"].setdefault(key, {"step": s["id"], "cell": cell, "host": host,
                                                "status": "pending", "reason": None, "attempts": []})


def needs_met(spec, state, step, cell_id, hidx) -> bool:
    by_id = {s["id"]: s for s in spec["steps"]}
    for n in step.get("needs") or []:
        dep = by_id[n]
        if dep["scope"] == "per_cell":
            if cell_id is not None:
                keys = [instance_key(n, cell_id)]
            else:
                keys = [k for k, i in state["instances"].items() if i["step"] == n]
                if any(state["instances"][k]["status"] not in TERMINAL for k in keys):
                    return False
                continue
        elif dep["scope"] == "per_host":
            keys = [instance_key(n, host=hidx)]
        else:
            keys = [instance_key(n)]
        for k in keys:
            if state["instances"].get(k, {}).get("status") not in OK_DONE:
                return False
    return True


def permission(spec, name: str) -> str:
    return (spec["agent"]["autonomy"] or {}).get(name, "ask")


def human_gate(state, gid: str, question: str) -> dict[str, Any] | None:
    """None when answered yes. An ask action otherwise (or a stop on no)."""
    g = state["gates"].get(gid)
    if g and g["result"] == "answered_yes":
        return None
    if g and g["result"] == "answered_no":
        return act("stopped", reason=f"owner answered no to {gid}")
    state["gates"][gid] = {"result": "asked", "at": now(), "detail": question}
    if state["status"] != "stopped":
        state["status"] = "blocked"
    state["blocked_on"] = gid
    return act("ask", gate=gid, question=question,
               record=f"python3 tools/expspec/expspec.py answer <spec> --gate {gid} --yes|--no --by <owner>")


def evaluate_gates(spec, spec_path, state, paths, key, inst, s, ctx, side):
    """Use the same human, polling, and failure rules on either side of a step."""
    for gid in (s.get("gates") or {}).get(side, []):
        gate = next(g for g in spec["gates"] if g["id"] == gid)
        gkey = f"{gid}:{key}"
        if gate["kind"] == "human":
            a = human_gate(state, gkey, render(gate["ask"], ctx))
            if a:
                return a
            continue
        prior = state["gates"].get(gkey, {})
        if prior.get("result") == "pass":
            continue
        passed, detail = all_checks(gate, ctx, paths)
        first = prior.get("first_at") or now()
        if passed:
            state["gates"][gkey] = {"result": "pass", "first_at": first, "at": now(), "detail": detail}
            event(state, "gate", f"{gkey} pass")
            continue
        if gate["kind"] == "wait_until":
            waited = (dt.datetime.now(dt.timezone.utc) - parse_ts(first)).total_seconds()
            if waited < gate["max_wait_s"]:
                state["gates"][gkey] = {"result": "waiting", "first_at": first, "at": now(),
                                        "detail": detail}
                if gate.get("refresh"):
                    rkey = f"_refresh-{gid}"
                    last = state["instances"].get(rkey, {}).get("attempts", [{}])[-1].get("carded_at")
                    if not last or (dt.datetime.now(dt.timezone.utc) - parse_ts(last)).total_seconds() \
                            >= gate["every_s"]:
                        state["instances"].setdefault(rkey, {"step": rkey, "cell": None, "host": None,
                                                             "status": "pending", "reason": None,
                                                             "attempts": []})
                        card = write_card(spec, spec_path, state, paths, rkey,
                                          {"argv": render(gate["refresh"], ctx), "timeout_seconds": 600,
                                           "success_exit_codes": [0], "outputs": [],
                                           "must_not_exist": [], "background": False}, [])
                        state["instances"][rkey]["status"] = "carded"
                        return act("run_card", key=rkey, card=card,
                                   relay=f"python3 tools/task_card.py prompt {card}",
                                   background=False, assess=False)
                state["status"] = "waiting"
                return act("wait", until=_after(gate["every_s"]), reason=f"gate {gid}: {detail}")
        state["gates"][gkey] = {"result": "fail", "first_at": first, "at": now(), "detail": detail}
        msg = apply_fail(spec, state, gate, inst, f"gate {gid} {side} {key}: {detail}")
        if msg.startswith("stopped"):
            return teardown_or(spec, spec_path, state, paths, act("stopped", reason=msg))
        if msg:
            a = human_gate(state, f"esc-{gkey}", f"ESCALATION: {msg}. Answer yes to skip this "
                                                 "instance and continue, no to stop.")
            if a:
                a["action"] = "escalate" if a["action"] == "ask" else a["action"]
                return a
            inst["status"], inst["reason"] = "not_attempted", msg
        break
    return None


def next_action(spec, spec_path, state) -> dict[str, Any]:
    result = _next_action(spec, spec_path, state)
    if result["action"] == "stopped":
        state["status"], state["blocked_on"] = "stopped", result.get("reason")
        return teardown_or(spec, spec_path, state, resolve_paths(spec), result)
    return result


def _next_action(spec, spec_path, state) -> dict[str, Any]:
    paths = resolve_paths(spec)
    cells = expand_cells(spec)
    cells_by_id = {c["id"]: c for c in cells}
    cands = spec["hosts"]["candidates"]
    problems = ingest_pending(spec, spec_path, state, paths, cells_by_id)
    if state["status"] == "stopped":
        return act("stopped", reason=state.get("blocked_on"))
    for p in problems:
        gid = "esc-" + hashlib.sha256(p.encode()).hexdigest()[:10]
        a = human_gate(state, gid, f"ESCALATION: {p}. Answer yes to continue past it, no to stop the experiment.")
        if a is not None:
            a["action"] = "escalate" if a["action"] == "ask" else a["action"]
            a["reason"] = p
            return a

    # Complete after-gates before scheduling any dependent work. Receipts
    # establish command completion; they do not answer human gates.
    for key, inst in list(state["instances"].items()):
        if inst["status"] not in OK_DONE or inst["step"].startswith("_"):
            continue
        step = next(s for s in spec["steps"] if s["id"] == inst["step"])
        hidx = inst["host"]
        host = (next(c for c in cands if c["id"] == state["hosts"][hidx]["candidate"])
                if hidx is not None else None)
        ctx = context(spec, paths, cells_by_id.get(inst["cell"]), host, hidx, state["vars"])
        gate_action = evaluate_gates(spec, spec_path, state, paths, key, inst, step, ctx, "after")
        if gate_action is not None:
            return gate_action

    for key, inst in state["instances"].items():
        if inst["status"] == "carded":
            att = inst["attempts"][-1]
            return act("run_card", key=key, card=att["card"],
                       relay=f"python3 tools/task_card.py prompt {att['card']}",
                       background=bool(yaml.safe_load((REPO / att["card"]).read_text())["background"]),
                       assess=_is_evidence(spec, inst))

    # Budget.
    used, cap = spend(spec, state), spec["budget"]["max_cost_usd"]
    if used > cap or (cap > 0 and used >= cap):
        state["status"], state["blocked_on"] = "stopped", f"budget cap reached: ${used} of ${cap}"
        return teardown_or(spec, spec_path, state, paths, act("stopped", reason=state["blocked_on"]))
    frac = spec["budget"].get("escalate_at_fraction", 0.8)
    if cap > 0 and used >= frac * cap:
        a = human_gate(state, f"budget-{int(frac * 100)}",
                       f"Spend estimate ${used} has passed {int(frac * 100)}% of the ${cap} budget. Continue.")
        if a:
            return a
    started = [parse_ts(h["started_at"]) for h in state["hosts"] if h.get("started_at")]
    if started:
        hours = (dt.datetime.now(dt.timezone.utc) - min(started)).total_seconds() / 3600
        if hours >= spec["budget"]["max_wall_hours"]:
            state["status"], state["blocked_on"] = "stopped", f"max_wall_hours reached ({hours:.1f} h)"
            return teardown_or(spec, spec_path, state, paths, act("stopped", reason=state["blocked_on"]))

    hidx = host_index(state)
    host = next(c for c in cands if c["id"] == state["hosts"][hidx]["candidate"]) if hidx is not None else None
    ensure_instances(spec, state, cells)

    # Cells that no remaining candidate can ever run.
    unavailable = {h["candidate"] for h in state["hosts"] if h["state"] == "unavailable"}
    no_paid = permission(spec, "launch_paid_host") == "never"
    usable = [c for c in cands if c["id"] not in unavailable
              and not (no_paid and c.get("paid", True) and c["backend"] != "local")]
    for c in cells:
        if not any(fits(h["provides"], c.get("requires")) and (not h.get("cells") or c["id"] in h["cells"])
                   for h in usable):
            mark_cell(state, c["id"], "not_attempted", "no available host candidate fits this cell")

    def cell_ok_here(cell_id):
        if cell_id is None or host is None:
            return True
        c = cells_by_id[cell_id]
        return fits(host["provides"], c.get("requires")) and (not host.get("cells") or cell_id in host["cells"])

    remote_running = False
    for s in spec["steps"]:
        insts = [(k, i) for k, i in state["instances"].items() if i["step"] == s["id"]]
        if s["scope"] == "per_cell":
            order = {c["id"]: n for n, c in enumerate(cells_for_step(s, cells))}
            insts.sort(key=lambda ki: order.get(ki[1]["cell"], 0))
        if s["scope"] == "per_host":
            insts = [(k, i) for k, i in insts if i["host"] == hidx]
        for key, inst in insts:
            if inst["status"] == "running_remote":
                remote_running = True
                continue
            if inst["status"] != "pending":
                continue
            if s["runs_on"] == "host" and (host is None or not cell_ok_here(inst["cell"])):
                continue
            if not needs_met(spec, state, s, inst["cell"], hidx):
                continue
            cell = cells_by_id.get(inst["cell"]) if inst["cell"] else None
            ctx = context(spec, paths, cell, host, hidx, state["vars"])
            gate_action = evaluate_gates(spec, spec_path, state, paths, key, inst, s, ctx, "before")
            if gate_action is not None:
                return gate_action
            if inst["status"] != "pending":
                continue
            if s.get("done_when") and all(run_check(c, ctx, paths)[0] for c in s["done_when"]):
                inst["status"], inst["reason"] = "skipped_done", "done_when passed"
                event(state, "skip", f"{key}: done_when passed")
                continue
            if s["runs_on"] == "host" and host["backend"] == "batch":
                inst["status"] = "running_remote"
                remote_running = True
                continue
            cmd, extra = step_command(spec, s, ctx, host)
            card = write_card(spec, spec_path, state, paths, key, cmd, extra)
            inst["host"] = hidx
            state["status"], state["blocked_on"] = "running", None
            return act("run_card", key=key, card=card, relay=f"python3 tools/task_card.py prompt {card}",
                       background=cmd["background"], assess=bool(s.get("evidence")))

    # Batch host bookkeeping: STATUS tokens decide remote instances.
    if host is not None and host["backend"] == "batch":
        hstate = state["hosts"][hidx]
        if remote_running:
            status_file = REPO / paths["collected"] / "STATUS"
            lines = status_file.read_text().splitlines() if status_file.is_file() else []
            for key, inst in state["instances"].items():
                if inst["status"] != "running_remote":
                    continue
                s = next(x for x in spec["steps"] if x["id"] == inst["step"])
                ctx = context(spec, paths, cells_by_id.get(inst["cell"]) if inst["cell"] else None,
                              host, hidx, state["vars"])
                if any(line.rstrip().endswith(render(s["status_done"], ctx)) for line in lines):
                    inst["status"] = "done"
                elif any(line.rstrip().endswith(render(t, ctx)) for t in s.get("status_failed") or []
                         for line in lines):
                    inst["status"], inst["reason"] = "failed", "STATUS reports failure"
            if any(i["status"] == "running_remote" for i in state["instances"].values()):
                every = host["lifecycle"].get("poll_every_s", 1800)
                last = hstate.get("last_poll_at") or hstate.get("started_at")
                if last and (dt.datetime.now(dt.timezone.utc) - parse_ts(last)).total_seconds() < every:
                    state["status"] = "waiting"
                    return act("wait", until=_after(every, last), reason=f"remote steps running on {host['id']}")
                return lifecycle_card(spec, spec_path, state, paths, hidx, host, "poll")
            return next_action(spec, spec_path, state)  # statuses changed; re-plan
        pending_here = [i for i in state["instances"].values()
                        if i["status"] == "pending" and _runs_on_host(spec, i) and cell_ok_here(i["cell"])]
        if not pending_here:
            if state["instances"].get(f"_collect#{hidx}", {}).get("status") not in OK_DONE | {"failed"}:
                return lifecycle_card(spec, spec_path, state, paths, hidx, host, "collect")
            return teardown_or(spec, spec_path, state, paths, None) or next_action(spec, spec_path, state)
    elif host is not None:
        pending_here = [i for i in state["instances"].values()
                        if i["status"] == "pending" and _runs_on_host(spec, i) and cell_ok_here(i["cell"])]
        if not pending_here:
            state["hosts"][hidx].update(state="torn_down", ended_at=now())
            event(state, "host", f"released {host['id']}")
            return next_action(spec, spec_path, state)

    # Need a host?
    waiting_host = [i for i in state["instances"].values() if i["status"] == "pending" and _runs_on_host(spec, i)]
    if host is None and waiting_host:
        cand = choose_host(spec, state, cells_by_id, waiting_host)
        if cand is None:
            for i in waiting_host:
                i["status"], i["reason"] = "not_attempted", "no host candidate available"
            return next_action(spec, spec_path, state)
        if cand.get("paid", True) and cand["backend"] != "local":
            perm = permission(spec, "launch_paid_host")
            limit = spec["agent"]["autonomy"].get("spend_without_asking_usd", 0)
            projected = cand.get("est_hourly_usd", 0) * ((cand.get("lifecycle") or {}).get("lifetime_s", 0) / 3600)
            if perm == "ask" or projected > limit:
                a = human_gate(state, f"launch-{cand['id']}-{len(state['hosts'])}",
                               f"Launch host {cand['id']} ({cand.get('note', cand['backend'])}) at about "
                               f"${cand.get('est_hourly_usd', 0)}/h, lifetime cap "
                               f"{(cand.get('lifecycle') or {}).get('lifetime_s', 0) // 3600} h "
                               f"(up to ${projected:.2f}). Spent so far ${used} of ${cap}.")
                if a:
                    return a
        state["hosts"].append({"candidate": cand["id"], "state": "provisioning", "handle": None,
                               "hourly_usd": cand.get("est_hourly_usd", 0), "started_at": now(),
                               "ended_at": None, "last_poll_at": None})
        hidx = len(state["hosts"]) - 1
        event(state, "host", f"selected {cand['id']}")
        if cand["backend"] in ("local", "ssh"):
            state["hosts"][hidx]["state"] = "up"
            return next_action(spec, spec_path, state)
        return lifecycle_card(spec, spec_path, state, paths, hidx, cand, "provision")

    if all(i["status"] in TERMINAL for i in state["instances"].values()):
        missing = missing_artifacts(spec, state, paths, cells_by_id)
        if missing and state["status"] != "completed":
            a = human_gate(state, "esc-missing-artifacts",
                           f"ESCALATION: required artifacts missing: {', '.join(missing[:8])}. "
                           "Answer yes to mark the run completed anyway, no to stop.")
            if a:
                a["action"] = "escalate" if a["action"] == "ask" else a["action"]
                return a
        state["status"], state["blocked_on"] = "completed", None
        return act("done", summary=summary(state), next="Reasoning tier: python3 tools/expspec/expspec.py facts "
                   "<spec>, then apply outcome.decision_rules and write reporting.results_file.")
    blocked = [k for k, i in state["instances"].items() if i["status"] == "pending"]
    return act("escalate", reason=f"no runnable instance; pending with unmet needs: {blocked[:10]}")


def _is_evidence(spec, inst) -> bool:
    s = next((x for x in spec["steps"] if x["id"] == inst["step"]), None)
    return bool(s and s.get("evidence"))


def _runs_on_host(spec, inst) -> bool:
    s = next((x for x in spec["steps"] if x["id"] == inst["step"]), None)
    return bool(s and (s["runs_on"] == "host" or s["scope"] == "per_host"))


def _after(seconds: int, base: str | None = None) -> str:
    start = parse_ts(base) if base else dt.datetime.now(dt.timezone.utc)
    return (start + dt.timedelta(seconds=seconds)).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def choose_host(spec, state, cells_by_id, waiting) -> dict[str, Any] | None:
    bad = {h["candidate"] for h in state["hosts"] if h["state"] == "unavailable"}
    want = [cells_by_id[i["cell"]] for i in waiting if i["cell"]]
    first = sorted(want, key=lambda c: -c["priority"])[0] if want else None
    ok = [c for c in spec["hosts"]["candidates"] if c["id"] not in bad
          and (first is None or (fits(c["provides"], first.get("requires"))
                                 and (not c.get("cells") or first["id"] in c["cells"])))
          and not (c.get("paid", True) and c["backend"] != "local"
                   and permission(spec, "launch_paid_host") == "never")]
    if not ok:
        return None
    if spec["hosts"]["selection"] == "cheapest_fit":
        ok.sort(key=lambda c: c.get("est_hourly_usd", 0))
    return ok[0]


def lifecycle_card(spec, spec_path, state, paths, hidx, host, phase) -> dict[str, Any]:
    key = f"_{phase}#{hidx}"
    if phase == "poll":
        key = f"_poll#{hidx}.{sum(1 for k in state['instances'] if k.startswith(f'_poll#{hidx}.')) + 1}"
    state["instances"][key] = {"step": f"_{phase}", "cell": None, "host": hidx, "status": "pending",
                               "reason": None, "attempts": []}
    ctx = context(spec, paths, None, host, hidx, state["vars"])
    card = write_card(spec, spec_path, state, paths, key, lifecycle_command(spec, host, phase, ctx), [])
    if state["status"] != "stopped":
        state["status"] = "running"
    return act("run_card", key=key, card=card, relay=f"python3 tools/task_card.py prompt {card}",
               background=host["lifecycle"][phase]["timeout_s"] > 600, assess=False)


def teardown_or(spec, spec_path, state, paths, fallback):
    """A teardown card for the first host still up, else `fallback`."""
    for hidx, h in enumerate(state["hosts"]):
        if h["state"] not in ("up", "provisioning"):
            continue
        host = next(c for c in spec["hosts"]["candidates"] if c["id"] == h["candidate"])
        if host["backend"] != "batch":
            h.update(state="torn_down", ended_at=now())
            continue
        if permission(spec, "terminate_own_hosts") == "never":
            return act("escalate", reason=f"host {h['candidate']} (handle {h.get('handle')}) needs teardown "
                       "and terminate_own_hosts is never")
        if permission(spec, "terminate_own_hosts") == "ask":
            answer = human_gate(state, f"teardown-{hidx}",
                                f"Terminate own host {h['candidate']} (handle {h.get('handle')}).")
            if answer is not None:
                if answer["action"] == "stopped":
                    return act("escalate", reason=f"teardown declined for {h['candidate']}; "
                               f"handle {h.get('handle')} remains active")
                return answer
        key = f"_teardown#{hidx}"
        previous = state["instances"].get(key, {})
        if previous.get("status") == "carded":
            card = previous["attempts"][-1]["card"]
            return act("run_card", key=key, card=card,
                       relay=f"python3 tools/task_card.py prompt {card}",
                       background=host["lifecycle"]["teardown"]["timeout_s"] > 600, assess=False)
        if previous.get("status") == "failed":
            return act("escalate", reason=f"teardown of {h['candidate']} failed; "
                       f"terminate handle {h.get('handle')} by hand")
        return lifecycle_card(spec, spec_path, state, paths, hidx, host, "teardown")
    return fallback


def missing_artifacts(spec, state, paths, cells_by_id) -> list[str]:
    attempted = {i["cell"] for i in state["instances"].values()
                 if i["cell"] and i["status"] in OK_DONE}
    out = []
    for art in spec["reporting"]["required_artifacts"]:
        targets = [cells_by_id[c] for c in sorted(attempted)] if "{{cell." in art.replace(" ", "") else [None]
        for cell in targets:
            p = render(art, context(spec, paths, cell, None, None, state["vars"]))
            if not (REPO / p).exists():
                out.append(p)
    return out


def summary(state) -> dict[str, int]:
    out: dict[str, int] = {}
    for i in state["instances"].values():
        if not i["step"].startswith("_"):
            out[i["status"]] = out.get(i["status"], 0) + 1
    return out


# ---------------------------------------------------------------------------
# Commands
# ---------------------------------------------------------------------------

def cmd_validate(args) -> int:
    bad = 0
    for p in args.spec:
        spec, errors, warnings = validate(Path(p).resolve())
        for w in warnings:
            print(f"{p}: warning: {w}")
        for e in errors:
            print(f"{p}: error: {e}")
        if errors:
            bad = 1
        else:
            print(f"{p}: OK ({spec['id']} v{spec['version']}, {spec['status']}, sha256 {sha256_file(Path(p))})")
    return bad


def cmd_plan(args) -> int:
    path = Path(args.spec).resolve()
    spec, errors, _ = validate(path)
    if errors:
        print("\n".join(errors))
        return 1
    paths = resolve_paths(spec)
    cells = expand_cells(spec)
    host = spec["hosts"]["candidates"][0]
    print(f"# {spec['id']} v{spec['version']} ({spec['status']}): {len(cells)} cells, {len(spec['steps'])} steps")
    for c in cells:
        print(f"cell {c['id']:<14} role={c['role']:<8} prio={c['priority']:<3} optional={c['optional']} "
              f"params={json.dumps(c['params'])}")
    for s in spec["steps"]:
        targets = cells_for_step(s, cells) if s["scope"] == "per_cell" else [None]
        for c in targets:
            key = instance_key(s["id"], c["id"] if c else None, 0 if s["scope"] == "per_host" else None)
            ctx = context(spec, paths, c, host, 0, {}, probe=True)
            what = shlex.join(render(s["argv"], ctx)) if "argv" in s else f"(remote; STATUS {s.get('status_done')})"
            gates = ",".join((s.get("gates") or {}).get("before", []))
            print(f"{key:<28} {s['runs_on']:<10} {('gates=' + gates) if gates else '':<20} {what}")
    return 0


def cmd_next(args) -> int:
    path = Path(args.spec).resolve()
    spec, errors, _ = validate(path)
    if errors:
        print(json.dumps(act("refused", reason="spec does not validate", errors=errors[:20]), indent=1))
        return 1
    if spec["status"] != "approved":
        print(json.dumps(act("refused", reason=f"spec status is {spec['status']}; only approved specs run")))
        return 1
    try:
        state = load_state(path, spec)
        result = next_action(spec, path, state)
    except SpecError as exc:
        print(json.dumps(act("escalate", reason=str(exc)), indent=1))
        return 1
    result["state"] = state["status"]
    result["spend_usd"] = spend(spec, state)
    event(state, "next", json.dumps({k: v for k, v in result.items() if k in ("action", "key", "gate", "reason")}))
    save_state(path, state)
    print(json.dumps(result, indent=1))
    return 0


def cmd_answer(args) -> int:
    path = Path(args.spec).resolve()
    spec = load_yaml(path)
    state = load_state(path, spec)
    g = state["gates"].get(args.gate)
    if not g or g["result"] != "asked":
        print(f"gate {args.gate} is not waiting for an answer")
        return 1
    g.update(result="answered_yes" if args.yes else "answered_no", at=now(), by=args.by)
    if state.get("blocked_on") == args.gate and state["status"] != "stopped":
        state["status"], state["blocked_on"] = "running", None
    event(state, "answer", f"{args.gate}: {'yes' if args.yes else 'no'} by {args.by}")
    save_state(path, state)
    print(f"recorded {args.gate}: {'yes' if args.yes else 'no'}")
    return 0


def cmd_ls(args) -> int:
    root = Path(args.dir).resolve()
    rows = []
    for p in sorted(root.glob("EXP-*.yaml")):
        if p.name.endswith(".state.yaml"):
            continue
        spec = load_yaml(p) or {}
        sp = state_path(p)
        st = load_yaml(sp) if sp.exists() else None
        done = total = 0
        if st:
            real = [i for i in st["instances"].values() if not i["step"].startswith("_")]
            total, done = len(real), sum(1 for i in real if i["status"] in TERMINAL)
        cost = spend(spec, st) if st and spec.get("hosts") else 0
        rows.append((spec.get("id", p.stem), str(spec.get("version", "")), spec.get("status", "?"),
                     st["status"] if st else "-", f"{done}/{total}" if st else "-",
                     f"${cost:g}/${(spec.get('budget') or {}).get('max_cost_usd', '?')}",
                     (st or {}).get("blocked_on") or "", spec.get("title", "")[:60]))
    head = ("id", "v", "spec", "run", "steps", "spend", "blocked_on", "title")
    widths = [max(len(str(r[i])) for r in rows + [head]) for i in range(len(head))]
    for r in [head] + rows:
        print("  ".join(str(x).ljust(w) for x, w in zip(r, widths)).rstrip())
    return 0


def cmd_facts(args) -> int:
    path = Path(args.spec).resolve()
    spec = load_yaml(path)
    paths = resolve_paths(spec)
    cells = expand_cells(spec)
    st = load_yaml(state_path(path)) if state_path(path).exists() else {"instances": {}, "vars": {}, "hosts": []}
    timing_ok = any(c["timing_valid"] for c in spec["hosts"]["candidates"]
                    if any(h["candidate"] == c["id"] for h in st["hosts"]))
    out: dict[str, Any] = {"experiment": spec["id"], "version": spec["version"],
                           "instances": {k: {"status": v["status"], "reason": v.get("reason")}
                                         for k, v in st["instances"].items() if not v["step"].startswith("_")},
                           "metrics": {}, "decision_rule_checks": {},
                           "missing_artifacts": missing_artifacts(spec, st, paths, {c["id"]: c for c in cells})}
    for m in spec.get("metrics") or []:
        if not m.get("source"):
            continue
        per = {}
        for c in (cells if "{{cell." in m["source"]["file"].replace(" ", "") else [None]):
            ctx = context(spec, paths, c, None, None, st["vars"])
            try:
                per[c["id"] if c else "-"] = json_value(m["source"], ctx)
            except (OSError, KeyError, IndexError, ValueError, TypeError):
                per[c["id"] if c else "-"] = None
        out["metrics"][m["id"]] = {"kind": m["kind"], "unit": m.get("unit"), "values": per,
                                   **({"label": "descriptive (no timing_valid host ran)"}
                                      if m["kind"] == "timing" and not timing_ok else {})}
    for rule in spec["outcome"]["decision_rules"]:
        if not rule.get("checks"):
            continue
        res = {}
        for cid in rule.get("cells") or [None]:
            ctx = context(spec, paths, {c["id"]: c for c in cells}.get(cid), None, None, st["vars"])
            res[cid or "-"] = [run_check(ch, ctx, paths) for ch in rule["checks"]]
        out["decision_rule_checks"][rule["id"]] = res
    print(json.dumps(out, indent=1, default=str))
    return 0


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    v = sub.add_parser("validate"); v.add_argument("spec", nargs="+")
    for name in ("plan", "next", "facts"):
        sub.add_parser(name).add_argument("spec")
    a = sub.add_parser("answer"); a.add_argument("spec"); a.add_argument("--gate", required=True)
    g = a.add_mutually_exclusive_group(required=True); g.add_argument("--yes", action="store_true")
    g.add_argument("--no", action="store_true"); a.add_argument("--by", required=True)
    ls = sub.add_parser("ls"); ls.add_argument("dir", nargs="?", default=str(REPO / "experiments" / "specs"))
    args = ap.parse_args(argv)
    try:
        return {"validate": cmd_validate, "plan": cmd_plan, "next": cmd_next, "answer": cmd_answer,
                "ls": cmd_ls, "facts": cmd_facts}[args.cmd](args)
    except SpecError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
