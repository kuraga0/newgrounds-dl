# Newgrounds downloader

Has two modes, 
parse mode (default) downloads webpage, finds track title and file link and downloads it, 
second mode tries to convert initial link without dowloading a page, and can fail when the title is weird.

```
Usage: newgrounds-dl [OPTIONS] <URL>

Arguments:
  <URL>

Options:
  -p, --parse-page <PARSE_PAGE>  [default: true] [possible values: true, false]
  -t, --title <TITLE>
  -o, --output-dir <OUTPUT_DIR>  [default: .]
  -b, --open-in-browser
  -v, --verbose...
  -h, --help                     Print help
  -V, --version                  Print version
```
