# any2webp
Daemon for converting any newly created image in particular directory to webp on the fly.

Supported source images: png, pnm, tiff, jpeg, bmp.

Usage:
`any2webp /pat/to/images/dir [-rll] -[rls]`
where **-rll** is an optional flag to remove lossless images after conversion to webp
and **-rls** means the same for lossy images.

## Where/when it would be needed?
In cases when you working with dumb old image-generating software which doesn't support webp. Like gnome-screenshots, for example.


### Example systemd user-level service file

```systemd
[Unit]
Description=Any image to webp convertor

[Service]
ExecStart=%h/bin/any2webp /tmp/Screenshots -rll
WorkingDirectory=/tmp/Screenshots

[Install]
WantedBy=default.target
```
