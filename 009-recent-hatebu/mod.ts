const fetchHatebuHtml = async (id: string): Promise<string> => {
  const url = `https://b.hatena.ne.jp/${id}/bookmark`;
  const resp = await fetch(url);
  return resp.text();
};

// TODO: parse HTML
type HatebuEntry = {};
const parseHatebuHtml = (html: string): HatebuEntry[] => {
  return [];
};

// TODO: print hatebu entry list
const printHatebuEntryList = (entryList: HatebuEntry[]) => {
};

const hatebuId = "emanon001";
const html = await fetchHatebuHtml(hatebuId);
const entryList = parseHatebuHtml(html);
printHatebuEntryList(entryList);
