"""Document rendering only: lossless PDF object/stream compression."""
from pathlib import Path
import fitz,json,hashlib
p=Path('report.pdf')
before=p.read_bytes()
doc=fitz.open(p)
doc.save('report.compact.pdf',garbage=4,deflate=True,deflate_fonts=True,use_objstms=1)
doc.close()
after=Path('report.compact.pdf').read_bytes()
if len(after)<len(before):
    Path('report.compact.pdf').replace(p)
else:
    Path('report.compact.pdf').unlink()
after=p.read_bytes()
Path('pdf-compact.json').write_text(json.dumps({'method':'PyMuPDF lossless object/stream compression; garbage4/deflate/deflate_fonts/use_objstms1; keep smaller file. No rasterization or image resampling.','original_bytes':len(before),'final_bytes':len(after),'final_sha256':hashlib.sha256(after).hexdigest()},indent=2)+'\n')
