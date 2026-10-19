"""Layout only: draw the recorded native-import metadata counts, not research computations."""
from pathlib import Path
import csv
from xml.sax.saxutils import escape

root = Path(__file__).parent
with (root / "review-20261019-counts.tsv").open() as source:
    rows = list(csv.DictReader(source, delimiter="\t"))
parts = [
    '<svg xmlns="http://www.w3.org/2000/svg" width="1050" height="580" viewBox="0 0 1050 580">',
    '<rect width="1050" height="580" fill="white"/>',
    '<style>text{font-family:DejaVu Sans,sans-serif;fill:#172b42;font-size:18px}.head{font-size:23px;font-weight:bold}.small{font-size:15px}</style>',
    '<text x="28" y="34" class="head">Genus3Covers ingestion review · 2026-10-19</text>',
]
for x, color, title, first, second in [
    (28, "#e5effa", "Pinned upstream snapshot", "221e74b7097a… · MIT", "9 exact source/data blobs"),
    (372, "#e4f3ea", "Native representation reader", "654 candidate rows imported", "Zero field polynomials rejected"),
    (716, "#fff0d8", "Geometry and ECDLP open", "No independent Magma run", "No checked map or cost claim"),
]:
    parts += [f'<rect x="{x}" y="58" width="308" height="106" rx="10" fill="{color}" stroke="#63788c"/>']
    for dy, label in [(29, title), (58, first), (85, second)]:
        parts.append(f'<text x="{x+13}" y="{58+dy}" class="small">{escape(label)}</text>')
parts += [
    '<path d="M339 111h28m316 0h28" stroke="#63788c" stroke-width="3"/>',
    '<text x="28" y="204" class="head">Pinned P2example.txt: recorded candidate-row counts</text>',
    '<text x="28" y="231" class="small">Population: all 654 file rows · representation only · exact inventory, no sampling interval</text>',
]
for i, row in enumerate(rows):
    count = int(row["rows"])
    y = 253 + 43 * i
    label = f'Type {row["cover_type"]}, genus {row["genus"]}'
    parts += [
        f'<text x="28" y="{y+20}">{escape(label)}</text>',
        f'<rect x="236" y="{y}" width="{count/400*680:.2f}" height="27" fill="#3978ad"/>',
        f'<text x="{245+count/400*680:.2f}" y="{y+21}">{count}</text>',
    ]
parts.append('<path d="M236 516H916" stroke="#63788c"/>')
for count in range(0, 401, 100):
    x = 236 + count / 400 * 680
    parts += [f'<path d="M{x} 516v5" stroke="#63788c"/>',
              f'<text x="{x}" y="538" text-anchor="middle" class="small">{count}</text>']
parts += ['<text x="576" y="555" text-anchor="middle" class="small">Imported candidate rows</text>',
          '<text x="28" y="569" class="small">Evidence: review-20261019-counts.tsv · upstream SHA-256/blob receipts · REVIEW-20261019.md</text>',
          '</svg>']
(root / "review-20261019.svg").write_text("\n".join(parts) + "\n")
