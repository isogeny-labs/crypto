"""Synthetic process and byte-writer checks; no research kernel execution."""
from pathlib import Path
import json
import os
import sys
import tempfile
import unittest
from bounded_process import run
from budget_io import DataBudget, OutputBudgetExceeded


class EnforcementTests(unittest.TestCase):
    def invoke(self, code, **limits):
        with tempfile.TemporaryDirectory() as directory:
            log = Path(directory)/'log'
            options = dict(wall_seconds=3, memory_bytes=128*1024**2, log_bytes=4096,
                           file_bytes=1024*1024)
            options.update(limits)
            receipt = run([sys.executable, '-c', code], directory, log, **options)
            return receipt, log.read_bytes()

    def test_child_observes_limits(self):
        receipt, data = self.invoke("import os,resource,json; print(json.dumps([sorted(os.sched_getaffinity(0)),resource.getrlimit(resource.RLIMIT_AS),resource.getrlimit(resource.RLIMIT_CPU),resource.getrlimit(resource.RLIMIT_FSIZE)]))")
        self.assertEqual(receipt['exit_code'], 0)
        affinity, memory, cpu, files = json.loads(data)
        self.assertEqual(affinity, [receipt['cpu']])
        self.assertEqual(memory, [128*1024**2]*2)
        self.assertEqual(cpu, [3,3])
        self.assertEqual(files, [1024*1024]*2)

    def test_memory_allocation_is_refused(self):
        receipt,data=self.invoke('x=bytearray(256*1024**2)')
        self.assertNotEqual(receipt['exit_code'],0)
        self.assertIn(b'MemoryError',data)

    def test_deadline_stops_sleeping_process(self):
        receipt,_=self.invoke('import time; time.sleep(30)',wall_seconds=0.2)
        self.assertEqual(receipt['exit_code'],124)
        self.assertEqual(receipt['stop_reason'],'deadline')
        self.assertFalse(Path('/proc',str(receipt['pid'])).exists())

    def test_log_cap_stops_writer(self):
        receipt,data=self.invoke("import os; os.write(1,b'x'*65536)",log_bytes=1024)
        self.assertEqual(receipt['exit_code'],125)
        self.assertEqual(len(data),1024)
        self.assertGreater(receipt['discarded_log_bytes'],0)

    def test_native_file_limit(self):
        receipt,_=self.invoke("open('large','wb').write(b'x'*8192)",file_bytes=1024)
        self.assertNotEqual(receipt['exit_code'],0)

    def test_aggregate_data_bytes_and_no_overwrite(self):
        with tempfile.TemporaryDirectory() as directory:
            budget=DataBudget(directory,limit=7)
            budget.write_text('a','α')
            with budget.open('b') as f:
                f.write('hello')
                self.assertEqual(f.tell(),5)
                with self.assertRaises(OutputBudgetExceeded): f.write('!')
            self.assertEqual(sum(p.stat().st_size for p in Path(directory).iterdir()),7)
            with self.assertRaises(FileExistsError):budget.write_text('a','x')
            with self.assertRaises(ValueError):budget.open('../outside')
            with self.assertRaises(OutputBudgetExceeded):DataBudget(directory,limit=6)


if __name__=='__main__':unittest.main()
