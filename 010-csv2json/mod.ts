import { parse } from "https://deno.land/std@0.161.0/encoding/csv.ts";
import { readAllSync } from "https://deno.land/std@0.161.0/streams/conversion.ts";

const readInput = (): string => {
  const content = readAllSync(Deno.stdin);
  const decoder = new TextDecoder();
  return decoder.decode(content);
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

const input = readInput();
const csv = parse(input);
if (csv.length === 0) {
  throw new Error("csv is empty");
}
const jsonObj = csvToJsonObject(csv);
const json = JSON.stringify(jsonObj);
console.log(json);
