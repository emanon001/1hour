import {
  Column,
  DataItem,
  stringify,
} from "https://deno.land/std@0.161.0/encoding/csv.ts";
import { isPlainObject } from "https://deno.land/x/is_what@v4.1.7/src/index.ts";
import { readAllSync } from "https://deno.land/std@0.161.0/streams/conversion.ts";

function readInput(): string {
  const content = readAllSync(Deno.stdin);
  const decoder = new TextDecoder();
  return decoder.decode(content);
}

type Csv = {
  headers: Column[];
  data: DataItem[];
};

function jsonToCsv(json: any): Csv {
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
  const headers = Array.from(keySet);

  // convert
  const data = json.map((obj) => {
    return headers.reduce((row, k) => {
      return { ...row, [k]: obj[k] ?? "" };
    }, {});
  });
  return {
    headers,
    data,
  };
}

const input = readInput();
const json = JSON.parse(input);
const csv = jsonToCsv(json);
const csvString = stringify(csv.data, { columns: csv.headers });
console.log(csvString);
