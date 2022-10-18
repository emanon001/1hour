// get pages
// ref. https://scrapbox.io/help-jp/API
const project = "emanon001";
const response = await fetch(`https://scrapbox.io/api/pages/${project}`);
const json = await response.json();
const pages = json.pages;

// choose random page
const idx = Math.floor(Math.random() * pages.length);
const page = pages[idx];

// open page (only for macOS)
const url = `https://scrapbox.io/${project}/${encodeURIComponent(page.title)}`;
const p = Deno.run({
  cmd: ["open", url],
});
await p.status();

const pageInfo = { title: page.title, url };
console.log(JSON.stringify(pageInfo, null, 2));
