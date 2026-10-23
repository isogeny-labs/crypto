#!/usr/bin/env python3
"""Tests for scripts/validation_gate.py, the builder's use of it, and the CI check.

    python3 scripts/test_validation_gate.py

Positive fixture: scripts/fixtures/validation_gate/positive (a complete
VALIDATION.json).  Negative fixtures: scripts/fixtures/validation_gate/
negative-no-rho-reference, plus in-test mutations of the positive one.  The toy
numbers in the fixtures are not a measurement.
"""
from __future__ import annotations

import copy
import json
import shutil
import subprocess
import sys
import tempfile
import unittest
import unittest.mock
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import audit_runs as audit  # noqa: E402
import validation_gate as g  # noqa: E402

REPO = g.REPO
FIX = REPO / "scripts/fixtures/validation_gate"


def dump(p: Path, o) -> None:
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(o, indent=1, sort_keys=True) + "\n")


def row(slug="icv1-toy", source="runs/toy/analysis.json") -> dict:
    return {"slug": slug, "source": source, "best": {"s": 1.5}}


class Scratch(unittest.TestCase):
    """A throwaway repository tree: tests never touch the real leaderboard."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=REPO / "scripts")  # inside REPO: paths stay relative to it
        self.repo = Path(self.tmp.name)
        self.addCleanup(self.tmp.cleanup)

    def evidence(self, name="runs/toy", mutate=None) -> Path:
        d = self.repo / name
        shutil.copytree(FIX / "positive", d, dirs_exist_ok=True)
        if mutate:
            mutate(d)
        return d

    def edit(self, d: Path, name: str, fn) -> None:
        o = json.loads((d / name).read_text())
        fn(o)
        dump(d / name, o)

    def repin(self, d: Path, key: str, name: str) -> None:
        self.edit(d, "VALIDATION.json", lambda m: m[key].update(sha256=g.sha256_file(d / name)))


class EvidenceDir(Scratch):
    def errors(self, d: Path) -> list[str]:
        return g.check_evidence_dir(d, self.repo)

    def test_positive_fixture_is_valid(self):
        self.assertEqual(g.check_evidence_dir(FIX / "positive", REPO), [])

    def test_committed_negative_fixture_is_refused(self):
        e = g.check_evidence_dir(FIX / "negative-no-rho-reference", REPO)
        self.assertEqual(e, ["rho_reference: missing"])

    def test_missing_manifest(self):
        d = self.repo / "empty"
        d.mkdir()
        self.assertEqual(self.errors(d), [f"{g.MANIFEST} is missing"])

    def test_wrong_schema(self):
        d = self.evidence()
        self.edit(d, "VALIDATION.json", lambda m: m.update(schema="other/v1"))
        self.assertIn("schema", self.errors(d)[0])

    def test_replay_certificate_missing_file(self):
        d = self.evidence()
        (d / "replay-certificate.json").unlink()
        self.assertTrue(any("replay certificate" in e for e in self.errors(d)))

    def test_tampered_certificate_fails_the_pin(self):
        d = self.evidence()
        self.edit(d, "replay-certificate.json", lambda c: c.update(extra=1))
        self.assertTrue(any("sha256 does not match" in e for e in self.errors(d)))

    def test_scalars_not_verified(self):
        d = self.evidence()
        self.edit(d, "replay-certificate.json", lambda c: c.update(scalars_verified=False))
        self.repin(d, "replay_certificate", "replay-certificate.json")
        self.assertEqual(self.errors(d), ["replay certificate: scalars_verified is not true"])

    def test_failed_replay_status(self):
        d = self.evidence()
        self.edit(d, "replay-certificate.json", lambda c: c.update(status="mismatch"))
        self.repin(d, "replay_certificate", "replay-certificate.json")
        self.assertIn("not a verified replay", self.errors(d)[0])

    def test_validator_headline_disagrees(self):
        d = self.evidence()
        self.edit(d, "validator-report.json", lambda r: r["headline"][0].update(recomputed=2.0))
        self.repin(d, "validator_report", "validator-report.json")
        self.assertIn("recomputed 2.0", self.errors(d)[0])

    def test_validator_raw_data_must_exist(self):
        d = self.evidence()
        (d / "raw-measurements.json").unlink()
        self.assertTrue(any("raw_data" in e for e in self.errors(d)))

    def test_validator_raw_data_hash_is_checked(self):
        d = self.evidence()
        dump(d / "raw-measurements.json", {"edited": True})
        self.assertTrue(any("sha256 mismatch" in e for e in self.errors(d)))

    def test_artifact_path_cannot_leave_the_repository(self):
        d = self.evidence()
        self.edit(d, "VALIDATION.json", lambda m: m["replay_certificate"].update(path="../../../../../etc/passwd"))
        self.assertTrue(any("leaves the repository" in e or "not a file" in e for e in self.errors(d)))

    def test_rho_reference_band_needs_three_points(self):
        d = self.evidence()
        self.edit(d, "VALIDATION.json", lambda m: m["rho_reference"]["gae_band"].pop("0.1"))
        self.assertTrue(any("gae_band must have exactly" in e for e in self.errors(d)))

    def test_rho_reference_must_be_strongest_and_same_binary(self):
        d = self.evidence()
        def weaken(m):
            m["rho_reference"].update(strongest_available=False, same_binary_or_process=False, kind="plain")
        self.edit(d, "VALIDATION.json", weaken)
        e = " ".join(self.errors(d))
        self.assertIn("strongest_available", e)
        self.assertIn("same_binary_or_process", e)
        self.assertIn("kind", e)

    def test_manifest_row_list_restricts_rows(self):
        d = self.evidence()
        self.edit(d, "VALIDATION.json", lambda m: m.update(rows=["icv1-other"]))
        self.assertTrue(g.check_evidence_dir(d, self.repo, slug="icv1-toy"))
        self.assertEqual(g.check_evidence_dir(d, self.repo, slug="icv1-other"), [])


class Board(Scratch):
    def run_board(self, rows, pre_gate=None, verdicts=None, allow=False):
        if pre_gate is not None:
            dump(self.repo / g.PRE_GATE_LEADERBOARD, {"rows": pre_gate})
        if verdicts is not None:
            p = self.repo / g.SWEEPS["leaderboard"]
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text("\n".join(json.dumps(v) for v in verdicts) + "\n")
        g.annotate_board(rows, g.row_digest, self.repo, allow_unvalidated=allow)
        return rows

    def rel(self, d: Path) -> str:
        return str(d.relative_to(self.repo))

    def test_validated_row_with_manifest(self):
        d = self.evidence("ev")
        r = row(source=self.rel(d) + "/analysis.json")
        self.run_board([r])
        self.assertEqual(r["validation"]["status"], g.VALIDATED)

    def test_frozen_row_is_pre_gate(self):
        r = row()
        self.run_board([r], pre_gate={g.board_key(r): g.row_digest(r)})
        self.assertEqual(r["validation"]["status"], g.PRE_GATE)

    def test_new_row_is_refused(self):
        with self.assertRaises(SystemExit) as cm:
            self.run_board([row()], pre_gate={})
        self.assertIn("validation gate refuses", str(cm.exception))

    def test_changed_row_loses_its_pre_gate_pass(self):
        r = row()
        frozen = {g.board_key(r): g.row_digest(r)}
        r["best"]["s"] = 1.0  # a change after the freeze
        with self.assertRaises(SystemExit) as cm:
            self.run_board([r], pre_gate=frozen)
        self.assertIn("changed since the pre-gate freeze", str(cm.exception))

    def test_allow_unvalidated_marks_instead_of_refusing(self):
        r = row()
        self.run_board([r], pre_gate={}, allow=True)
        self.assertEqual(r["validation"]["status"], g.UNVALIDATED)

    def test_digest_ignores_the_annotation(self):
        r = row()
        d = g.row_digest(r)
        r["validation"] = {"status": "x"}
        self.assertEqual(g.row_digest(r), d)

    def test_sweep_pass_verdict_validates_a_pre_gate_row(self):
        r = row()
        v = {"row": r["slug"], "verdict": "validated", "replay_certificate": "c", "validator_report": "v",
             "rho_reference": "r"}
        self.run_board([r], pre_gate={g.board_key(r): g.row_digest(r)}, verdicts=[v])
        self.assertEqual(r["validation"]["status"], g.VALIDATED)

    def test_sweep_pass_without_the_three_artifacts_does_not_validate(self):
        r = row()
        v = {"row": r["slug"], "verdict": "validated", "replay_certificate": "c"}
        self.run_board([r], pre_gate={g.board_key(r): g.row_digest(r)}, verdicts=[v])
        self.assertEqual(r["validation"]["status"], g.PRE_GATE)

    def test_sweep_failure_beats_everything(self):
        d = self.evidence("ev")
        r = row(source=self.rel(d) + "/analysis.json")
        with self.assertRaises(SystemExit):
            self.run_board([r], pre_gate={}, verdicts=[{"row": r["slug"], "verdict": "failed"}])
        self.assertEqual(r["validation"]["status"], g.FAILED)

    def test_verdict_for_an_older_row_is_ignored(self):
        r = row()
        v = {"row": r["slug"], "verdict": "failed", "row_digest": "0" * 64}
        self.run_board([r], pre_gate={g.board_key(r): g.row_digest(r)}, verdicts=[v])
        self.assertEqual(r["validation"]["status"], g.PRE_GATE)

    # --- the validation sweep's own line shape (row_id, table prefix, its verdict words) ---

    def sweep_line(self, r, verdict, prefix="A:", **kw):
        v = {"row_id": prefix + r["slug"], "table": prefix[0], "verdict": verdict,
             "pricing_ok": "not_applicable", "rho_reference_ok": "not_applicable"}
        v.update(kw)
        return v

    def frozen(self, r):
        return {g.board_key(r): g.row_digest(r)}

    def test_row_id_with_table_prefix_reaches_the_board_row(self):
        for prefix in ("A:", "B:", "C:"):
            r = row()
            self.run_board([r], pre_gate=self.frozen(r),
                           verdicts=[self.sweep_line(r, "contradicted", prefix)], allow=True)
            self.assertEqual(r["validation"]["status"], g.FAILED, prefix)

    def test_non_board_prefix_is_not_stripped(self):
        r = row()
        self.run_board([r], pre_gate=self.frozen(r),
                       verdicts=[self.sweep_line(r, "contradicted", "S:")])
        self.assertEqual(r["validation"]["status"], g.PRE_GATE)
        self.assertIn("S:" + r["slug"], g.load_verdicts("leaderboard", self.repo))
        self.assertNotIn(r["slug"], g.load_verdicts("leaderboard", self.repo))

    def test_row_key_still_works(self):
        r = row()
        self.run_board([r], pre_gate=self.frozen(r), verdicts=[{"row": r["slug"], "verdict": "contradicted"}],
                       allow=True)
        self.assertEqual(r["validation"]["status"], g.FAILED)

    def test_contradicted_fails_and_the_builder_refuses(self):
        r = row()
        with self.assertRaises(SystemExit) as cm:
            self.run_board([r], pre_gate=self.frozen(r), verdicts=[self.sweep_line(r, "contradicted")])
        self.assertIn("contradicted", str(cm.exception))

    def test_caveat_and_unvalidated_words_are_recognized_but_never_pass(self):
        for word in ("validated_with_caveats", "unvalidated"):
            r = row()
            self.run_board([r], pre_gate=self.frozen(r), verdicts=[self.sweep_line(
                r, word, pricing_ok=False, rho_reference_ok="partial",
                replay_certificate="c", validator_report="v", rho_reference="r")])
            self.assertEqual(r["validation"]["status"], g.PRE_GATE, word)
            self.assertEqual(r["validation"]["sweep"]["verdict"], word)
            self.assertIs(r["validation"]["sweep"]["pricing_ok"], False)
            self.assertEqual(r["validation"]["sweep"]["rho_reference_ok"], "partial")
            self.assertIn("rho_reference_ok=partial", "; ".join(r["validation"]["reasons"]))

    def test_caveated_row_not_in_the_freeze_is_refused_not_promoted(self):
        r = row()
        with self.assertRaises(SystemExit):
            self.run_board([r], pre_gate={}, verdicts=[self.sweep_line(r, "validated_with_caveats")])

    def test_validated_word_without_artifact_pointers_does_not_pass(self):
        r = row()  # the sweep's own `validated` line carries none of the three artifact keys
        self.run_board([r], pre_gate=self.frozen(r), verdicts=[self.sweep_line(r, "validated")])
        self.assertEqual(r["validation"]["status"], g.PRE_GATE)
        self.assertIn("lacks replay_certificate", "; ".join(r["validation"]["reasons"]))

    def test_validated_with_pointers_passes_only_when_caveats_are_clean(self):
        art = {"replay_certificate": "c", "validator_report": "v", "rho_reference": "r"}
        r = row()
        self.run_board([r], pre_gate=self.frozen(r), verdicts=[self.sweep_line(r, "validated", **art)])
        self.assertEqual(r["validation"]["status"], g.VALIDATED)
        for bad in ({"pricing_ok": False}, {"rho_reference_ok": "partial"}, {"replay_ok": False}):
            r = row()
            self.run_board([r], pre_gate=self.frozen(r), verdicts=[self.sweep_line(r, "validated", **art, **bad)])
            self.assertEqual(r["validation"]["status"], g.PRE_GATE, bad)

    def test_unknown_verdict_word_is_an_explicit_error(self):
        r = row()
        with self.assertRaises(SystemExit) as cm:
            self.run_board([r], pre_gate=self.frozen(r), verdicts=[self.sweep_line(r, "probably_fine")])
        self.assertIn("unrecognized verdict", str(cm.exception))
        with self.assertRaises(SystemExit):
            g.load_verdicts("leaderboard", self.repo)

    def test_missing_verdict_or_row_is_an_explicit_error(self):
        r = row()
        for line in ({"row_id": "A:" + r["slug"]}, {"verdict": "validated"}):
            with self.assertRaises(SystemExit):
                self.run_board([r], pre_gate=self.frozen(r), verdicts=[line])

    def test_failing_verdict_wins_over_manifest_and_over_a_later_passing_line(self):
        d = self.evidence("ev")
        r = row(source=self.rel(d) + "/analysis.json")
        art = {"replay_certificate": "c", "validator_report": "v", "rho_reference": "r"}
        lines = [self.sweep_line(r, "contradicted"), self.sweep_line(r, "validated", **art)]
        with self.assertRaises(SystemExit):
            self.run_board([r], pre_gate={}, verdicts=lines)
        self.assertEqual(r["validation"]["status"], g.FAILED)


class CiCheck(Scratch):
    LINE = "| 2026-10-19 | [`20261019-toy`](20261019-toy/) | toy | headline | host | study |"

    def tree(self, rows, readme_lines, lb_pre=None, bm_pre=None):
        dump(self.repo / g.LEADERBOARD_JSON, {"board": rows})
        (self.repo / g.BENCHMARKS_README).parent.mkdir(parents=True, exist_ok=True)
        (self.repo / g.BENCHMARKS_README).write_text("# Benchmarks\n\n| date | entry |\n|:--|:--|\n"
                                                    + "\n".join(readme_lines) + "\n")
        dump(self.repo / g.PRE_GATE_LEADERBOARD, {"rows": lb_pre or {}})
        dump(self.repo / g.PRE_GATE_BENCHMARKS, {"rows": bm_pre or {}})

    def annotated(self, r, status):
        r = copy.deepcopy(r)
        r["validation"] = {"status": status}
        return r

    def test_negative_new_leaderboard_row_fails(self):
        self.tree([self.annotated(row(), g.PRE_GATE)], [])
        problems = g.check(self.repo)
        self.assertEqual(len(problems), 1)
        self.assertIn("not in the pre-gate freeze", problems[0])

    def test_negative_new_benchmarks_row_fails(self):
        self.tree([], [self.LINE])
        problems = g.check(self.repo)
        self.assertEqual(len(problems), 1)
        self.assertIn("benchmarks 20261019-toy", problems[0])

    def test_negative_edited_benchmarks_row_fails(self):
        self.tree([], [self.LINE.replace("headline", "a better headline")],
                  bm_pre={"20261019-toy": g.line_digest(self.LINE)})
        self.assertIn("changed since the pre-gate freeze", g.check(self.repo)[0])

    def test_positive_frozen_rows_pass(self):
        r = row()
        self.tree([self.annotated(r, g.PRE_GATE)], [self.LINE],
                  lb_pre={g.board_key(r): g.row_digest(r)}, bm_pre={"20261019-toy": g.line_digest(self.LINE)})
        self.assertEqual(g.check(self.repo), [])

    def test_positive_validated_rows_pass(self):
        d = self.evidence("runs/toy")
        r = row(source="runs/toy/analysis.json")
        bm = self.repo / "benchmarks/20261019-toy"
        shutil.copytree(FIX / "positive", bm)
        self.tree([self.annotated(r, g.VALIDATED)], [self.LINE])
        self.assertEqual(g.check(self.repo), [])
        self.assertTrue(d.is_dir())

    def test_committed_status_must_match_the_judgement(self):
        r = row()
        self.tree([self.annotated(r, g.VALIDATED)], [],
                  lb_pre={g.board_key(r): g.row_digest(r)})
        self.assertIn("run scripts/build_ic_leaderboard.py", g.check(self.repo)[0])

    def test_duplicate_benchmarks_directories_are_one_unit(self):
        two = self.LINE + "\n" + self.LINE.replace("toy |", "toy again |")
        groups = g.benchmark_groups("\n" + two + "\n")
        self.assertEqual(list(groups), ["20261019-toy"])
        self.assertEqual(groups["20261019-toy"].count("\n"), 1)

    def archive(self, row_text=None, directory="20261019-toy", disposition="incomplete"):
        row_text = row_text or self.LINE
        provenance = self.repo / "research/example/README.md"
        provenance.parent.mkdir(parents=True, exist_ok=True)
        provenance.write_text("Source evidence index.\n")
        obj = {"directory": directory, "row": row_text,
               "sha256": g.hashlib.sha256(row_text.strip().encode()).hexdigest(),
               "disposition": disposition, "reason": "Evidence is incomplete; claims remain unvalidated.",
               "provenance": ["research/example/README.md"]}
        dump(self.repo / g.BENCHMARKS_ARCHIVE,
             {"schema": g.ARCHIVE_SCHEMA, "rows": [obj]})
        return obj

    def test_archived_row_is_not_admitted_and_preserves_exact_row(self):
        self.tree([], [])
        row = self.archive()
        self.assertEqual(g.benchmark_archive_errors(self.repo, {}), [])
        self.assertEqual(row["row"], self.LINE)

    def test_archive_cannot_bypass_admission_for_active_row(self):
        self.tree([], [self.LINE])
        self.archive()
        problems = g.check(self.repo)
        self.assertTrue(any("still admitted" in p for p in problems))
        self.assertTrue(any("unvalidated" in p for p in problems))

    def test_removed_row_requires_exact_archival_disposition(self):
        self.tree([], [])
        old_readme = "# Benchmarks\n" + self.LINE + "\n"
        with unittest.mock.patch.object(g, "git_show", return_value=old_readme):
            # The base readme is the old dated row; absence from the admitted table
            # is permitted only when that exact line is carried in the archive.
            errors = g.benchmark_archive_errors(self.repo, {}, base="base")
        self.assertTrue(any("lacks an exact archival disposition" in e for e in errors))
        self.archive()
        with unittest.mock.patch.object(g, "git_show", side_effect=lambda ref, path:
                                        old_readme if path == g.BENCHMARKS_README else None):
            self.assertEqual(g.benchmark_archive_errors(self.repo, {}, base="base"), [])

    def test_archive_row_hash_and_provenance_are_required(self):
        self.tree([], [])
        row = self.archive()
        row["sha256"] = "0" * 64
        row["provenance"] = []
        dump(self.repo / g.BENCHMARKS_ARCHIVE, {"schema": g.ARCHIVE_SCHEMA, "rows": [row]})
        errors = g.benchmark_archive_errors(self.repo, {})
        self.assertTrue(any("sha256" in e for e in errors))
        self.assertTrue(any("provenance" in e for e in errors))

    def test_archive_history_cannot_be_silently_removed(self):
        self.tree([], [])
        old = {"schema": g.ARCHIVE_SCHEMA, "rows": [{"directory": "20261019-toy", "row": self.LINE}]}
        with unittest.mock.patch.object(g, "git_show", side_effect=lambda ref, path:
                                        json.dumps(old) if path == g.BENCHMARKS_ARCHIVE else None):
            errors = g.benchmark_archive_errors(self.repo, {}, base="base")
        self.assertTrue(any("archival history" in e for e in errors))

    def test_new_archive_entry_must_match_a_base_row(self):
        self.tree([], [])
        self.archive()
        old_readme = "# Benchmarks\n"
        with unittest.mock.patch.object(g, "git_show", side_effect=lambda ref, path:
                                        old_readme if path == g.BENCHMARKS_README else None):
            errors = g.benchmark_archive_errors(self.repo, {}, base="base")
        self.assertTrue(any("does not match a row from the base table" in e for e in errors))

    def test_archive_entry_already_on_base_needs_no_base_row(self):
        self.tree([], [])
        obj = self.archive()
        old = {"schema": g.ARCHIVE_SCHEMA, "rows": [obj]}
        files = {g.BENCHMARKS_README: "# Benchmarks\n", g.BENCHMARKS_ARCHIVE: json.dumps(old)}
        with unittest.mock.patch.object(g, "git_show", side_effect=lambda ref, path: files.get(path)):
            self.assertEqual(g.benchmark_archive_errors(self.repo, {}, base="base"), [])

    def test_freeze_may_only_shrink(self):
        self.assertEqual(g.grown({"a": "1", "b": "2"}, {"a": "1", "b": "2"}), [])
        self.assertEqual(g.grown({"a": "1"}, {"a": "1", "b": "2"}), [])
        self.assertEqual(g.grown({"a": "1", "c": "3"}, {"a": "1"}), ["c"])
        self.assertEqual(g.grown({"a": "9"}, {"a": "1"}), ["a"])


class AuditRuns(Scratch):
    def session(self, name="sessions/s1") -> Path:
        d = self.repo / name
        d.mkdir(parents=True)
        (d / "plan.json").write_text("{}")
        (d / "records.jsonl").write_text('{"r":1}\n')
        dump(d / "session.json", {"schema": "ecbench.session/v1", "status": "complete",
                                  "plan_sha256": g.sha256_file(d / "plan.json"),
                                  "records_sha256": g.sha256_file(d / "records.jsonl")})
        return d

    def test_sampling_is_deterministic_per_nonce_and_varies_across_nonces(self):
        items = [("ecbench", f"s{i}") for i in range(40)]
        self.assertEqual(audit.sample(items, "aa", 5), audit.sample(items, "aa", 5))
        self.assertNotEqual(audit.sample(items, "aa", 5), audit.sample(items, "bb", 5))

    def test_positive_session_digests_pass(self):
        d = self.session()
        self.assertEqual(audit.digest_check(self.repo, str(d.relative_to(self.repo))), [])

    def test_negative_edited_records_fail(self):
        d = self.session()
        (d / "records.jsonl").write_text('{"r":2}\n')
        errs = audit.digest_check(self.repo, str(d.relative_to(self.repo)))
        self.assertEqual(len(errs), 1)
        self.assertIn("records.jsonl", errs[0])

    def test_validation_dirs_are_audited_in_full(self):
        d = self.evidence("ev")
        self.edit(d, "replay-certificate.json", lambda c: c.update(extra=1))
        res = audit.audit(self.repo, "validation", str(d.relative_to(self.repo)), None, 0)
        self.assertFalse(res["ok"])

    def test_candidates_find_sessions_and_manifests_but_not_fixtures(self):
        self.session()
        self.evidence("ev")
        (self.repo / "fixtures").mkdir()
        shutil.copytree(FIX / "positive", self.repo / "fixtures/x")
        kinds = sorted(audit.candidates(self.repo))
        self.assertEqual([k for k, _ in kinds], ["ecbench", "validation"])


class Committed(unittest.TestCase):
    """The repository as committed passes its own gate."""

    def test_gate_passes_on_the_committed_tree(self):
        self.assertEqual(g.check(REPO), [])

    def test_every_committed_board_row_is_annotated(self):
        doc = json.loads((REPO / g.LEADERBOARD_JSON).read_text())
        self.assertTrue(doc["board"])
        for r in doc["board"]:
            self.assertIn(r["validation"]["status"], {g.VALIDATED, g.PRE_GATE})

    def test_markdown_renders_the_status(self):
        md = (REPO / "docs/ic/LEADERBOARD.md").read_text()
        self.assertIn("| validation |", md)

    def test_builder_refuses_check_with_allow_unvalidated(self):
        r = subprocess.run([sys.executable, str(REPO / "scripts/build_ic_leaderboard.py"),
                            "--check", "--allow-unvalidated"], capture_output=True, text=True)
        self.assertNotEqual(r.returncode, 0)
        self.assertIn("never allows", r.stderr)


if __name__ == "__main__":
    unittest.main()
