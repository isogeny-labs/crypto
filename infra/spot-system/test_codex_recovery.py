import json
import os
from pathlib import Path
import socket
import sqlite3
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import codex_recovery as recovery


class EnrollmentTests(unittest.TestCase):
    def test_empty_idle_tabs_are_omitted_but_missing_work_fails_closed(self):
        with tempfile.TemporaryDirectory() as tmp:
            local = Path(tmp)
            with sqlite3.connect(local/'state_5.sqlite') as db:
                db.execute('CREATE TABLE threads (id TEXT)')
                db.execute("INSERT INTO threads VALUES ('saved')")
            saved = {'id':'saved','source':'vscode','status':{'type':'active'}}
            empty = {'id':'empty','source':'vscode','status':{'type':'idle'}}
            manifest = {'files':{}}
            self.assertEqual(recovery.persisted_parents(local,[saved,empty],manifest),([saved],['empty']))
            for status in ['active','systemError']:
                with self.assertRaises(ValueError):
                    recovery.persisted_parents(local,[dict(empty,status={'type':status})],manifest)
            with self.assertRaises(ValueError):
                recovery.persisted_parents(local,[empty],{'files':{'sessions/rollout-empty.jsonl':{}}})

    def test_mounted_workspace_uses_directory_identity(self):
        with tempfile.TemporaryDirectory() as tmp:
            durable=Path(tmp)/'durable';workspace=durable/'home'
            local=Path(tmp)/'local';workspace.mkdir(parents=True);local.mkdir()
            with patch.object(recovery.os.path,'ismount',return_value=True):
                recovery.prepare_home(durable,local,home=workspace)
            self.assertEqual((workspace/'.codex').resolve(),local.resolve())

    def test_unrelated_mount_is_rejected_before_state_link(self):
        with tempfile.TemporaryDirectory() as tmp:
            durable=Path(tmp)/'durable';workspace=durable/'home'
            other=Path(tmp)/'other';local=Path(tmp)/'local'
            workspace.mkdir(parents=True);other.mkdir();local.mkdir()
            with patch.object(recovery.os.path,'ismount',return_value=True):
                with self.assertRaises(ValueError):
                    recovery.prepare_home(durable,local,home=other)
            self.assertFalse((other/'.codex').exists())

    def test_preserved_deadline_does_not_block_agent_recovery(self):
        for state in ['running', 'complete', 'deadline-expired']:
            self.assertTrue(recovery.dependency_ready({'state': state, 'observed': 100}, 101))
            self.assertFalse(recovery.dependency_ready({'state': state, 'observed': 1}, 101))
        for state in ['waiting-for-source', 'error']:
            self.assertFalse(recovery.dependency_ready({'state': state, 'observed': 100}, 101))
        self.assertFalse(recovery.dependency_ready({'state': 'running', 'observed': 102}, 101))

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
