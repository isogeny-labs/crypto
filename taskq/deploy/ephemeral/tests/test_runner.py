"""Exercise runner setup exports and real signal delivery without AWS."""
import os
from pathlib import Path
import signal
import shlex
import shutil
import subprocess
import sys
import tempfile
import time
import unittest


RUNNER = Path(__file__).resolve().parents[1] / "runner.sh"


class RunnerTests(unittest.TestCase):
    def test_resume_directory_is_available_to_setup(self):
        source = RUNNER.read_text()
        setup = source[source.index("export EPHEMERAL_JOB_ID="):
                       source.index('if [ -f "$W/spot-interrupt" ]; then')]
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            script = """
set -eu
W=$1 OUT=$1/out SRC=$1/src WORKDIR=$1 EPHEMERAL_JOB=test RESUME_JOB=prior
say() { :; }
fail() { exit 1; }
job() { echo 'printf "%s\\n" "$EPHEMERAL_RESUME_DIR" > setup-resume'; }
""" + setup
            subprocess.run(["bash", "-c", script, "runner-test", directory], check=True)
            self.assertEqual((root / "setup-resume").read_text().strip(),
                             str(root / "resume"))

    def test_spot_process_group_signal_reaches_application(self):
        source = RUNNER.read_text()
        run = source.split("# ---- run ", 1)[1].split("\n", 1)[1]
        run = run.split("# ---- collect", 1)[0]
        timer = shutil.which("time")
        self.assertIsNotNone(timer, "install GNU time to exercise the real runner")
        run = run.replace("/usr/bin/time", shlex.quote(timer))
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "env").touch()
            app = root / "app.py"
            app.write_text("""
import os, pathlib, signal, sys, time
root = pathlib.Path(sys.argv[1])
def checkpoint(*_):
    (root / 'checkpoint').write_text('saved after SIGTERM')
    sys.exit(0)
signal.signal(signal.SIGTERM, checkpoint)
(root / 'ready').write_text(str(os.getpgrp()))
while True: time.sleep(0.02)
""")
            # The command receives its paths via the environment, as real jobs do.
            (root / "cmd.sh").write_text(
                'exec "$RUNNER_TEST_PYTHON" "$RUNNER_TEST_ROOT/app.py" "$RUNNER_TEST_ROOT"\n')
            script = """
set -uo pipefail
W=$1 WORKDIR=$1 EPHEMERAL_DEADLINE=$(($(date +%s) + 1200))
say() { :; }
status() { :; }
job() { echo signal-test; }
fail() { exit 1; }
""" + run
            env = {**os.environ, "RUNNER_TEST_ROOT": directory,
                   "RUNNER_TEST_PYTHON": sys.executable}
            proc = subprocess.Popen(["bash", "-c", script, "runner-test", directory],
                                    env=env, start_new_session=True,
                                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            command_group = None
            try:
                deadline = time.monotonic() + 5
                while not (root / "ready").exists() and time.monotonic() < deadline:
                    time.sleep(0.02)
                self.assertTrue((root / "ready").exists(), "application never started")
                command_group = int((root / "command.pid").read_text())
                self.assertEqual(int((root / "ready").read_text()), command_group)
                os.killpg(command_group, signal.SIGTERM)
                while not (root / "checkpoint").exists() and time.monotonic() < deadline:
                    time.sleep(0.02)
                self.assertEqual((root / "checkpoint").read_text(), 'saved after SIGTERM')
                proc.wait(timeout=5)
            finally:
                for group in (command_group, proc.pid):
                    if group is not None:
                        try:
                            os.killpg(group, signal.SIGKILL)
                        except ProcessLookupError:
                            pass
                proc.wait(timeout=5)


if __name__ == "__main__":
    unittest.main()
