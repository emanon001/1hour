import process from 'process';
import hljs from 'highlight.js';
import nodeHtmlToImage from 'node-html-to-image';
import { readFileSync } from 'fs';

/**
 * 標準入力から文字列を読み込む
 * @return {string}
 */
const readInput = () => {
  return readFileSync(process.stdin.fd, 'utf8');
};

/**
 * HTMLを作成する
 * @param {string} highlight
 * @return {string}
 */
const buildHtml = (highlight) => {
  return `
<!DOCTYPE html>
<html lang="ja">
  <head>
    <meta charset="UTF-8">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/destyle.css@3.0.2/destyle.min.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.6.0/styles/dark.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.6.0/highlight.min.js"></script>
    <script>hljs.highlightAll();</script>
  </head>
  <body>
    <pre><code>${highlight}</code></pre>
  </body>
</html>`;
};

const code = readInput();

// build HTML
const highlight = hljs.highlightAuto(code).value;
const html = buildHtml(highlight);

// create image
nodeHtmlToImage({
  output: './image.png',
  html,
}).then(() => {
  console.log('The image was created successfully');
});
