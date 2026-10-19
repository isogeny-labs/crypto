import json
import os
from pathlib import Path
import socket
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import codex_recovery as recovery


class EnrollmentTests(unittest.TestCase):
    def test_readiness_is_explicitly_sent_to_systemd(self):
        with tempfile.TemporaryDirectory() as tmp:
            path=str(Path(tmp)/'notify.sock')
            with socket.socket(socket.AF_UNIX,socket.SOCK_DGRAM) as server:
                server.bind(path)
                server.settimeout(1)
                with patch.dict(os.environ,{'NOTIFY_SOCKET':path}):
                    recovery.notify_ready()
                self.assertEqual(server.recv(256),b'READY=1')

    def test_only_active_unblocked_parent_chats_are_eligible(self):
        self.assertTrue(recovery.resumable({'source':'vscode','status':{'type':'active','activeFlags':[]}}))
        for status in ['idle','systemError','notLoaded']:
            self.assertFalse(recovery.resumable({'source':'vscode','status':{'type':status}}))
        for flag in ['waitingOnApproval','waitingOnUserInput']:
            self.assertFalse(recovery.resumable({'source':'vscode','status':{'type':'active','activeFlags':[flag]}}))
        self.assertFalse(recovery.resumable({'source':{'subAgent':{}},'status':{'type':'active'}}))

    def test_incomplete_enrollment_never_reaches_aws_or_starts_processes(self):
        complete={'schema':'spot.codex-migration/v1','enabled':True,
                  'authentication_transfer_authorized':True,
                  'resume_active_threads_authorized':True,'workspace_verified':True}
        with tempfile.TemporaryDirectory() as tmp:
            path=Path(tmp)/'agent.json'
            for key in complete:
                config=dict(complete);config.pop(key)
                path.write_text(json.dumps(config))
                with patch.object(recovery,'instance_id') as identity, patch.object(recovery.subprocess,'Popen') as spawn:
                    with self.assertRaises(ValueError):
                        recovery.main(SimpleNamespace(config=path))
                    identity.assert_not_called();spawn.assert_not_called()


if __name__ == '__main__':
    unittest.main()
