# NyxNiri integration

COSMIC Files for Niri keeps the upstream application ID:

```text
com.system76.CosmicFiles
```

That means existing XDG associations and portal integrations keep working.

## Niri window rule

Add this to `~/.config/niri/rules.kdl`:

```kdl
window-rule {
    match app-id=r"^com\.system76\.CosmicFiles$"

    opacity 0.90
    geometry-corner-radius 14
    clip-to-geometry true
    draw-border-with-background false

    background-effect {
        blur true
        xray false
    }
}
```

Reload Niri:

```sh
niri msg action load-config-file
```

## Make it the default file manager

```sh
xdg-mime default com.system76.CosmicFiles.desktop inode/directory
xdg-mime default com.system76.CosmicFiles.desktop application/x-directory
xdg-mime default com.system76.CosmicFiles.desktop x-scheme-handler/file
```

## Design direction

The fork intentionally keeps libcosmic rather than replacing the UI with GTK or
Qt. NyxNiri styling is implemented by combining:

1. system/libcosmic theme and accent colors;
2. less saturated selection surfaces with accent outlines;
3. a more visual grid-first layout;
4. compact navigation chrome;
5. compositor-owned opacity, blur and outer corner clipping.

This keeps upstream functionality and accessibility while making the app fit a
Niri/NyxNiri session.
