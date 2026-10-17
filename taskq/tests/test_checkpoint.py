import json
import threading
import time

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
    assert _resume_checkpoint(tmp_path, task, 3, "state.bin") == older
    save(newer, newer)
    newer.write_bytes(b"corrupted")
    assert _resume_checkpoint(tmp_path, task, 3, "state.bin") == older


def test_spot_shutdown_requeues_and_resumes_from_durable_checkpoint(
        store, repos, repo, tmp_path):
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
