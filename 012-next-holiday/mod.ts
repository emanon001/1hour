import { serve } from "https://deno.land/std@0.162.0/http/server.ts";
import holidayJp from "https://cdn.skypack.dev/@holiday-jp/holiday_jp?dts";

const svgTemplate = Deno.readTextFileSync("resources/holiday-template.svg");

type Holiday = {
  daysLeft: number;
};

function getNextHoliday(_now: Date): Holiday {
  return {
    daysLeft: 2,
  };
}

function createHolidaySVG(holiday: Holiday): string {
  return svgTemplate.replace("{daysLeft}", holiday.daysLeft.toString());
}

serve((_req) => {
  const nextHoliday = getNextHoliday(new Date());
  const svg = createHolidaySVG(nextHoliday);
  return new Response(svg, {
    headers: { "content-type": "image/svg+xml" },
  });
});
