import { serve } from "https://deno.land/std@0.162.0/http/server.ts";
import holidayJp from "https://esm.sh/@holiday-jp/holiday_jp@2.4.0";
import { DateTime, datetime } from "https://deno.land/x/ptera@v1.0.2/mod.ts";

type Holiday = {
  daysLeft: number;
};

function getNextHoliday(now: DateTime): Holiday {
  const nowJp = now.toZonedTime("Asia/Tokyo");
  let daysLeft = 0;
  for (let dayOffset = 1; dayOffset <= 8; dayOffset++) {
    const dt = nowJp.add({ day: dayOffset });

    // 土日の判定
    const dayOfWeek = Number(dt.format("w"));
    if (dayOfWeek === 6 || dayOfWeek === 0) {
      daysLeft = dayOffset;
      break;
    }

    // 祝日の判定
    const jsDate = new Date(dt.year, dt.month - 1, dt.day);
    if (holidayJp.isHoliday(jsDate)) {
      daysLeft = dayOffset;
      break;
    }
  }
  return {
    daysLeft,
  };
}

const SVG_TEMPLATE = await Deno.readTextFile("resources/holiday-template.svg");

function createHolidaySVG(holiday: Holiday): string {
  // 本当はパースしてid指定でテキストを変更した方がよい
  return SVG_TEMPLATE.replace("{day}", holiday.daysLeft.toString());
}

const ROOT_ROUTE = new URLPattern({ pathname: "/" });

serve((req) => {
  const match = ROOT_ROUTE.exec(req.url);
  if (match) {
    const nextHoliday = getNextHoliday(datetime());
    const svg = createHolidaySVG(nextHoliday);
    return new Response(svg, {
      headers: {
        "content-type": "image/svg+xml",
        "cache-control": "max-age=1800",
      },
    });
  }

  return new Response("Not found", {
    status: 404,
  });
});
