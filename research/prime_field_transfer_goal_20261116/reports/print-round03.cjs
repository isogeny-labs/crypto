// Document rendering only. Run from this directory after pandoc.
const fs=require('fs');
const {chromium}=require(process.env.PLAYWRIGHT_MODULE||'playwright');
(async()=>{
  let browser;
  try {
    browser=await chromium.launch({executablePath:process.env.CHROMIUM_BIN||'/usr/bin/chromium',
      headless:true,args:['--no-sandbox']});
    const page=await browser.newPage();
    let html=fs.readFileSync('round-03.html','utf8').replace(
      '<link rel="stylesheet" href="round-03.css" />',
      '<style>'+fs.readFileSync('round-03.css','utf8')+'</style>');
    html=html.replace(/<img src="(round-03-[^"]+\.svg)"[^>]*\/>/g,
      (_,p)=>fs.readFileSync(p,'utf8'));
    await page.setContent(html,{waitUntil:'load'});
    await page.pdf({path:'round-03.pdf',preferCSSPageSize:true,printBackground:true});
  } finally {if(browser)await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
