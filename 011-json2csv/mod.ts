import { stringify } from "https://deno.land/std@0.161.0/encoding/csv.ts";
import { isPlainObject } from "https://deno.land/x/is_what@v4.1.7/src/index.ts";
import { readAllSync } from "https://deno.land/std@0.161.0/streams/conversion.ts";

function readInput(): string {
  const content = readAllSync(Deno.stdin);
  const decoder = new TextDecoder();
  return decoder.decode(content);
}

function jsonToCsv(json: any): string {
  // check type
  if (!Array.isArray(json)) {
    throw new Error("JSON is not array");
  }
  if (!json.every(isPlainObject)) {
    throw new Error("JSON item is not object");
  }

  // get keys
  const keySet = new Set<string>();
  json.forEach((obj) => {
    Object.keys(obj).forEach((k) => keySet.add(k));
  });
  const columns = Array.from(keySet);

  // convert
  const data = json.map((obj) => {
    return columns.reduce((row, k) => {
      return { ...row, [k]: obj[k] ?? "" };
    }, {});
  });
  return stringify(data, { headers: true, columns });
}

const input = readInput();
const json = JSON.parse(input);
const csv = jsonToCsv(json);
console.log(csv);
