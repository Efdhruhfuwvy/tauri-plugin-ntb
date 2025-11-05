# Tauri Plugin NTB

**NTB** (**N**ative **T**itle **B**ar) is a [Tauri v2](https://v2.tauri.app)
plugin providing native-looking title bar.

## Features

- Changes with Linux theme and window control layout
- Supports Windows Snap Overlay
- Customizable window controls side

It doesn't support macOS, because Tauri supports the window controls overlay on
macOS.

## Usage

### Permissions

NTB has all the needed permissions in the default permission.

### Usage

NTB currently only supports Svelte.

#### Svelte

```svelte
<script>
  import TitleBar from "tauri-plugin-ntb-api/svelte";
</script>

<TitleBar
  leftControls={true /* show the left title bar controls (default: true) */}
  rightControls={true /* show the right title bar controls (default: true) */}>
  {#snippet left()}
    <!-- the left title bar content -->
  {/snippet}
  {#snippet center()}
    <!-- the center title bar content -->
  {/snippet}
  {#snippet right()}
    <!-- the right title bar content -->
  {/snippet}
</TitleBar>
```
