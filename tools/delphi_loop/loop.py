#!/usr/bin/env python3
"""delphi-loop: a continuous learning loop for the task-card lane, with LangSmith.

`tools/task_card.py lane` sends the director's two calls (card-compile and
assess) to Delphi and leaves one record per lane in `<card>.lane/lane.json`
next to the prompts and replies. Every reply passes through `ingest`, a
mechanical referee: it is accepted, rejected by the tool, refused by the model,
or (compile only) it lists specification errors, which are the design's gap and
not the model's. This tool turns those verdicts into a routing policy and gives
the policy back to the lane:

    harvest   lane records -> episodes (.delphi-loop/episodes.jsonl), one per call
    push      episodes -> LangSmith: a trace per lane, an llm run per call, and the
              ingest verdict as feedback (hashes only unless --include-text)
    pull      LangSmith -> episodes: runs another container pushed, and human review
              feedback (key `review`, score 0/1) from annotation queues
    dataset   accepted (prompt, reply) pairs -> a LangSmith dataset per family
    eval      replay stored prompts on a candidate target through `delphi run`,
              scored by the same ingest; the results become episodes
    fit       episodes -> capability registry (.delphi-loop/registry.json)
    route     `delphi route -family F -json`, answered from the registry
    export    episodes -> `delphi.episode.v1` JSONL for `delphi learn` (stdout or -o FILE)

`tools/delphi_loop/delphi` is a drop-in `$DELPHI`: it answers `route` here and
passes every other subcommand to the real delphi binary, so
`DELPHI=tools/delphi_loop/delphi python3 tools/task_card.py lane ... --route`
works without a calibrated Delphi registry.

Scores are mechanical (the ingest verdict) or explicit human review; no model
judges another model. Nothing leaves the machine unless `push`, `pull`, `dataset`
or `eval` is run, and prompt or reply text goes to LangSmith only with
--include-text (a design can be unpublished research: AGENTS.md "Model tiers").
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
import random
import subprocess
import sys
import time
import uuid
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any, Iterable

import yaml

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
sys.path.insert(0, str(HERE.parent))
import task_card as tc  # noqa: E402  (the canonical lane; imported, never edited)

EPISODE_SCHEMA = "crypto.delphi-loop.episode.v1"
REGISTRY_SCHEMA = "crypto.delphi-loop.registry.v1"
NAMESPACE = uuid.UUID("5d1e1a4b-7c0e-4b8e-9a59-d31f1c0f0b11")   # uuid5 namespace for stable ids
STAGE_FAMILY = dict(tc.FAMILY)                                    # compile -> card-compile, assess -> assess
LABELS = ("accepted", "rejected", "refused", "spec_error")
BATCH_DISCOUNT = 0.5                                              # Delphi runs on batch APIs: half list price
REVIEW_KEY = "review"
DELPHI_EPISODE_SCHEMA = "delphi.episode.v1"
DELPHI_USAGE_KEYS = ("input_tokens", "output_tokens", "cache_write_tokens", "cache_read_tokens")


class LoopError(Exception):
    pass


# ---------------------------------------------------------------------------
# Paths, config, stores
# ---------------------------------------------------------------------------

def home(repo: Path = REPO) -> Path:
    return Path(os.environ.get("DELPHI_LOOP_HOME") or repo / ".delphi-loop")


def load_config(path: Path | None = None) -> dict[str, Any]:
    path = path or Path(os.environ.get("DELPHI_LOOP_TARGETS") or HERE / "targets.yaml")
    config = yaml.safe_load(path.read_text(encoding="utf-8"))
    for family, spec in config["families"].items():
        names = [t["target"] for t in spec["targets"]]
        if spec["incumbent"] not in names:
            raise LoopError(f"{family}: incumbent {spec['incumbent']} is not among its targets")
    return config


def load_prices() -> dict[str, Any]:
    return json.loads((HERE.parent / "token_prices.json").read_text(encoding="utf-8"))["models"]


def read_episodes(repo: Path = REPO) -> dict[str, dict]:
    path = home(repo) / "episodes.jsonl"
    out: dict[str, dict] = {}
    if path.is_file():
        for line in path.read_text(encoding="utf-8").splitlines():
            if line.strip():
                row = json.loads(line)
                out[row["id"]] = row
    return out


def write_episodes(episodes: dict[str, dict], repo: Path = REPO) -> Path:
    path = home(repo) / "episodes.jsonl"
    path.parent.mkdir(parents=True, exist_ok=True)
    rows = sorted(episodes.values(), key=lambda r: (r["group"], r["call"]))
    tmp = path.with_suffix(".tmp")
    tmp.write_text("".join(json.dumps(r, sort_keys=True) + "\n" for r in rows), encoding="utf-8")
    tmp.replace(path)
    return path


def delphi_row(row: dict) -> dict:
    """One loop episode as a `delphi.episode.v1` row (review and usage only when present)."""
    out: dict[str, Any] = {"schema": DELPHI_EPISODE_SCHEMA, "id": row["id"], "family": row["family"],
                           "target": row["target"], "label": row["label"]}
    if row.get("review") is not None:
        out["review"] = float(row["review"])
    usage = {k: row["usage"][k] for k in DELPHI_USAGE_KEYS if (row.get("usage") or {}).get(k)}
    if usage:
        out["usage"] = usage
    out.update({k: row[k] for k in ("group", "call", "source", "observed_at")})
    return out


def export_episodes(repo: Path = REPO, out: Path | None = None,
                    say=lambda m: print(m, file=sys.stderr)) -> int:
    rows = sorted(read_episodes(repo).values(), key=lambda r: (r["group"], r["call"]))
    text = "".join(json.dumps(delphi_row(r), sort_keys=True) + "\n" for r in rows)
    if out is None:
        sys.stdout.write(text)
    else:
        tmp = out.with_name(out.name + ".tmp")
        tmp.write_text(text, encoding="utf-8")
        tmp.replace(out)
    say(f"EXPORT {len(rows)} episodes -> {out if out else 'stdout'}")
    return 0


def _state(repo: Path, name: str) -> dict[str, Any]:
    path = home(repo) / name
    return json.loads(path.read_text(encoding="utf-8")) if path.is_file() else {}


def _save_state(repo: Path, name: str, data: dict[str, Any]) -> None:
    path = home(repo) / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def show(repo: Path, path: Path) -> str:
    """A path for messages: repository-relative when it is inside, else absolute."""
    try:
        return path.resolve().relative_to(repo.resolve()).as_posix()
    except ValueError:
        return str(path)


def stable_id(*parts: str) -> str:
    return str(uuid.uuid5(NAMESPACE, "\x1f".join(parts)))


# ---------------------------------------------------------------------------
# Harvest: lane records -> episodes
# ---------------------------------------------------------------------------

def target_name(target: dict | None, backend: str) -> str:
    """provider:model@effort, the same spelling `delphi route` uses."""
    if target is None:
        if backend == "conductor":
            return f"conductor:{tc.DIRECTOR_PROFILE}"
        target = {"provider": "anthropic", "model": tc.DIRECTOR_MODEL, "effort": "high"}
    name = f"{target['provider']}:{target['model']}"
    return name + (f"@{target['effort']}" if target.get("effort") else "")


def _spec_errors(reply: str) -> list:
    blocks = [b for b in tc._blocks(reply) if isinstance(b, dict) and "specification_errors" in b]
    return (blocks[0]["specification_errors"] or []) if blocks else []


def label_call(stage: str, call: dict, reply: str | None) -> str:
    if "refused" in call:
        return "refused"
    if call.get("accepted"):
        return "accepted"
    if stage == "compile" and reply is not None and _spec_errors(reply):
        return "spec_error"     # the design's gap; ingest stops escalating on it, and so does scoring
    return "rejected"


def lane_episodes(lane_json: Path, repo: Path = REPO) -> list[dict]:
    record = json.loads(lane_json.read_text(encoding="utf-8"))
    workdir = lane_json.parent
    group = show(repo, workdir)
    backend = record.get("backend", "delphi")
    rows = []
    for name, call in sorted(record.get("calls", {}).items()):
        stage = name.split("-escalation-")[0]
        if stage not in STAGE_FAMILY:
            continue
        attempt = int(name.rsplit("-", 1)[1]) if "-escalation-" in name else 0
        reply_path = workdir / f"{name}.reply.md"
        reply = reply_path.read_text(encoding="utf-8") if reply_path.is_file() else None
        route = (record.get("routes") or {}).get(stage) or {}
        rows.append({
            "schema": EPISODE_SCHEMA,
            "id": stable_id(group, name, call.get("reply_sha256") or call.get("refused", "")),
            "source": "eval" if record.get("eval_of") else "lane",
            "eval_of": record.get("eval_of"),
            "group": group,
            "call": name,
            "stage": stage,
            "family": STAGE_FAMILY[stage],
            "attempt": attempt,
            "backend": backend,
            "free": record.get("free"),
            "target": target_name(call.get("target"), backend),
            "label": label_call(stage, call, reply),
            "prompt_sha256": call.get("prompt_sha256"),
            "reply_sha256": call.get("reply_sha256"),
            "prompt_file": show(repo, workdir / f"{stage}.prompt.md"),
            "reply_file": show(repo, reply_path) if reply is not None else None,
            "card": record.get("card"),
            "usage": call.get("usage"),
            "stop_reason": call.get("stop_reason"),
            "truncated": call.get("truncated"),
            "batch_id": call.get("batch_id"),
            "route_basis": route.get("basis"),
            "lane_outcome": record.get("outcome"),
            "observed_at": datetime.fromtimestamp(lane_json.stat().st_mtime, timezone.utc).isoformat(),
        })
    return rows


def find_lanes(repo: Path = REPO, roots: Iterable[str] = ()) -> list[Path]:
    if roots:
        return sorted(p for r in roots for p in Path(r).resolve().rglob("lane.json") if p.parent.name.endswith(".lane"))
    proc = subprocess.run(["git", "-C", str(repo), "ls-files", "-co", "--exclude-standard", "*.lane/lane.json"],
                          capture_output=True, text=True)
    found = [repo / line for line in proc.stdout.splitlines() if line.strip()]
    if (home(repo) / "eval").is_dir():     # eval records live in the ignored loop home
        found += list((home(repo) / "eval").rglob("lane.json"))
    return sorted(set(found))


def harvest(repo: Path = REPO, roots: Iterable[str] = (), say=print) -> int:
    episodes = read_episodes(repo)
    before = len(episodes)
    for lane_json in find_lanes(repo, roots):
        for row in lane_episodes(lane_json, repo):
            old = episodes.get(row["id"])
            if old and "review" in old:
                row["review"] = old["review"]
            episodes[row["id"]] = row
    write_episodes(episodes, repo)
    say(f"HARVEST {len(episodes)} episodes ({len(episodes) - before} new) -> {show(repo, home(repo) / 'episodes.jsonl')}")
    return 0


# ---------------------------------------------------------------------------
# Fit: episodes -> capability registry
# ---------------------------------------------------------------------------

def effective_label(row: dict) -> str:
    """Human review in LangSmith overrides the mechanical verdict (score 1 accepts, 0 rejects)."""
    review = row.get("review")
    if review is not None and row["label"] in ("accepted", "rejected"):
        return "accepted" if float(review) >= 0.5 else "rejected"
    return row["label"]


def call_cost(row: dict, prices: dict[str, Any]) -> float | None:
    usage, model = row.get("usage") or {}, row["target"].split(":", 1)[-1].split("@")[0]
    if not usage or model not in prices:
        return None
    p = prices[model]
    # Delphi results spell the cache counts cache_read_tokens / cache_write_tokens; the
    # Anthropic API spells them cache_read_input_tokens / cache_creation_input_tokens.
    tokens = {"input": usage.get("input_tokens", 0), "output": usage.get("output_tokens", 0),
              "cache_read": usage.get("cache_read_tokens") or usage.get("cache_read_input_tokens", 0),
              "cache_write_5m": usage.get("cache_write_tokens") or usage.get("cache_creation_input_tokens", 0)}
    return BATCH_DISCOUNT * sum(p.get(k, 0.0) * n for k, n in tokens.items()) / 1e6


def posterior(k: int, n: int, prior: tuple[float, float], z: float) -> tuple[float, float, float, float]:
    a, b = prior[0] + k, prior[1] + n - k
    mean = a / (a + b)
    sd = math.sqrt(a * b / ((a + b) ** 2 * (a + b + 1)))
    return a, b, mean, max(0.0, mean - z * sd)


def fit(episodes: Iterable[dict], config: dict[str, Any], prices: dict[str, Any] | None = None) -> dict[str, Any]:
    prices = prices if prices is not None else load_prices()
    rows = list(episodes)
    digest = hashlib.sha256("".join(sorted(r["id"] + effective_label(r) for r in rows)).encode()).hexdigest()
    registry: dict[str, Any] = {"schema": REGISTRY_SCHEMA, "episodes": len(rows), "episodes_digest": digest,
                                "fitted_at": datetime.now(timezone.utc).isoformat(timespec="seconds"),
                                "families": {}}
    for family, spec in config["families"].items():
        prior = tuple(spec.get("prior", config.get("prior", [1, 1])))
        z = float(spec.get("z", config.get("z", 1.2816)))
        stats = []
        for t in spec["targets"]:
            mine = [r for r in rows if r["family"] == family and r["target"] == t["target"]]
            counts = {lab: sum(effective_label(r) == lab for r in mine) for lab in LABELS}
            n = counts["accepted"] + counts["rejected"] + counts["refused"]
            a, b, mean, lcb = posterior(counts["accepted"], n, prior, z)
            costs = [c for c in (call_cost(r, prices) for r in mine) if c is not None]
            per_call = sum(costs) / len(costs) if costs else None
            stats.append({"target": t["target"], "free": bool(t.get("free")), "n": n, **counts,
                          "alpha": a, "beta": b, "mean": round(mean, 4), "lcb": round(lcb, 4),
                          "usd_per_call": per_call,
                          "usd_per_accept": (per_call / mean) if per_call is not None and mean > 0 else None})
        registry["families"][family] = {
            "incumbent": spec["incumbent"], "threshold": float(spec.get("threshold", config.get("threshold", 0.8))),
            "min_n": int(spec.get("min_n", config.get("min_n", 5))), "z": z, "targets": stats}
    return registry


def save_registry(registry: dict[str, Any], repo: Path = REPO) -> Path:
    path = home(repo) / "registry.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(registry, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return path


# ---------------------------------------------------------------------------
# Route: the `delphi route -family F -json` contract task_card.lane reads
# ---------------------------------------------------------------------------

def route(registry: dict[str, Any], family: str, policy: str = "exploit", free: str | None = None,
          rng: random.Random | None = None) -> dict[str, Any]:
    """Targets are listed cheapest first. exploit: the cheapest target whose acceptance
    lower bound clears the threshold on at least min_n episodes, else the incumbent (no
    unmeasured target takes over). thompson: draw each acceptance from its posterior and
    take the cheapest draw over the threshold (else the best draw), so cheaper targets keep
    being tried while their evidence is thin. The escalation chain is every listed target
    after the choice, so a refused step always moves to a stronger model."""
    fam = registry["families"].get(family)
    if fam is None:
        return {"error": f"family {family!r} is not in the registry ({', '.join(registry['families'])})"}
    targets = [t for t in fam["targets"] if t["free"]] if free else list(fam["targets"])
    if not targets:
        return {"error": f"no free target for {family} (DELPHI_FREE={free})"}
    names = [t["target"] for t in targets]
    if policy == "thompson":
        rng = rng or random.Random()
        draws = [rng.betavariate(t["alpha"], t["beta"]) for t in targets]
        ok = [i for i, d in enumerate(draws) if d >= fam["threshold"]]
        i = ok[0] if ok else max(range(len(targets)), key=draws.__getitem__)
        basis = (f"delphi-loop thompson: draw {draws[i]:.3f} for {names[i]} "
                 f"({targets[i]['accepted']}/{targets[i]['n']} accepted); threshold {fam['threshold']}")
    elif policy == "exploit":
        ok = [i for i, t in enumerate(targets) if t["n"] >= fam["min_n"] and t["lcb"] >= fam["threshold"]]
        if ok:
            i = ok[0]
            t = targets[i]
            basis = (f"delphi-loop exploit: {t['accepted']}/{t['n']} accepted, lower bound {t['lcb']:.3f} "
                     f">= {fam['threshold']}, the cheapest target that clears it")
        else:
            i = names.index(fam["incumbent"]) if fam["incumbent"] in names else len(names) - 1
            basis = (f"delphi-loop exploit: no target clears lower bound {fam['threshold']} on "
                     f">= {fam['min_n']} episodes; incumbent {names[i]}")
    else:
        return {"error": f"unknown policy {policy!r} (exploit|thompson)"}
    basis += f"; registry {registry['episodes']} episodes, digest {registry['episodes_digest'][:12]}"
    return {"route": {"family": family, "choice": names[i], "escalation": names[i + 1:], "basis": basis,
                      "policy": policy}}


# ---------------------------------------------------------------------------
# LangSmith
# ---------------------------------------------------------------------------

def langsmith_client():
    if not (os.environ.get("LANGSMITH_API_KEY") or os.environ.get("LANGCHAIN_API_KEY")):
        raise LoopError("LANGSMITH_API_KEY is not set; push/pull/dataset need a LangSmith key")
    try:
        from langsmith import Client
    except ImportError:
        raise LoopError("the langsmith package is not installed (pip install -r tools/delphi_loop/requirements.txt)")
    # Synchronous uploads: a one-shot CLI must fail loudly, not lose runs on a background thread.
    return Client(auto_batch_tracing=False)


def project_id(client, project: str) -> str:
    """The project's id: feedback carries it as session_id (runs carry project_name)."""
    try:
        return str(client.read_project(project_name=project).id)
    except Exception as exc:      # langsmith raises LangSmithNotFoundError; keep the import optional
        if "NotFound" not in type(exc).__name__:
            raise
        return str(client.create_project(project, description="task_card lane traces (delphi-loop)").id)


