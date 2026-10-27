"""Export the SVGs to PNG/PDF and check the interactive comparison controls."""

import asyncio
import os
import shutil
from pathlib import Path
import re

from playwright.async_api import async_playwright


async def main():
    directory = Path(__file__).resolve().parent
    async with async_playwright() as p:
        browser = await p.chromium.launch(executable_path=os.environ.get("CHROMIUM_EXECUTABLE") or shutil.which("chromium"),args=["--no-sandbox"])
        page = await browser.new_page(viewport={"width":1440,"height":940},device_scale_factor=1)
        errors=[]
        page.on("pageerror",lambda error: errors.append(str(error)))
        for name in ["01-workflow","02-smoothness","03-cover","04-costs"]:
            svg=(directory/(name+".svg")).read_text()
            await page.set_content('<html><body style="margin:0">'+svg+'</body></html>')
            await page.screenshot(path=str(directory/(name+".png")))
        html=(directory/"index.html").read_text()
        html=re.sub(r'<img src="([^"]+)"[^>]*>',lambda match:(directory/match.group(1)).read_text(),html)
        await page.set_content(html)
        await page.select_option("#mode","group")
        assert await page.locator("#rhoText0").text_content()=="2^65.1"
        assert await page.locator("#icText3").text_content()=="2^43.4"
        async with page.expect_download() as download_info:
            await page.click("#saveCost")
        download=await download_info.value
        await download.save_as(str(directory/"04-costs-equal-groups.svg"))
        await page.select_option("#mode","field")
        await page.locator("#bits").fill("160")
        await page.locator("#bits").dispatch_event("input")
        assert await page.locator("#rhoText3").text_content()=="2^240.0"
        bounds=await page.locator("#rhoText3").evaluate("el=>({x:el.getBBox().x,width:el.getBBox().width})")
        assert bounds["x"]+bounds["width"]<1384
        await page.locator("#bits").fill("130.2")
        await page.locator("#bits").dispatch_event("input")
        await page.pdf(path=str(directory/"index-calculus-diagrams.pdf"),print_background=True,prefer_css_page_size=True)
        assert not errors,errors
        await browser.close()
    print("Rendered four PNGs and a PDF; checked both cost modes, the slider limit, and SVG download.")


if __name__=="__main__":
    asyncio.run(main())
