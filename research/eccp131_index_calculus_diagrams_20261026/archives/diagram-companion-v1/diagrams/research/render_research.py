"""Render research figures and exercise the three complexity comparison modes."""

import asyncio
import os
import shutil
import json
from pathlib import Path
import xml.etree.ElementTree as ET
import zipfile

from playwright.async_api import async_playwright


async def main():
    directory=Path(__file__).resolve().parent
    figures=json.loads((directory/"figures.json").read_text())
    errors=[]
    async with async_playwright() as p:
        browser=await p.chromium.launch(executable_path=os.environ.get("CHROMIUM_EXECUTABLE") or shutil.which("chromium"),args=["--no-sandbox"])
        page=await browser.new_page(viewport={"width":1600,"height":1040},device_scale_factor=1)
        page.on("pageerror",lambda error:errors.append(str(error)))
        for figure in figures:
            source=(directory/(figure["name"]+".svg")).read_text()
            root=ET.fromstring(source)
            identifiers=[node.attrib['id'] for node in root.iter() if 'id' in node.attrib]
            assert len(identifiers)==len(set(identifiers)),figure['name']
            for node in root.iter():
                for key,value in node.attrib.items():
                    if key.endswith('}href') and value.startswith('#'):
                        assert value[1:] in identifiers,(figure['name'],value)
            await page.set_content('<html><body style="margin:0">'+source+'</body></html>')
            await page.screenshot(path=str(directory/(figure["name"]+".png")))
            outliers=await page.locator("svg text").evaluate_all("nodes=>nodes.map(n=>({text:n.textContent,r:n.getBoundingClientRect()})).filter(o=>o.r.width>0&&(o.r.left < -1 || o.r.right > 1601 || o.r.top < -1 || o.r.bottom > 1041)).map(o=>o.text)")
            assert not outliers,(figure["name"],outliers)
        await page.set_content((directory/"index.html").read_text())
        assert await page.locator("#rhoText3").text_content()=="2^65.1"
        assert await page.locator("#icText3").text_content()=="2^130.2 †"
        await page.select_option("#mode","field")
        assert await page.locator("#rhoText3").text_content()=="2^195.3"
        assert await page.locator("#icText2").text_content()=="2^173.6"
        await page.select_option("#mode","group")
        assert await page.locator("#rhoText3").text_content()=="2^65.1"
        assert await page.locator("#icText3").text_content()=="2^43.4"
        async with page.expect_download() as download_info:
            await page.click("#saveCost")
        download=await download_info.value
        await download.save_as(str(directory/"06-complexity-equal-groups.svg"))
        exported=directory/"06-complexity-equal-groups.svg"
        exported.write_text("\n".join(line.rstrip() for line in exported.read_text().splitlines())+"\n")
        await page.select_option("#mode","field")
        await page.locator("#bits").fill("160")
        await page.locator("#bits").dispatch_event("input")
        assert await page.locator("#rhoText3").text_content()=="2^240.0"
        bound=await page.locator("#rhoText3").evaluate("el=>el.getBoundingClientRect().right-document.querySelector('#figure6').getBoundingClientRect().left")
        assert bound<1600
        await page.locator("#bits").fill("130.2")
        await page.locator("#bits").dispatch_event("input")
        await page.select_option("#mode","embedded")
        await page.select_option('#rankMode','dependent')
        assert await page.locator('#rankA3').text_content()=='406'
        assert await page.locator('#rankB3').text_content()=='5'
        assert await page.locator('#rankAugmented').text_content()=='rank [M | −b] = 2'
        assert 'do not determine k' in await page.locator('#rankConclusion').text_content()
        await page.select_option('#rankMode','usable')
        assert '37' in await page.locator('#rankConclusion').text_content()
        assert await page.locator('#modelAlternativeValue').text_content()=='200.0'
        await page.locator('#trialCost').fill('1')
        await page.locator('#trialCost').dispatch_event('input')
        assert await page.locator('#modelAlternativeValue').text_content()=='50.0'
        assert '0.50×' in await page.locator('#modelRatio').text_content()
        await page.locator('#infoYield').fill('25')
        await page.locator('#infoYield').dispatch_event('input')
        assert await page.locator('#modelAlternativeValue').text_content()=='100.0'
        await page.locator('#trialCost').fill('4')
        await page.locator('#trialCost').dispatch_event('input')
        await page.locator('#infoYield').fill('50')
        await page.locator('#infoYield').dispatch_event('input')
        assert await page.locator('#modelAlternativeValue').text_content()=='200.0'
        await page.pdf(path=str(directory/"research-diagrams.pdf"),print_background=True,prefer_css_page_size=True)
        assert not errors,errors
        await browser.close()
    # A numeric check of the exact left-kernel extraction identity in Figure 1.
    n,k=101,37
    logs=[11,23]
    matrix=[[1,0],[0,1],[1,1]]
    b=[2,3,7]
    a=[(sum(x*y for x,y in zip(row,logs))-k*beta)%n for row,beta in zip(matrix,b)]
    w=[-1,-1,1]
    assert all(sum(w[i]*matrix[i][j] for i in range(3))%n==0 for j in range(2))
    assert -sum(x*y for x,y in zip(w,a))*pow(sum(x*y for x,y in zip(w,b))%n,-1,n)%n==k
    with zipfile.ZipFile(directory/"research-diagrams.zip","w",zipfile.ZIP_DEFLATED) as archive:
        for path in sorted(directory.iterdir()):
            if path.suffix in {".svg",".png",".html",".pdf",".py",".tex",".json",".csv"}:
                archive.write(path,path.name)
    print(f"Rendered {len(figures)} PNGs and a PDF; complexity modes, rank toggle, collection sliders and SVG export passed.")
    print("Checked SVG references, text bounds and numerical algebra; wrote research-diagrams.zip.")


if __name__=="__main__":
    asyncio.run(main())
