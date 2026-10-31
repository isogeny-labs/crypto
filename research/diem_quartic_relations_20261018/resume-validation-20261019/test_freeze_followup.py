import copy
import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location('followup', Path(__file__).with_name('diem-freeze-followup.py'))
followup = importlib.util.module_from_spec(spec)
spec.loader.exec_module(followup)


class ContinuationGates(unittest.TestCase):
    def setUp(self):
        self.conflict = dict(holder_task_ref='T-7', holder_task_id='expected-id',
                             holder_owner='ec2-user', holder_status='ready',
                             kind='scope_overlap', holder_mode='write_exclusive',
                             requested='dir:home/ec2-user/w3b', resource='dir:home/ec2-user/w3b')
        self.decision = dict(outcome='block_conflict', conflicts=[self.conflict])

    def allowed(self, decision):
        return followup.owns_reservation(decision, 'T-7', 'expected-id', 'ec2-user', '/home/ec2-user/w3b')

    def test_clear_and_exact_reservation(self):
        self.assertTrue(self.allowed({'outcome': 'allow'}))
        self.assertTrue(self.allowed(self.decision))
        self.conflict['holder_status'] = 'claimed'
        self.assertTrue(self.allowed(self.decision))

    def test_every_identity_and_scope_field_is_required(self):
        for key in self.conflict:
            with self.subTest(key=key):
                d = copy.deepcopy(self.decision)
                d['conflicts'][0][key] = 'different'
                self.assertFalse(self.allowed(d))
                del d['conflicts'][0][key]
                self.assertFalse(self.allowed(d))

    def test_mixed_holder_and_duplicate_fail_closed(self):
        d = copy.deepcopy(self.decision)
        d['conflicts'].append(dict(self.conflict, holder_task_id='other-task'))
        self.assertFalse(self.allowed(d))
        d = copy.deepcopy(self.decision)
        d['duplicates'] = [{'task_id': 'other-task'}]
        self.assertFalse(self.allowed(d))

    def test_ambiguous_decisions_fail_closed(self):
        for d in [{}, {'outcome': 'allow_with_warning'}, {'outcome': 'block_conflict'},
                  {'outcome': 'allow', 'conflicts': [self.conflict]},
                  {'outcome': 'suggest_join'}, {'outcome': 'block_duplicate'}]:
            with self.subTest(decision=d):
                self.assertFalse(self.allowed(d))

    def test_freeze_gate_still_requires_all_three_conditions(self):
        self.assertTrue(followup.complete_gate({'status':'generation_complete'}, 52, False))
        for status, count, live in [('generating',52,False), ('generation_complete',46,False),
                                    ('generation_complete',52,True), ('failed',52,False)]:
            self.assertFalse(followup.complete_gate({'status':status}, count, live))


if __name__ == '__main__':
    unittest.main()
