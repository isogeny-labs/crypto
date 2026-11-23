"""Single-writer aggregate data budget; reservations precede every write.

Use a fresh owned runs directory. This controls routed artifact writes, not
arbitrary filesystem writes by untrusted programs. Failed writes keep charges.
"""
from pathlib import Path

DATA_LIMIT = 4 * 1024**3 - 32 * 1024**2


class OutputBudgetExceeded(Exception):
    pass


class DataBudget:
    def __init__(self, root, limit=DATA_LIMIT):
        self.root = Path(root).resolve()
        self.limit = limit
        self.used = 0
        if self.root.exists():
            for p in self.root.rglob('*'):
                if p.is_symlink():
                    raise ValueError('symlink in output directory')
                if p.is_file(): self.used += p.stat().st_size
        self.reserve(0)

    def reserve(self, count):
        if count < 0 or self.used + count > self.limit:
            raise OutputBudgetExceeded('aggregate data budget exceeded')
        self.used += count

    def open(self, name):
        # Output names are fixed local filenames, not arbitrary paths.
        if Path(name).name != name or name in ('', '.', '..'):
            raise ValueError('output must be a local filename')
        return BudgetFile(self, self.root / name)

    def write_text(self, name, text):
        with self.open(name) as f:
            f.write(text)


class BudgetFile:
    def __init__(self, budget, path):
        self.budget = budget
        self.file = path.open('xb')

    def write(self, text):
        data = text.encode('utf-8')
        self.budget.reserve(len(data))
        written = self.file.write(data)
        if written != len(data): raise OSError('short artifact write')
        return len(text)

    def tell(self): return self.file.tell()
    def flush(self): return self.file.flush()
    def __enter__(self): return self
    def __exit__(self, *args): self.file.close()
