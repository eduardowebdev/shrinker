# shrinker

> Lossless image compression for everyone — including people who don't code.

`shrinker` takes any supported image, re-encodes it with a **lossless** codec and
keeps whichever of the two outputs is smaller. No quality is lost.

## Install

```bash
cargo install --path .
```

## Usage

```bash
# single file → shrinks it into image.shrink.<ext>
shrinker compress photo.png

# overwrite the original, in place
shrinker compress photo.png -i

# pick the output file explicitly
shrinker compress photo.png -o photo-optimized.png

# batch a whole folder, recursively, in parallel
shrinker compress ./photos -r

# force a specific output format (only lossless formats are accepted)
shrinker compress photo.png -f webp

# inspect metadata without changing anything
shrinker info photo.png
```

## How it works

| Source format                       | Encoder            | Lossless? |
| ----------------------------------- | ------------------ | --------- |
| PNG                                 | oxipng             | ✅ yes    |
| JPEG / WebP / GIF / BMP / TIFF WebP | lossless (libwebp) | ✅ yes    |

If the re-encoded file ends up larger than the original, `shrinker` keeps the original bytes – you never get a worse file out of it.

## License

MIT OR Apache-2.0
