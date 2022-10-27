import process from 'process';
import hljs from 'highlight.js';
import nodeHtmlToImage from 'node-html-to-image';

/**
 * 標準入力から文字列を読み込む
 * @return Promise<string>
 */
const readInput = async () => {
  const bufferList = [];
  for await (const chunk of process.stdin) {
    bufferList.push(chunk);
  }
  const buffer = Buffer.concat(bufferList);
  return buffer.toString();
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
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.6.0/styles/dark.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.6.0/highlight.min.js"></script>
    <script>hljs.highlightAll();</script>
  </head>
  <body>
    <pre>
      <code>${highlight}</code>
    </pre>
  </body>
</html>`;
};

const code = await readInput();

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
