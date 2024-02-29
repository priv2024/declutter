# declutter

Use filters and matches conditions against a list of URLs

```
Use filters and matchers conditions against a list of URLs 🧨

Usage: declutter [OPTIONS]

Options:
  -h, --help     Print help
  -V, --version  Print version

Matchers ✅:
  -a, --allow <ALLOW_EXTENSIONS>  Extensions that should be matched. Can be omitted to only deny extensions

Filters 🚫:
  -d, --deny <DENY_EXTENSIONS>  Extensions to always block. Can be omitted to allow all extensions [default: jpg jpeg png svg ico gif webp bmp css scss tif tiff ttf otf woff woff2 eot pdf mp3 mp4 avi]
      --dup                     allow duplicate urls

Debug 🐛:
  -q, --quiet  Do not print anything to stderr
```
