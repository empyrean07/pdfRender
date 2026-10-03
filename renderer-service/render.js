import { chromium } from 'playwright';
import path from 'path';
import fs from 'fs';

async function main() {
  const args = process.argv.slice(2);
  if (args.length < 2) {
    console.error('Usage: node render.js <htmlPath> <outputPath>');
    process.exit(1);
  }

  const htmlPath = path.resolve(args[0]);
  const outputPath = path.resolve(args[1]);

  if (!fs.existsSync(htmlPath)) {
    console.error(`HTML file not found: ${htmlPath}`);
    process.exit(1);
  }

  console.log(`[PDF RENDER] Rendering HTML to PDF: ${htmlPath} -> ${outputPath}`);

  let browser;
  try {
    browser = await chromium.launch({
      headless: true,
      args: ['--no-sandbox', '--disable-setuid-sandbox', '--allow-file-access-from-files']
    });

    const page = await browser.newPage();
    
    const fileUrl = `file:///${htmlPath.replace(/\\/g, '/')}`;
    console.log(`[PDF RENDER] Navigating to: ${fileUrl}`);

    await page.goto(fileUrl, { waitUntil: 'networkidle', timeout: 30000 });

    // Wait for fonts and all image loading
    await page.evaluate(async () => {
      await document.fonts.ready;
      const images = Array.from(document.querySelectorAll('img'));
      await Promise.all(
        images.map((img) => {
          if (img.complete) return Promise.resolve();
          return new Promise((resolve) => {
            img.onload = resolve;
            img.onerror = resolve; // Continue even if an image fails to load
          });
        })
      );
    });

    // Ensure CSS print styles are evaluated in print media context
    await page.emulateMedia({ media: 'print' });

    // Render A4 PDF with background graphics enabled
    await page.pdf({
      path: outputPath,
      format: 'A4',
      printBackground: true,
      preferCSSPageSize: true,
      displayHeaderFooter: false
    });

    console.log(`[PDF RENDER] PDF successfully created at: ${outputPath}`);
    await browser.close();
    process.exit(0);
  } catch (err) {
    console.error(`[PDF RENDER ERROR] ${err.message}`, err);
    if (browser) await browser.close();
    process.exit(1);
  }
}

main();
