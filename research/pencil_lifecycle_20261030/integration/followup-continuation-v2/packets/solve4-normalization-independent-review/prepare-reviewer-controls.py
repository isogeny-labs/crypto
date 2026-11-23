from pathlib import Path
import re,json,hashlib
D=Path(Path('/var/tmp/normalization-review-location').read_text());S=D/'source'
(S/'ReviewerControls.lean').write_text('''import Controls
set_option autoImplicit false
namespace NormalizationReviewer
open Solve4Stage Solve4Normalize
theorem first_column_all_entries : wordScale 7 0 (fun j => (#[3,6,0,0,2]:Array UInt64)[j.val]!) 5 = (fun j => (#[1,2,0,0,3]:Array UInt64)[j.val]!) := by decide
theorem last_column_includes_rhs : wordScale 7 3 (fun j => (#[0,0,0,3,6]:Array UInt64)[j.val]!) 5 = (fun j => (#[0,0,0,1,2]:Array UInt64)[j.val]!) := by decide
theorem characteristic_two_word_case : wordScale 2 0 (fun j => (#[1,1,0,0,1]:Array UInt64)[j.val]!) 1 = (fun j => (#[1,1,0,0,1]:Array UInt64)[j.val]!) := by decide
end NormalizationReviewer
''')
names=[]
for f,ns in [('Normalization','Solve4Normalize'),('Controls','NormalizationControls'),('ReviewerControls','NormalizationReviewer')]:names += [ns+'.'+n for n in re.findall(r'^theorem (\w+)',(S/(f+'.lean')).read_text(),re.M)]
(S/'Readback.lean').write_text('import ReviewerControls\n'+''.join('#print axioms '+n+'\n' for n in names));(D/'expected-names.json').write_text(json.dumps(names,indent=2)+'\n')
