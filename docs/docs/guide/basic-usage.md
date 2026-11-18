# Basic Usage

Now that you're all set up and running, here's a quick walkthrough of how you can use Atuin best.

## What does Atuin record?

While you work, Atuin records:

1. The command you run
2. The directory you ran it in
3. The time you ran it, and how long it took to run
4. The exit code of the command
5. The hostname + user of the machine
6. The shell session you ran it in

### Hostnames on macOS

On macOS, Atuin records the hostname that stays the same across networks. The system hostname
can change when you join a different network, because macOS can get it from DHCP or DNS.
Atuin uses the first available name in this order:

1. The `ATUIN_HOST_NAME` environment variable
2. The `HostName` that `scutil --set HostName` sets
3. The local hostname that System Settings shows under General > Sharing, without the `.local`
   suffix. For example, Atuin records `My-MacBook-Pro.local` as `My-MacBook-Pro`.
4. The system hostname

Earlier versions of Atuin recorded the system hostname. Existing entries keep that name. The
[host filter](advanced-usage.md#filter-mode) can find them by install ID or by an older name linked
to that ID. On upgrade, Atuin fills in IDs where the records provide enough information. Some
older names may stay absent from the host filter if no surviving entry links them to the current
install. Global search still finds those entries.

## Opening and using the TUI

At any time, you can open the TUI with the default keybindings of the up arrow, or `ctrl-r`.

Once in the TUI, press enter to immediately execute a command, or press tab to insert it into your shell for editing.

While searching in the TUI, you can narrow the search scope by pressing `ctrl-r` to cycle through [filter modes](advanced-usage.md#filter-mode) — the full history, this machine, the current directory, the current repository, or the current shell session.

See the [advanced usage](advanced-usage.md) page for more options.

## Common config changes

For a full set of config values, please see the [config reference page](../configuration/config.md).

The default configuration file is located at `~/.atuin/config.toml`.

### Keybindings

We've got a [full page dedicated to keybinding adjustments](../configuration/key-binding.md).
It covers a whole bunch of options, including disabling the up arrow behavior if you don't like it.

### Enter to run

You may prefer that Atuin always inserts the selected command for editing. To configure this, set

```toml
enter_accept = false
```

in your config file.

### Inline window

If you find the full screen TUI overwhelming or too large, you can adjust it like so:

```toml
# height of the search window
inline_height = 40
```

You may also prefer the compact UI mode:

```toml
style = "compact"
```

### tmux popup

If you use tmux, Atuin can open the search UI in a popup floating above your
current pane rather than drawing over it:

```toml
[tmux]
enabled = true
```

See the [`tmux` config reference](../configuration/config.md#tmux) for sizing and
requirements. If you use iTerm2's native tmux integration, keep the popup off —
it can't display tmux popups.
