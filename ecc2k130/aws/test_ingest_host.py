"""Exercise the billable EC2 launcher with an offline AWS boundary."""

import os
from pathlib import Path
import subprocess
import tempfile
import unittest


HERE = Path(__file__).resolve().parent


class IngestHostLauncherTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        aws = self.root / "aws"
        aws.write_text("""#!/usr/bin/env bash
set -euo pipefail
case "$1 $2" in
  'sts get-caller-identity') echo 123456789012 ;;
  'ec2 describe-instances')
    if [[ "$*" == *'Name=tag:Name,Values=rho-ecc2k-walker'* ]]; then
      echo None
    elif [[ "$*" == *'SecurityGroups[0].GroupId'* ]]; then
      echo sg-existing
    elif [[ "$*" == *'SubnetId'* ]]; then
      echo subnet-existing
    elif [[ "$*" == *'Name=tag:IngestRole,Values=publisher'* && "${TEST_EXISTING_PUBLISHER:-0}" = 1 ]]; then
      echo i-existing
    fi ;;
  'ec2 describe-images') echo ami-test ;;
  'iam get-instance-profile') echo '{}' ;;
  'rds describe-db-instances') echo db.example ;;
  's3 cp') exit 0 ;;
  'ec2 run-instances')
    printf '%s\\n' "$@" > "$TEST_RUN_ARGS"
    while [ "$#" -gt 0 ]; do
      if [ "$1" = --user-data ]; then
        cp "${2#file://}" "$TEST_USERDATA"
        break
      fi
      shift
    done
    echo i-test ;;
  *) echo "unexpected AWS call: $*" >&2; exit 3 ;;
esac
""")
        aws.chmod(0o755)
        launchctl = self.root / "launchctl"
        launchctl.write_text("#!/bin/sh\n"
                             "if [ \"${TEST_LOCAL_RUNNING:-0}\" = 1 ]; then\n"
                             "  echo '    state = running'\n"
                             "fi\n")
        launchctl.chmod(0o755)

    def run_launcher(self, role, command, **overrides):
        env = os.environ.copy()
        env.update(HOME=str(self.root), PATH=f"{self.root}:{env['PATH']}", INGEST_ROLE=role,
                   TEST_RUN_ARGS=str(self.root / "run-args"),
                   TEST_USERDATA=str(self.root / "user-data"))
        env.update(overrides)
        return subprocess.run(["bash", str(HERE / "ingest_host.sh"), command],
                              cwd=HERE, env=env, text=True, capture_output=True)

    def test_preflight_uses_stopped_host_when_no_walker_runs(self):
        result = self.run_launcher("worker", "preflight")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("role=worker", result.stdout)
        self.assertIn("security_group=sg-existing subnet=subnet-existing", result.stdout)
        self.assertFalse((self.root / "user-data").exists())

    def test_each_role_reaches_systemd_and_instance_tags(self):
        for role, worker_only in (("publisher", "0"), ("worker", "1")):
            with self.subTest(role=role):
                result = self.run_launcher(role, "up")
                self.assertEqual(result.returncode, 0, result.stderr)
                unit = (self.root / "user-data").read_text()
                args = (self.root / "run-args").read_text()
                self.assertIn(f"Environment=RHO_INGEST_WORKER_ONLY={worker_only}", unit)
                self.assertIn(f"IngestRole,Value={role}", args)
                self.assertIn("sg-existing", args)
                self.assertIn("subnet-existing", args)

    def test_invalid_role_never_launches(self):
        result = self.run_launcher("unspecified", "up")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("INGEST_ROLE must be publisher or worker", result.stderr)
        self.assertFalse((self.root / "user-data").exists())

    def test_preflight_refuses_an_active_older_local_ingester(self):
        installed = self.root / "Library/Application Support/ECC2K130/ingest/dp_ingest.py"
        installed.parent.mkdir(parents=True)
        installed.write_text("older uncoordinated ingester\n")
        result = self.run_launcher("worker", "preflight", TEST_LOCAL_RUNNING="1")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("active local DP LaunchAgent runs a different ingester copy", result.stderr)
        self.assertFalse((self.root / "user-data").exists())

    def test_second_ec2_publisher_is_refused_before_upload(self):
        result = self.run_launcher("publisher", "preflight", TEST_EXISTING_PUBLISHER="1")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("EC2 publisher i-existing already exists", result.stderr)
        self.assertFalse((self.root / "user-data").exists())


if __name__ == "__main__":
    unittest.main()