def project_name(args_project: str | None) -> str:
    return args_project or os.environ.get("LANGSMITH_PROJECT") or "delphi-harness"


def _dotted(start: datetime, run_id: str) -> str:
    return start.strftime("%Y%m%dT%H%M%S%fZ") + run_id


def _text(repo: Path, rel_path: str | None) -> str | None:
    if rel_path and (repo / rel_path).is_file():
        return (repo / rel_path).read_text(encoding="utf-8")
    return None


def push(client, project: str, include_text: bool, repo: Path = REPO, say=print) -> int:
    episodes = read_episodes(repo)
    state = _state(repo, "pushed.json")
    roots, done = state.setdefault("roots", {}), set(state.setdefault("episodes", []))
    pid = project_id(client, project)
    pending = [r for r in episodes.values() if r["id"] not in done and r["source"] != "langsmith"]
    by_group: dict[str, list[dict]] = {}
    for row in pending:
        by_group.setdefault(row["group"], []).append(row)
    for group, rows in sorted(by_group.items()):
        start = datetime.fromisoformat(rows[0]["observed_at"])
        root = roots.get(group)
        if root is None:
            root = {"id": stable_id("root", group), "start": start.isoformat()}
            outcome = rows[0].get("lane_outcome")
            client.create_run(name="task_card.lane", run_type="chain", project_name=project, id=root["id"],
                              trace_id=root["id"], dotted_order=_dotted(start, root["id"]), start_time=start,
                              end_time=start, inputs={"lane": group, "card": rows[0].get("card")},
                              outputs={"outcome": outcome}, tags=["delphi-loop", rows[0]["source"]])
            client.create_feedback(run_id=root["id"], trace_id=root["id"], session_id=pid, start_time=start,
                                   feedback_id=stable_id("feedback", root["id"], "lane_outcome"),
                                   key="lane_outcome", value=outcome, score=1.0 if outcome not in ("impeded", "card_refused", "assessment_refused") else 0.0)
            roots[group] = root
        root_start = datetime.fromisoformat(root["start"])
        for offset, row in enumerate(sorted(rows, key=lambda r: (r["stage"] != "compile", r["attempt"])), 1):
            t0 = root_start + timedelta(microseconds=offset)
            inputs: dict[str, Any] = {"prompt_sha256": row["prompt_sha256"]}
            outputs: dict[str, Any] = {"reply_sha256": row["reply_sha256"], "label": row["label"]}
            if include_text:
                inputs["prompt"] = _text(repo, row["prompt_file"])
                outputs["reply"] = _text(repo, row["reply_file"])
            meta = {k: row.get(k) for k in ("family", "stage", "attempt", "target", "backend", "free", "batch_id",
                                            "stop_reason", "truncated", "route_basis", "group", "call", "source")}
            meta["episode"] = row
            client.create_run(name=f"delphi.{row['family']}", run_type="llm", project_name=project, id=row["id"],
                              parent_run_id=root["id"], trace_id=root["id"],
                              dotted_order=_dotted(root_start, root["id"]) + "." + _dotted(t0, row["id"]),
                              start_time=t0, end_time=t0, inputs=inputs, outputs=outputs,
                              error=row["label"] if row["label"] == "refused" else None,
                              extra={"metadata": {**meta, "ls_model_name": row["target"]}},
                              tags=["delphi-loop", row["family"], row["target"]])
            client.create_feedback(run_id=row["id"], trace_id=root["id"], session_id=pid, start_time=t0,
                                   feedback_id=stable_id("feedback", row["id"], "ingest_accepted"), key="ingest_accepted",
                                   score=1.0 if row["label"] == "accepted" else 0.0, value=row["label"],
                                   comment="task_card ingest verdict (mechanical)")
            done.add(row["id"])
    state["episodes"] = sorted(done)
    _save_state(repo, "pushed.json", state)
    say(f"PUSH {len(pending)} episodes in {len(by_group)} traces -> LangSmith project {project}"
        + ("" if include_text else " (hashes only)"))
    return 0


