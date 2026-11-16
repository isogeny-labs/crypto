"""Read-only independent full-field replay of the published program."""
from pathlib import Path
import json,sys
from verify_program import verify
D=Path(__file__).resolve().parent
path=Path(sys.argv[1]) if len(sys.argv)>1 else D/'certificate/program.json'
print(json.dumps(verify(json.loads(path.read_text()))))
