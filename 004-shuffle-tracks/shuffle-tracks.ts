import { run } from "https://deno.land/x/jxa_run@v0.0.3/mod.ts";
import type {} from "https://deno.land/x/jxa_run@v0.0.3/global.d.ts";

await run(
  () => {
    const music = Application("Music");
    music.stop();
    music.shuffleEnabled = true;
    const playlist = music.playlists[1]; // all music
    music.play(playlist);
  },
);
