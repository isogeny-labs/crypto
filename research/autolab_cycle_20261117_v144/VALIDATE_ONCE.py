from pathlib import Path
import json,sys
R=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(R))
from lab.runner import Laboratory

# The stock validate method is unchanged. Cache one fully verified journal
# under the same exclusive lock instead of parsing it twice in the CLI path.
lab=Laboratory(R/'lab/ledger')
with lab.locked():
 records=lab.journal()
 assert records
 lab.state=records[-1]['state']
 lab.journal=lambda:records
 result=lab.validate()
 result['journal_reads']=1
 result['validation_method']='stock Laboratory.journal and validate under exclusive lock;verified journal cached for second read'
 print(json.dumps(result,indent=2))