def pull(client, project: str, repo: Path = REPO, say=print) -> int:
    episodes = read_episodes(repo)
    added = 0
    for run in client.list_runs(project_name=project, run_type="llm", filter='has(tags, "delphi-loop")'):
        meta = ((run.extra or {}).get("metadata") or {})
        row = meta.get("episode")
        if isinstance(row, dict) and row.get("schema") == EPISODE_SCHEMA and row["id"] not in episodes:
            episodes[row["id"]] = {**row, "source": "langsmith"}
            added += 1
    ids = list(episodes)
    reviews = 0
    for i in range(0, len(ids), 100):
        latest: dict[str, tuple] = {}
        for fb in client.list_feedback(run_ids=ids[i:i + 100], feedback_key=[REVIEW_KEY]):
            rid = str(fb.run_id)
            if fb.score is not None and (rid not in latest or fb.created_at > latest[rid][0]):
                latest[rid] = (fb.created_at, float(fb.score))
        for rid, (_, score) in latest.items():
            if episodes[rid].get("review") != score:
                episodes[rid]["review"] = score
                reviews += 1
    write_episodes(episodes, repo)
    say(f"PULL {added} episodes from other runs, {reviews} review updates from LangSmith project {project}")
    return 0


def dataset(client, family: str, name: str | None, repo: Path = REPO, say=print) -> int:
    """Accepted (prompt, reply) pairs: the reference set `eval` replays and LangSmith experiments compare on."""
    name = name or f"delphi-{family}"
    rows = [r for r in read_episodes(repo).values()
            if r["family"] == family and effective_label(r) == "accepted"]
    examples = []
    for r in rows:
        prompt, reply = _text(repo, r["prompt_file"]), _text(repo, r["reply_file"])
        if prompt is None or reply is None:
            continue
        examples.append({"id": stable_id("example", r["prompt_sha256"]), "inputs": {"prompt": prompt},
                         "outputs": {"reply": reply},
                         "metadata": {"family": family, "target": r["target"], "group": r["group"],
                                      "card": r.get("card"), "episode": r["id"], "prompt_file": r["prompt_file"]}})
    examples = list({e["id"]: e for e in examples}.values())
    if not client.has_dataset(dataset_name=name):
        client.create_dataset(name, description=f"Accepted task_card {family} replies (delphi-loop)")
    if examples:
        client.create_examples(dataset_name=name, examples=examples)
    say(f"DATASET {name}: {len(examples)} examples (prompt and reply text uploaded)")
    return 0


