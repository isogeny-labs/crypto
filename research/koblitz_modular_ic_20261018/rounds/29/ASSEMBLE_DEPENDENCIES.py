#!/usr/bin/env python3
from pathlib import Path
import hashlib
here=Path(__file__).resolve().parent
rows=[('../28/runs/eval28-01/paired-image-0.sing',436,1),('runs/eval29-01/paired-degree12-1.sing',12,1),('../28/runs/eval28-01/paired-image-2.sing',284,2),('../28/runs/eval28-01/paired-image-3.sing',83,2)]
(here/'source-data').mkdir(exist_ok=True)
with (here/'source-data/dependencies-input-template.sing').open('w')as f:
 for case,(name,degree,kind)in enumerate(rows):
  f.write((here/name).read_text())
  f.write(f'int originalDegree29={degree};int imageCase29={case};int functionKind29={kind};\n')
  f.write((here/'IMAGEDEPENDENCIES29.sing').read_text())
 f.write('print("IMAGEDEPENDENCIES29_SCOPE original E49 closed-image dependencies only; no C5lift or newICrank claim");\nprint("IMAGEDEPENDENCIES29_ALL_CHECKS_PASS");quit;\n')
(here/'dependency-inputs.sha256').write_text(''.join(hashlib.sha256((here/name).read_bytes()).hexdigest()+'  '+name+'\n'for name,_,_ in rows))
