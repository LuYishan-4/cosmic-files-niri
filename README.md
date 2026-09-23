# COSMIC Files for Niri / NyxNiri

A Niri-focused fork of [pop-os/cosmic-files](https://github.com/pop-os/cosmic-files).

This fork keeps the COSMIC Files codebase and file-management features while
adjusting the default UI for a modern Niri/NyxNiri desktop:

- grid view by default;
- larger file and folder icons;
- compact two-line grid labels;
- narrower navigation/sidebar layout;
- softer selection cards using the system accent;
- blur-friendly libcosmic surfaces;
- system theme integration, so dark/light and accent changes continue to follow COSMIC/libcosmic.

See [NYXNIRI.md](NYXNIRI.md) for the recommended Niri rule and integration
settings.

## Build from source

```sh
git clone https://github.com/LuYishan-4/cosmic-files-niri
cd cosmic-files-niri
cargo build --release
cargo run --release
```

The binary and application ID intentionally remain compatible with upstream
COSMIC Files (`cosmic-files` / `com.system76.CosmicFiles`) so it can be used
as a drop-in replacement.

## Upstream

Original project: [pop-os/cosmic-files](https://github.com/pop-os/cosmic-files)

## License

GPL-3.0-only. See [LICENSE](LICENSE).
