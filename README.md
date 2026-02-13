# Wallpicker

A small Rust project for learning the basics of the language.

## Description

Wallpicker is a simple wallpaper manager that allows you to browse and select wallpapers from your `~/pictures/wallpapers` directory. It integrates with [Walker](https://github.com/abenz1267/walker) (a dmenu-like application launcher) to provide an interactive selection menu, and uses [swww](https://github.com/LGFae/swww) to set the selected wallpaper.

## Features

- Scans `~/pictures/wallpapers` for image files (jpg, jpeg, png, bmp, gif)
- Displays available wallpapers in Walker's dmenu interface
- Sets the selected wallpaper using swww
- Sends desktop notifications on success/failure

## Usage

Run the binary directly:

```bash
wallpicker
```

Or integrate it into your system menu/keybindings via the `.desktop` file at `~/.local/share/applications/wallpicker.desktop`.

## Dependencies

- [dirs](https://crates.io/crates/dirs) - Platform-specific directory paths
- `walker` - Interactive dmenu launcher (system dependency)
- `swww` - Wallpaper setter (system dependency)
- `notify-send` - Desktop notifications (system dependency)
