"""Document rendering only; no cryptographic computation or result generation."""
from pathlib import Path
from playwright.sync_api import sync_playwright

root = Path(__file__).resolve().parent
target = root / "report.pdf"
if target.exists():
    raise SystemExit("Refuse overwrite; retain the earlier PDF before rendering again.")
html = (root / "report.html").read_text()
svg = (root / "recovery-flow.svg").read_text().replace(
    "<svg ", '<svg style="width:100%;height:auto;max-height:108mm" ', 1
)
tag = '<img src="recovery-flow.svg" alt="Unknown-log relation collection, rank, descent and independent replay; development controls passed, production pilot not run">'
if tag not in html:
    raise SystemExit("Report diagram placeholder is missing.")
with sync_playwright() as api:
    browser = api.chromium.launch(
        executable_path="/usr/bin/chromium", headless=True,
        args=["--no-sandbox", "--disable-dev-shm-usage", "--disable-crash-reporter"],
    )
    page = browser.new_page()
    page.route("**/*", lambda route: route.abort())
    page.set_content(html.replace(tag, svg), wait_until="domcontentloaded")
    page.pdf(path=str(target), print_background=True, prefer_css_page_size=True)
    browser.close()
print("Rendered report.pdf; inspect every page before publication.")
