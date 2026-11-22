// Document rendering only, following the existing round03 renderer.
const fs=require('fs'),{chromium}=require(process.env.PLAYWRIGHT_MODULE||'playwright');
(async()=>{
  let browser;
  try{
    browser=await chromium.launch({executablePath:process.env.CHROMIUM_BIN||'/usr/bin/chromium',headless:true,args:['--no-sandbox']});
    const page=await browser.newPage();
    let html=fs.readFileSync('round-05.html','utf8').replace('<link rel="stylesheet" href="round-05.css" />',
      '<style>'+fs.readFileSync('round-05.css','utf8')+'</style>');
    html=html.replace(/<img src="(round-05-[^"]+\.svg)"[^>]*\/>/g,(_,p)=>fs.readFileSync(p,'utf8'));
    await page.setContent(html,{waitUntil:'load'});
    await page.pdf({path:'round-05.pdf',preferCSSPageSize:true,printBackground:true});
  }finally{if(browser)await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
