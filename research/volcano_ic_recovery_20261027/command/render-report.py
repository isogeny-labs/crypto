#!/usr/bin/env python3
"""Document rendering only; no arithmetic, research analysis or solver calls."""
from pathlib import Path
import sys
from playwright.sync_api import sync_playwright

here = Path(__file__).resolve().parent
assert len(sys.argv) == 2, 'usage: render-report.py NEW_PDF_PATH'
target = Path(sys.argv[1]).resolve()
assert not target.exists(), 'preserve the previous rendered artifact'
html = (here / 'report.html').read_text()
html = html.replace('<img src="flow.svg" alt="Original public source to local image to small-fixture replay; live recovery remains pending">', (here / 'flow.svg').read_text())
with sync_playwright() as p:
    browser = p.chromium.launch(executable_path='/usr/bin/chromium', headless=True, args=['--no-sandbox'])
    page = browser.new_page()
    page.route('**/*', lambda route: route.abort())
    page.set_content(html, wait_until='load')
    page.pdf(path=str(target), prefer_css_page_size=True, print_background=True)
    browser.close()