# ---------------------------------------------------------------------------
# Eval: replay stored prompts on a candidate target, scored by ingest
# ---------------------------------------------------------------------------

def _score(stage: str, reply: str, row: dict, out: str, repo: Path) -> list[str]:
    if stage == "compile":
        return tc.ingest(reply, out, "card", None, repo)
    _, _, card = tc.load_card(row["card"], repo)
    return tc.ingest(reply, out, "assessment", card, repo)


def evaluate(family: str, target: str, limit: int, repo: Path = REPO, ask=None, say=print) -> int:
    """Each replay is a lane-shaped record under .delphi-loop/eval/, so `harvest` reads it like any lane."""
    stage = {v: k for k, v in STAGE_FAMILY.items()}[family]
    seen, picks = set(), []
    for r in read_episodes(repo).values():
        if (r["family"] == family and r["source"] != "eval" and effective_label(r) == "accepted"
                and r["prompt_sha256"] not in seen and _text(repo, r["prompt_file"]) is not None
                and (stage == "compile" or r.get("card"))):
            seen.add(r["prompt_sha256"])
            picks.append(r)
    picks = sorted(picks, key=lambda r: r["group"])[:limit]
    if not picks:
        raise LoopError(f"no accepted {family} episodes with their prompt on disk to replay")
    ask = ask or (lambda prompt, workdir, name, opts: tc._ask("delphi", prompt, workdir, name, opts))
    if show(repo, home(repo)).startswith("/"):
        raise LoopError("eval scores through task_card ingest, which writes inside the repository; "
                        "DELPHI_LOOP_HOME must be inside it")
    run_dir = home(repo) / "eval" / f"{time.strftime('%Y%m%dT%H%M%SZ', time.gmtime())}-{family}"
    opts = {"delphi": os.environ.get("DELPHI_REAL") or os.environ.get("DELPHI", "delphi"), "model": tc.DIRECTOR_MODEL,
            "target": tc._target(target), "free": os.environ.get("DELPHI_FREE") or None}
    tally = {lab: 0 for lab in LABELS}
    for i, row in enumerate(picks):
        workdir = run_dir / f"{i:03d}.lane"
        workdir.mkdir(parents=True)
        text = _text(repo, row["prompt_file"])
        prompt = workdir / f"{stage}.prompt.md"
        prompt.write_text(text, encoding="utf-8")
        record: dict[str, Any] = {"backend": "delphi", "card": row.get("card"), "eval_of": row["id"], "calls": {}}
        try:
            reply, meta = ask(prompt, workdir, stage, opts)
        except tc._Refused as exc:
            record["calls"][stage] = {"prompt_sha256": tc.sha256_bytes(text.encode()), "target": opts["target"],
                                      "refused": str(exc)}
        else:
            (workdir / f"{stage}.reply.md").write_text(reply, encoding="utf-8")
            findings = _score(stage, reply, row, tc.rel(repo, workdir / f"scored.{stage}.yaml"), repo)
            record["calls"][stage] = {"prompt_sha256": tc.sha256_bytes(text.encode()),
                                      "reply_sha256": tc.sha256_bytes(reply.encode()), "target": opts["target"],
                                      "accepted": not findings, "findings": findings, **meta}
        record["outcome"] = "eval"
        (workdir / "lane.json").write_text(json.dumps(record, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        for ep in lane_episodes(workdir / "lane.json", repo):
            episodes = read_episodes(repo)
            episodes[ep["id"]] = ep
            write_episodes(episodes, repo)
            tally[ep["label"]] += 1
            say(f"EVAL {i + 1}/{len(picks)} {target} on {row['group']}: {ep['label']}")
    say(f"EVAL {family} {target}: " + ", ".join(f"{k} {v}" for k, v in tally.items()) + f"; records {show(repo, run_dir)}")
    return 0


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def print_registry(registry: dict[str, Any], say=print) -> None:
    for family, fam in registry["families"].items():
        say(f"{family}  (incumbent {fam['incumbent']}, threshold {fam['threshold']}, min_n {fam['min_n']})")
        say(f"  {'target':42} {'n':>4} {'acc':>4} {'rej':>4} {'ref':>4} {'spec':>4} {'mean':>6} {'lcb':>6} {'$/accept':>9}")
        for t in fam["targets"]:
            cost = f"{t['usd_per_accept']:.4f}" if t["usd_per_accept"] is not None else "-"
            say(f"  {t['target']:42} {t['n']:>4} {t['accepted']:>4} {t['rejected']:>4} {t['refused']:>4} "
                f"{t['spec_error']:>4} {t['mean']:>6.3f} {t['lcb']:>6.3f} {cost:>9}")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--repo", type=Path, default=REPO, help="the repository the lanes live in")
    sub = parser.add_subparsers(dest="command", required=True)
    h = sub.add_parser("harvest", help="lane records -> episodes")
    h.add_argument("roots", nargs="*", help="directories to search instead of git's tracked and untracked files")
    sub.add_parser("fit", help="episodes -> registry, printed and written")
    r = sub.add_parser("route", help="the `delphi route` contract")
    r.add_argument("-family", "--family", required=True)
    r.add_argument("-json", "--json", action="store_true")
    r.add_argument("-policy", "--policy", default=os.environ.get("DELPHI_LOOP_POLICY", "exploit"))
    r.add_argument("-seed", "--seed", type=int, default=None)
    for name in ("push", "pull", "dataset"):
        p = sub.add_parser(name)
        p.add_argument("--project", help="LangSmith project (default $LANGSMITH_PROJECT or delphi-harness)")
        if name == "push":
            p.add_argument("--include-text", action="store_true",
                           help="upload prompt and reply text (publishes the design to LangSmith)")
        if name == "dataset":
            p.add_argument("--family", required=True, choices=sorted(STAGE_FAMILY.values()))
            p.add_argument("--name")
            p.add_argument("--yes-upload-text", action="store_true", required=True,
                           help="required: datasets carry prompt and reply text")
    x = sub.add_parser("export", help="episodes -> delphi.episode.v1 JSONL for `delphi learn`")
    x.add_argument("-o", "--output", type=Path, help="write atomically to FILE (default stdout)")
    e = sub.add_parser("eval", help="replay accepted prompts on a candidate target (paid model calls)")
    e.add_argument("--family", required=True, choices=sorted(STAGE_FAMILY.values()))
    e.add_argument("--target", required=True, help="provider:model@effort")
    e.add_argument("--limit", type=int, default=10)
    args = parser.parse_args(argv)
    repo = args.repo.resolve()
    try:
        if args.command == "harvest":
            return harvest(repo, roots=args.roots)
        if args.command in ("fit", "route"):
            registry = fit(read_episodes(repo).values(), load_config())
            if args.command == "fit":
                print_registry(registry)
                print(f"REGISTRY {save_registry(registry, repo)}")
                return 0
            rng = random.Random(args.seed) if args.seed is not None else None
            answer = route(registry, args.family, args.policy, os.environ.get("DELPHI_FREE") or None, rng)
            print(json.dumps(answer, sort_keys=True) if args.json else
                  answer.get("error") or f"{answer['route']['choice']}  ({answer['route']['basis']})")
            return 1 if "error" in answer else 0
        if args.command == "export":
            return export_episodes(repo, args.output)
        if args.command == "eval":
            return evaluate(args.family, args.target, args.limit, repo)
        client, project = langsmith_client(), project_name(args.project)
        if args.command == "push":
            return push(client, project, args.include_text, repo)
        if args.command == "pull":
            return pull(client, project, repo)
        return dataset(client, args.family, args.name, repo)
    except (LoopError, tc.CardError) as exc:
        print(f"DELPHI-LOOP ERROR {exc}")
        return 2


if __name__ == "__main__":
    sys.exit(main())
