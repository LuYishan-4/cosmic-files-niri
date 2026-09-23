# COSMIC Files for Niri / NyxNiri

A Niri-focused fork of [pop-os/cosmic-files](https://github.com/pop-os/cosmic-files).

The file-management engine stays COSMIC Files, while the default presentation is
tuned for a NyxNiri desktop: grid-first navigation, larger icons, compact capsule
breadcrumbs, a narrower inset sidebar, softer accent selections, immediate hover
feedback and compositor-friendly blur.

## Arch Linux install

This fork installs as a drop-in replacement for the official `cosmic-files`
package.

```sh
sudo pacman -S --needed base-devel git
git clone https://github.com/LuYishan-4/cosmic-files-niri
cd cosmic-files-niri
makepkg -si
cosmic-files-niri-setup
```

`makepkg -si` will install the required build/runtime dependencies and will ask
to replace the official `cosmic-files` package if it is currently installed.

The setup helper then:

- makes COSMIC Files the default handler for directories;
- switches XDG FileChooser preference to the COSMIC portal;
- changes common Dolphin/Nautilus/Nemo Niri launcher bindings to `cosmic-files`;
- adds the NyxNiri opacity/blur/14px corner rule;
- reloads Niri and restarts the user portal services.

It creates `.bak` copies before changing existing Niri bind/rule files.

## Standalone Niri behavior

The default build intentionally does **not** enable libcosmic's `dbus-config`
feature. A Niri session does not normally run `cosmic-settings-daemon`, and
enabling that backend causes repeated `CosmicTheme.*` / `CosmicTk` watcher
errors. Configuration falls back to libcosmic's non-daemon config path instead.

Normal application blur is also compositor-owned. The Niri rule in
[NYXNIRI.md](NYXNIRI.md) provides blur/opacity/corner clipping without creating
COSMIC layer-surface overlap subscriptions.

## Visual changes

- Grid view is the default for normal file browsing and dialogs.
- Grid icons default to 125%; list icons to 110%.
- File names use a compact two-line grid label.
- Grid spacing is increased slightly for a card-like layout.
- Pointer hover receives immediate surface feedback.
- Selected entries use a surface-colored card with accent treatment rather than
  a fully saturated block.
- Breadcrumbs use compact header-bar capsules.
- Navigation controls use the same capsule-oriented header treatment.
- Sidebar maximum width is 248 px and is inset from the outer window.
- Search and tab chrome are slightly more compact.
- libcosmic blur is requested for the normal application window as well as
  desktop surfaces.
- Niri owns final opacity, blur and outer corner clipping.

## Manual Niri integration

See [NYXNIRI.md](NYXNIRI.md) if you do not want to run the setup helper.

## Build without makepkg

```sh
cargo build --release --locked --workspace
cargo run --release
```

The binary and application ID intentionally remain compatible with upstream
COSMIC Files (`cosmic-files` / `com.system76.CosmicFiles`) so XDG associations,
desktop files and COSMIC portal integration continue to work.

## Upstream

Original project: [pop-os/cosmic-files](https://github.com/pop-os/cosmic-files)

## License

GPL-3.0-only. See [LICENSE](LICENSE).
