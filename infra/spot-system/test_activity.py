import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import MagicMock, patch
from activity import collect


class ActivityTests(unittest.TestCase):
    def test_unregistered_user_experiment_keeps_worker_busy(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d)
            process=root/'proc/99999999'
            process.mkdir(parents=True)
            (process/'comm').write_text('long_calculation\n')
            (process/'status').write_text('Uid:\t'+str(os.getuid())+'\n')
            (process/'cmdline').write_bytes(b'/work/long_calculation\0')
            with patch('activity.urllib.request.build_opener') as net:
                net.return_value.open.side_effect=OSError('node unavailable')
                self.assertTrue(collect(root,root/'nodes',root/'proc')['Busy'])

    def test_missing_manifest_does_not_hide_legacy_workers(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            process = root/'proc/123'
            process.mkdir(parents=True)
            (process/'comm').write_text('factor_base_shape_ladder\n')
            with patch('activity.urllib.request.build_opener') as net:
                net.return_value.open.side_effect = OSError('node unavailable')
                result = collect(root, root/'nodes', root/'proc')
            self.assertTrue(result['Busy'])
            self.assertTrue(result['Unknown'])

    def test_real_work_queues_count_while_node_daemons_do_not(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root/'proc').mkdir()
            (root/'jobs.json').write_text('[]')
            for name in ['queue','running']:
                (root/'nodes/agent/jobs'/name).mkdir(parents=True)
            response = MagicMock()
            response.__enter__.return_value.status = 200
            with patch('activity.urllib.request.build_opener') as net, \
                    patch('activity.subprocess.check_output', return_value='0\n'), \
                    patch('activity.subprocess.run'):
                net.return_value.open.return_value = response
                self.assertEqual(collect(root, root/'nodes', root/'proc'),
                                 {'Busy':False, 'Pending':0, 'Unknown':False})
                (root/'nodes/agent/jobs/queue/job.json').write_text('{}')
                queued = collect(root, root/'nodes', root/'proc')
                self.assertTrue(queued['Busy'])
                self.assertEqual(queued['Pending'], 1)

    def test_database_probe_failure_blocks_shutdown(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root/'proc').mkdir()
            (root/'jobs.json').write_text('[]')
            for name in ['queue','running']:
                (root/'nodes/agent/jobs'/name).mkdir(parents=True)
            response = MagicMock()
            response.__enter__.return_value.status = 200
            with patch('activity.urllib.request.build_opener') as net, \
                    patch('activity.subprocess.check_output', side_effect=OSError('DB down')):
                net.return_value.open.return_value = response
                self.assertTrue(collect(root, root/'nodes', root/'proc')['Unknown'])


if __name__ == '__main__':
    unittest.main()
