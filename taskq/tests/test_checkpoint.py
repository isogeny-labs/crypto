import hashlib
import json
import os
import threading
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

import pytest

from conftest import make_spec
from taskq.checkpoint import save
from taskq.execute import _resume_checkpoint
from taskq.worker import Worker


def test_resume_ignores_torn_or_corrupt_checkpoint(tmp_path):
    task = "T-checkpoint"
    older = tmp_path / task / "attempt-1" / "state.bin"
    newer = tmp_path / task / "attempt-2" / "state.bin"
    older.parent.mkdir(parents=True)
    older.write_bytes(b"valid")
    save(older, older)
    newer.parent.mkdir(parents=True)
    newer.write_bytes(b"incomplete")
    resumed = _resume_checkpoint(tmp_path, task, 3, "state.bin")
    assert resumed.read_bytes() == b"valid"
    assert resumed.parent.name == "attempt-3"
    save(newer, newer)
    checksum = newer.with_name(newer.name + ".sha256").read_text().strip()
    newer.with_name(f".{newer.name}.{checksum}.snapshot").write_bytes(b"corrupted")
    resumed = _resume_checkpoint(tmp_path, task, 3, "state.bin")
    assert resumed.read_bytes() == b"valid"


def test_interrupted_publication_keeps_last_committed_generation(tmp_path, monkeypatch):
    source = tmp_path / "source"
    destination = tmp_path / "T-checkpoint" / "attempt-1" / "state.bin"
    source.write_bytes(b"first complete checkpoint")
    save(source, destination)
    source.write_bytes(b"next checkpoint")
    replace = os.replace

    def interrupt_marker(src, dst):
        if str(dst).endswith(".sha256"):
            raise OSError("interrupted before marker commit")
        return replace(src, dst)

    monkeypatch.setattr(os, "replace", interrupt_marker)
    with pytest.raises(OSError, match="interrupted"):
        save(source, destination)
    resumed = _resume_checkpoint(tmp_path, "T-checkpoint", 2, "state.bin")
    assert resumed.read_bytes() == b"first complete checkpoint"
    assert destination.read_bytes() == b"first complete checkpoint"


def test_committed_generation_survives_interrupted_stable_copy(tmp_path, monkeypatch):
    source = tmp_path / "source"
    destination = tmp_path / "T-checkpoint" / "attempt-1" / "state.bin"
    source.write_bytes(b"first")
    save(source, destination)
    source.write_bytes(b"second")
    replace = os.replace

    def interrupt_alias(src, dst):
        if dst == destination:
            raise OSError("interrupted after marker commit")
        return replace(src, dst)

    monkeypatch.setattr(os, "replace", interrupt_alias)
    with pytest.raises(OSError, match="interrupted"):
        save(source, destination)
    assert destination.read_bytes() == b"first"
    resumed = _resume_checkpoint(tmp_path, "T-checkpoint", 2, "state.bin")
    assert resumed.read_bytes() == b"second"


def test_resume_owns_verified_bytes_after_prior_worker_replaces_payload(tmp_path):
    old = tmp_path / "T-checkpoint" / "attempt-1" / "state.bin"
    old.parent.mkdir(parents=True)
    old.write_bytes(b"verified legacy checkpoint")
    old.with_name(old.name + ".sha256").write_text(hashlib.sha256(old.read_bytes()).hexdigest())
    resumed = _resume_checkpoint(tmp_path, "T-checkpoint", 2, "state.bin")
    old.write_bytes(b"late write after lease loss")
    assert resumed != old
    assert resumed.read_bytes() == b"verified legacy checkpoint"


def test_stable_path_is_independent_of_immutable_generation(tmp_path):
    destination = tmp_path / "T-checkpoint" / "attempt-1" / "state.bin"
    source = tmp_path / "source"
    source.write_bytes(b"verified checkpoint")
    save(source, destination)
    destination.write_bytes(b"application overwrote its stable path")
    resumed = _resume_checkpoint(tmp_path, "T-checkpoint", 2, "state.bin")
    assert resumed.read_bytes() == b"verified checkpoint"


def test_repeated_saves_bound_storage_and_keep_current_and_previous(tmp_path):
    destination = tmp_path / "T-checkpoint" / "attempt-1" / "state.bin"
    source = tmp_path / "source"
    for data in (b"first", b"second", b"third", b"fourth"):
        source.write_bytes(data)
        save(source, destination)
    generations = list(destination.parent.glob(".*.snapshot"))
    assert {path.read_bytes() for path in generations} == {b"third", b"fourth"}
    resumed = _resume_checkpoint(tmp_path, "T-checkpoint", 2, "state.bin")
    assert resumed.read_bytes() == b"fourth"


