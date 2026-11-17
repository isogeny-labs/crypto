"""Inspect document bounds and attachment bytes; no research computation."""
import hashlib
import json
import sys
from pathlib import Path

import fitz

root = Path(__file__).resolve().parent.parent
preview = Path(sys.argv[1])
preview.mkdir(parents=True, exist_ok=False)
records = []
pdfs = [root / 'REPORT.pdf', *sorted((root / 'reports').glob('*/REPORT.pdf'))]
for pdf in pdfs:
    doc = fitz.open(pdf)
    label = 'main' if pdf.parent == root else pdf.parent.name
    pages = []
    for index, page in enumerate(doc):
        assert len(page.get_text().strip()) > 50, (pdf, index, 'empty page')
        for block in page.get_text('dict')['blocks']:
            for line in block.get('lines', []):
                for span in line['spans']:
                    x0, y0, x1, y1 = span['bbox']
                    assert x0 >= 10 and y0 >= 8 and x1 <= page.rect.width - 10 and y1 <= page.rect.height - 8, (pdf, index, span)
        name = f'{label}-{index + 1}.png'
        page.get_pixmap(dpi=110).save(preview / name)
        pages.append(name)
    attachments = []
    for name in doc.embfile_names():
        info = doc.embfile_info(name)
        path = (root / info['description']).resolve()
        assert path.is_relative_to(root)
        data = doc.embfile_get(name)
        assert data == path.read_bytes(), (pdf, name, 'attachment mismatch')
        attachments.append({'name': name, 'path': str(path.relative_to(root)), 'bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest()})
    blob = pdf.read_bytes()
    records.append({'path': str(pdf.relative_to(root)), 'bytes': len(blob), 'sha256': hashlib.sha256(blob).hexdigest(), 'pages': len(doc), 'preview_pages': pages, 'attachments': attachments})
    doc.close()
receipt = {'kind': 'pdf-document-admission/v1', 'document_bounds_and_attachment_bytes_verified': True, 'visual_review': 'pending; open every rendered page', 'documents': records}
(root / 'evidence/pdf-admission.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps({'documents': len(records), 'pages': sum(r['pages'] for r in records), 'main_attachments': len(records[0]['attachments']), 'preview': str(preview)}))
