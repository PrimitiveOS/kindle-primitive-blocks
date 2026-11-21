<div align="center">

# Blocks

**Fit pieces and clear the board**

[![Release](https://img.shields.io/github/v/release/PrimitiveOS/kindle-primitive-blocks?label=release)](https://github.com/PrimitiveOS/kindle-primitive-blocks/releases) [![Licence: PolyForm-Noncommercial-1.0.0](https://img.shields.io/badge/licence-PolyForm--Noncommercial--1.0.0-blue)](LICENSE) [![Store downloads](https://img.shields.io/endpoint?url=https%3A%2F%2Fprimitiveos.github.io%2Fkindle-primitive-catalog%2Fv1%2Fbadges%2Fprimitive-blocks.json)](https://primitiveos.github.io/kindle-primitive-catalog/v1/index.json)

[**Download**](https://github.com/PrimitiveOS/kindle-primitive-blocks/releases/latest) · [**Install guide**](INSTALL.md) · [**Changelog**](CHANGELOG.md)

</div>

---

Place three pieces at a time on a nine-by-nine board. Fill a row, column, or three-by-three box to clear it and beat your best score.

## Quick start

After setting up PrimitiveOS, open **Store** from Primitive Launcher. Choose **Blocks** under **Apps**,
tap **Get**, and open it from the Launcher grid. The [install guide](INSTALL.md) also covers the standalone USB package.

## Screenshots

<table>
<tr>
<td align="center"><img src="docs/images/blocks.png" width="260" alt="Nine-by-nine board and three offered pieces"><br><sub>Choose an opening piece</sub></td>
<td align="center"><img src="docs/images/blocks-playing-dark.png" width="260" alt="Board with placed pieces in dark mode"><br><sub>Continue a game in dark mode</sub></td>
<td align="center"><img src="docs/images/blocks-reset.png" width="260" alt="New game confirmation over an existing board"><br><sub>Confirm before starting a new game</sub></td>
</tr>
</table>

<sub>Mac simulation captures at 1264 × 1680 pixels.</sub>

## Highlights

- **Place pieces.** Choose an offered piece and tap a position on the board.
- **Clear several lines at once.** A single placement can complete a row, column, and box.
- **Resume a game.** Blocks saves the current board and your best score.

## Develop locally

Keep `doannt-kindle-tools` and `doannt-kindle-launcher` beside this checkout, then run the following from this repo.

```sh
cargo test --offline --locked --workspace
cargo run --offline --locked -p blocks-desktop
```

The desktop target writes previews to `build/previews/`. The desktop simulation exercises placement and the new-game confirmation.
On an ARMHF Kindle Oasis 3 running firmware 5.18.2, Blocks selected and deselected a piece, preserving the
original board and tray, then returned to Launcher. See the [device notes](docs/device-notes.md) for the
verification checklist.

## Status

| Device | Status |
|---|---|
| Kindle Oasis 3 (1264 × 1680, firmware 5.18.2) | Tested on hardware |

**Current version: 0.1.1.** Changes are in [CHANGELOG.md](CHANGELOG.md).

## Downloads

The badge at the top shows the number of Store downloads. It counts GitHub downloads of this app's release packages, across all versions and architectures, and is refreshed daily from the [Store catalog](https://primitiveos.github.io/kindle-primitive-catalog/v1/index.json).

## Documentation

[Install guide](INSTALL.md) · [Changelog](CHANGELOG.md) · [Device notes](docs/device-notes.md)

## Licence

Copyright (C) 2026 Doan Nguyen.

Code is licensed under [PolyForm Noncommercial 1.0.0](LICENSE). The original documentation and pictures are licensed under [CC BY-NC-SA 4.0](LICENSE-DOCS).