def test_overlapping_saves_cannot_remove_the_committed_generation(tmp_path, monkeypatch):
    destination = tmp_path / "T-checkpoint" / "attempt-1" / "state.bin"
    first, second = tmp_path / "first", tmp_path / "second"
    first.write_bytes(b"periodic checkpoint")
    second.write_bytes(b"SIGTERM checkpoint")
    entered, release, second_started = (threading.Event() for _ in range(3))
    replace = os.replace

    def pause_first_marker(src, dst):
        if dst == destination.with_name("state.bin.sha256") and not entered.is_set():
            entered.set()
            assert release.wait(5)
        return replace(src, dst)

    def publish_second():
        second_started.set()
        return save(second, destination)

    monkeypatch.setattr(os, "replace", pause_first_marker)
    with ThreadPoolExecutor(max_workers=2) as pool:
        a = pool.submit(save, first, destination)
        try:
            assert entered.wait(5)
            b = pool.submit(publish_second)
            assert second_started.wait(5)
            # B must not publish a marker while A can still prune generations.
            time.sleep(0.05)
            assert not b.done()
        finally:
            release.set()
        a.result(timeout=5)
        b.result(timeout=5)
    resumed = _resume_checkpoint(tmp_path, "T-checkpoint", 2, "state.bin")
    assert resumed.read_bytes() == b"SIGTERM checkpoint"


def test_invalid_utf8_marker_falls_back_to_older_checkpoint(tmp_path):
    old = tmp_path / "T-checkpoint" / "attempt-1" / "state.bin"
    old.parent.mkdir(parents=True)
    old.write_bytes(b"valid")
    save(old, old)
    newer = tmp_path / "T-checkpoint" / "attempt-2" / "state.bin"
    newer.parent.mkdir(parents=True)
    newer.write_bytes(b"other")
    newer.with_name(newer.name + ".sha256").write_bytes(b"\xff\xfe")
    resumed = _resume_checkpoint(tmp_path, "T-checkpoint", 3, "state.bin")
    assert resumed.read_bytes() == b"valid"


@pytest.mark.parametrize("relative_root", [False, True])
def test_spot_shutdown_requeues_and_resumes_from_durable_checkpoint(
        store, repos, repo, tmp_path, monkeypatch, relative_root):
    script = """
import hashlib, json, os, pathlib, signal, sys, time
resume = os.environ['TASKQ_RESUME_FILE']
if resume:
    value = pathlib.Path(resume).read_bytes().decode()
    pathlib.Path(os.environ['TASKQ_OUTPUT_DIR'], 'metrics.json').write_text(
        json.dumps({'resumed': value}))
    sys.exit(0)
dest = pathlib.Path(os.environ['TASKQ_CHECKPOINT_FILE'])
def checkpoint(*_):
    tmp = dest.with_name(dest.name + '.tmp')
    tmp.write_bytes(b'progress-17')
    tmp.replace(dest)
    dest.with_name(dest.name + '.sha256').write_text(
        hashlib.sha256(dest.read_bytes()).hexdigest())
    sys.exit(0)
signal.signal(signal.SIGTERM, checkpoint)
(dest.parent / 'started').write_text('1')
while True: time.sleep(0.05)
"""
    spec = make_spec(repo[1], ["-c", script],
                     checkpoint={"file": "state.bin"},
                     retry={"max_attempts": 3})
    tid = store.submit(spec)["task_id"]
    root = tmp_path / "checkpoints"
    if relative_root:
        monkeypatch.chdir(tmp_path)
        root = Path("checkpoints")
    w1 = Worker(store, ["cpu"], repos, worker_id="spot-1",
                artifact_dir=tmp_path / "artifacts", checkpoint_dir=root,
                result_dir=tmp_path / "results", block_seconds=0.2)
    thread = threading.Thread(target=w1.run_once)
    thread.start()
    started = root / tid / "attempt-1" / "started"
    deadline = time.monotonic() + 10
    while not started.exists() and time.monotonic() < deadline:
        time.sleep(0.05)
    assert started.exists()
    w1.request_shutdown()
    thread.join(10)
    assert not thread.is_alive()
    assert store.get_task(tid)["state"] == "queued"
    assert store.get_result(tid) is None

    w2 = Worker(store, ["cpu"], repos, worker_id="spot-2",
                artifact_dir=tmp_path / "artifacts", checkpoint_dir=root,
                result_dir=tmp_path / "results", block_seconds=0.2)
    assert w2.run_once()
    result = store.get_result(tid)
    assert result["status"] == "succeeded"
    assert result["attempt"] == 2
    assert result["runs"][0]["metrics"] == {"resumed": "progress-17"}
