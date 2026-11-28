#!/usr/bin/env python3
"""PDF rendering/inspection support only; no research calculations."""
from pathlib import Path
import hashlib
import json
import re
import subprocess
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parent
OUT = Path('/tmp/crypto-pursue/pdf-inspection')
OUT.mkdir(parents=True, exist_ok=True)
pages = []
receipts = []
pdfs = [ROOT / 'REPORT.pdf', *sorted((ROOT / 'reports').glob('*/REPORT.pdf'))]
for i, pdf in enumerate(pdfs):
    info = subprocess.check_output(['pdfinfo', str(pdf)], text=True)
    count = int(re.search(r'^Pages:\s+(\d+)', info, re.M)[1])
    text = subprocess.check_output(['pdftotext', '-layout', str(pdf), '-'], text=True)
    assert len(text.strip()) > 200 and '\ufffd' not in text, pdf
    md = pdf.with_suffix('.md').read_text()
    session = re.search(r'Session `(ECBS1h[0-9a-f]+)`', md)
    if session:
        assert session[1] in text and 'Requested' in text and 'recorded' in text, pdf
    prefix = OUT / f'doc-{i:02d}'
    subprocess.run(['pdftoppm', '-scale-to', '1000', '-png', str(pdf), str(prefix)],
                   check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    images = sorted(OUT.glob(prefix.name + '-*.png'))
    assert len(images) == count, pdf
    for n, image in enumerate(images, 1):
        pages.append((str(pdf.relative_to(ROOT)) + f' p{n}', image))
    receipts.append({'path': str(pdf.relative_to(ROOT)), 'sha256': hashlib.sha256(pdf.read_bytes()).hexdigest(),
                     'pages': count, 'text_extracted': True, 'session_identity_present': bool(session)})
for start in range(0, len(pages), 6):
    canvas = Image.new('RGB', (1560, 1450), 'white')
    draw = ImageDraw.Draw(canvas)
    for j, (label, path) in enumerate(pages[start:start+6]):
        x, y = (j % 3) * 520, (j // 3) * 725
        with Image.open(path) as im:
            im.thumbnail((510, 685))
            canvas.paste(im, (x, y + 30))
        draw.text((x+4,y+3), label, fill='black')
    canvas.save(OUT / f'contact-{start//6:02d}.png')
(ROOT / 'PDF_INSPECTION.json').write_text(json.dumps({
    'method': 'Extract every page text; check identities; render every page; contact-sheet visual review plus full-size main report.',
    'pdfs': receipts, 'total_pages': len(pages),
    'visual_review': 'pending assistant inspection'}, indent=2) + '\n')
print(json.dumps({'pdfs': len(pdfs), 'pages': len(pages), 'contact_sheets': (len(pages)+5)//6}))
