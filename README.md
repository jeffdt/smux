# smux

A keyboard-driven tmux session picker. It replaces `prefix + s`: pinned
sessions stay on top in an order you choose, the rest sort by recency, and each
session expands into its windows. Opens on demand via `tmux popup -E`.

![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust&logoColor=white)
![TUI](https://img.shields.io/badge/TUI-ratatui-1f6feb)
![License](https://img.shields.io/badge/license-MIT-green)
![Platform](https://img.shields.io/badge/platform-macOS%20(Apple%20Silicon)-lightgrey)
![Vibe coded](https://img.shields.io/badge/vibe%20coded-100%25-ff69b4)

## Install

```sh
brew install jeffdt/tap/smux
```

Then add a keybind to `~/.tmux.conf`:

```tmux
bind S display-popup -E -w 80% -h 80% "exec smux"
```

Reload tmux and press `prefix + Shift+S`.

## Keys

| Key | Action |
| --- | --- |
| `↵` | Switch to the selected session/window and close |
| `1`-`9` | Switch to that session immediately |
| `M-1`-`M-9` | Focus and expand that session (Option/Alt) |
| `j` / `k` | Move the cursor (also `↓` / `↑`) |
| `l` / `h` | Expand / collapse a session |
| `z` | Expand or collapse all |
| `p` | Pin / unpin the selected session |
| `⇧J` / `⇧K` | Reorder a pinned session down / up |
| `q` / `Esc` | Quit |

`M-` is Meta (Option on macOS). Your terminal must send Option as Meta: in
Ghostty set `macos-option-as-alt = true` (iTerm2: "Left Option key → Esc+";
Terminal.app: "Use Option as Meta key"). On Linux it is automatic.

## Configuration

Pins and sort order persist to `~/.config/smux/config.toml`:

```toml
pinned = ["workbench", "config-tmux"]
sort = "activity"  # or "created"
```

You normally don't edit this by hand; pin and reorder from the picker and it
saves automatically.

## Disclaimer

This project was fully vibe coded. Use at your own risk.

## License

MIT
