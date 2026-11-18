#!/usr/bin/env python3
"""Metadata/HTML rendering only. Preserve all recorded science and priced rows.

Use maintained roster, curve-row and page renderers; never execute a research
producer, derive a statistic, reprice a method or assign a cover certificate.
Complete historical-source rebuilds retain their separate failed checks.
"""
import copy
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path.cwd()
sys.path.insert(0, str(ROOT / "scripts"))
import build_ic_leaderboard as leaderboard
import build_lab_browser as browser


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def text(path, value):
    Path(path).write_text(value)


report = ROOT / "research/twist_routes_1_3_20261021/execution-v19/CATALOGUE_REFRESH.json"
if report.exists():
    raise RuntimeError("fresh metadata receipt required")
names = leaderboard.Names()
original = json.loads(leaderboard.OUT_JSON.read_text())
doc = copy.deepcopy(original)
old_roster = {r["slug"]: r for r in doc["roster"]}
measured = {r["slug"] for r in doc["board"]}
records, _ = leaderboard.curve_records()
added = []
for row in leaderboard.roster(names, measured):
    if row["slug"] in old_roster:
        continue
    row["curves_yaml_key"] = (records.get(row["slug"]) or {}).get("key")
    row["factor_base_link_status"] = (records.get(row["slug"]) or {}).get(
        "factor_base_link_status", "no_curves_yaml_record")
    added.append(row)
doc["roster"].extend(added)
source = leaderboard.SOURCES["twist_sparse_confirmation"]
doc["twist_sparse_confirmation"] = leaderboard.load(source)
doc["sources"]["twist_sparse_confirmation"] = {"path": source, "sha256": digest(source)}
doc["sources"]["registry"] = {"path": leaderboard.SOURCES["registry"],
                                "sha256": digest(leaderboard.SOURCES["registry"])}
for key, value in original.items():
    if key not in {"roster", "sources"}:
        assert doc[key] == value, key
assert doc["roster"][:len(original["roster"])] == original["roster"]
text(leaderboard.OUT_JSON, json.dumps(leaderboard.stable(doc), indent=1, ensure_ascii=False) + "\n")
text(leaderboard.OUT_MD, leaderboard.markdown(doc))
text(leaderboard.OUT_HTML, leaderboard.page(doc, standalone=True))

path = ROOT / "docs/browser/data.json"
old_browser = json.loads(path.read_text())
view = copy.deepcopy(old_browser)
known = {r["slug"] for r in view["curves"]}
new_rows = []
for row in browser.curve_rows(names.reg, doc):
    if row["slug"] in known:
        continue
    row.update({"hyperelliptic_cover": None, "cover_links": [],
                "standards_provenance": [], "store_objects": []})
    new_rows.append(row)
view["curves"].extend(new_rows)
view["counts"]["curves"] = len(view["curves"])
view["counts"]["cover_assessed"] = sum(r.get("hyperelliptic_cover") is not None for r in view["curves"])
view["counts"]["cover_unassessed"] = sum(r.get("hyperelliptic_cover") is None for r in view["curves"])
for filename in ("docs/curves/registry.json", "docs/ic/leaderboard.json"):
    view["sources"][filename] = digest(filename)
for key, value in old_browser.items():
    if key not in {"curves", "sources", "counts"}:
        assert view[key] == value, key
assert view["curves"][:len(old_browser["curves"])] == old_browser["curves"]
assert {r["slug"] for r in view["curves"]} == {r["slug"] for r in names.reg["curves"]}
text(path, json.dumps(view, indent=1, ensure_ascii=False) + "\n")
receipt = {"schema": "twist.catalogue-metadata-refresh/v1", "added_roster_models": len(added),
           "added_searchable_models": len(new_rows), "all_registry_models_findable": True,
           "all_old_roster_and_browser_rows_unchanged": True, "all_priced_rows_and_scientific_metadata_unchanged": True,
           "source": source, "source_sha256": digest(source), "renderer": str(Path(__file__).relative_to(ROOT)),
           "scope": "Metadata-only append and canonical page rendering from existing recorded JSON. Complete historical-source generators still require missing inputs. New cover assessments remain unknown; no benchmark promotion or timeline merge date assigned.",
           "generated": {str(p.relative_to(ROOT)): digest(p) for p in (
               leaderboard.OUT_JSON, leaderboard.OUT_MD, leaderboard.OUT_HTML, path)}}
report.write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps({k: receipt[k] for k in ("added_roster_models", "added_searchable_models", "all_registry_models_findable")}))
