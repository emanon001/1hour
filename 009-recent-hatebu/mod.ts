import {
  DOMParser,
  Element,
} from "https://deno.land/x/deno_dom@v0.1.35-alpha/deno-dom-wasm.ts";

const fetchHatebuHtml = async (id: string): Promise<string> => {
  const url = `https://b.hatena.ne.jp/${id}/bookmark`;
  const resp = await fetch(url);
  return resp.text();
};

type HatebuEntry = {
  url: string;
  text: string;
};
const parseHatebuHtml = (html: string): HatebuEntry[] => {
  const doc = new DOMParser().parseFromString(html, "text/html")!;
  const entryList = Array.from(
    doc.querySelectorAll(".bookmark-item"),
  ) as Element[];
  return entryList.map((elm) => {
    const titleElem = elm.querySelector(".centerarticle-entry-title a")!;
    const url = titleElem.getAttribute("href")!;
    const text = titleElem.textContent;
    return { url, text };
  });
};

const printHatebuEntryList = (entryList: HatebuEntry[]) => {
  entryList.forEach((ent) => {
    console.log(`${ent.text} / ${ent.url}`);
  });
};

const hatebuId = "emanon001";
const html = await fetchHatebuHtml(hatebuId);
const entryList = parseHatebuHtml(html);
printHatebuEntryList(entryList);
