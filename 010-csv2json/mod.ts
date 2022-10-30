import { parse } from "https://deno.land/std@0.161.0/encoding/csv.ts";

const readInput = async (): Promise<string> => {
  const decoder = new TextDecoder();
  const decodedList: string[] = [];
  for await (const chunk of Deno.stdin.readable) {
    decodedList.push(decoder.decode(chunk));
  }
  return decodedList.join("");
};

const csvToJsonObject = (csv: string[][]): Record<string, unknown>[] => {
  const [header, ...rows] = csv;
  return rows.map((row) => {
    return row.reduce((acc, col, i) => {
      const prop = header[i];
      return { ...acc, [prop]: col };
    }, {});
  });
};

const input = await readInput();
const csv = parse(input);
if (csv.length === 0) {
  throw new Error("csv is empty");
}
const jsonObj = csvToJsonObject(csv);
const json = JSON.stringify(jsonObj);
console.log(json);
