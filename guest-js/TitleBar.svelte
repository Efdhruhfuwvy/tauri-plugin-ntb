<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import type { Snippet } from "svelte";
  import {
    addMaximizeButton,
    getWindowControlImages,
    titleBarMouseDown,
  } from "./common";
  import {
    WindowControl,
    WindowControlImageType,
    type WindowControlImages,
    type WindowControls
  } from "./models";

  const {
    left = null,
    center = null,
    right = null,
    dialog = null,
    leftControls = true,
    rightControls = true
  }: {
    left?: Snippet | null;
    center?: Snippet | null;
    right?: Snippet | null;
    dialog?: HTMLDialogElement | null,
    leftControls?: boolean;
    rightControls?: boolean;
  } = $props();

  let titleBar: Element;
  let maximized = $state(false);
  invoke("plugin:ntb|is_maximized")
    .then((maximized1) => maximized = maximized1 as boolean);
  getCurrentWindow().onResized(async () =>
    maximized = await invoke("plugin:ntb|is_maximized"));
  let windowControls: WindowControls = $state({
    left: [],
    right: [],
  });
  invoke("plugin:ntb|get_window_controls")
    .then((controls: WindowControls) => windowControls = controls);
  let windowControlImages: WindowControlImages = $state({
    unmaximized: {
      [WindowControl.Minimize]: {
        type: WindowControlImageType.Text,
        font: "",
        size: 0,
        text: "",
      },
      [WindowControl.Maximize]: {
        type: WindowControlImageType.Text,
        font: "",
        size: 0,
        text: "",
      },
      [WindowControl.Close]: {
        type: WindowControlImageType.Text,
        font: "",
        size: 0,
        text: "",
      },
    },
    maximized: {
      [WindowControl.Minimize]: {
        type: WindowControlImageType.Text,
        font: "",
        size: 0,
        text: "",
      },
      [WindowControl.Maximize]: {
        type: WindowControlImageType.Text,
        font: "",
        size: 0,
        text: "",
      },
      [WindowControl.Close]: {
        type: WindowControlImageType.Text,
        font: "",
        size: 0,
        text: "",
      },
    },
  });
  getWindowControlImages()
    .then((images) => windowControlImages = images);

  // svelte-ignore non_reactive_update
  let maximizeButton: Element;
</script>

{#snippet windowControl(control: WindowControl)}
  {#snippet internal(onClick: () => void, dialogOnClick: () => void = () => {})}
    {@const image = windowControlImages[maximized ? "maximized" : "unmaximized"]
      [control]}
    {#if !dialog || control === WindowControl.Close}
      {#if control === WindowControl.Maximize}
        <button
          bind:this={maximizeButton}
          {@attach addMaximizeButton}
          onclick={dialog ? dialogOnClick : onClick}
          tabindex={-1}>
          {#if image?.type === WindowControlImageType.SVG}
            {@html image.svg}
          {:else if image?.type === WindowControlImageType.Text}
            <span style={`font: ${image.size}px ${image.font}`}>
              {image.text}
            </span>
          {/if}
        </button>
      {:else}
        <button
          class:close={control === WindowControl.Close}
          onclick={dialog ? dialogOnClick : onClick}
          tabindex={-1}>
          {#if image?.type === WindowControlImageType.SVG}
            {@html image.svg}
          {:else if image?.type === WindowControlImageType.Text}
            <span style={`font: ${image.size}px ${image.font}`}>{image.text}</span>
          {/if}
        </button>
      {/if}
    {/if}
  {/snippet}
  {#if control === WindowControl.Minimize}
    {@render internal(() => invoke("plugin:ntb|minimize"))}
  {:else if control === WindowControl.Maximize}
    {@render internal(() => invoke("plugin:ntb|toggle_maximize"))}
  {:else if control === WindowControl.Close}
    {@render internal(() => invoke("plugin:ntb|close"), () => dialog.close())}
  {/if}
{/snippet}

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  bind:this={titleBar}
  class="title-bar"
  onmousedown={titleBarMouseDown}
  onclick={(event) => {
    if (event.detail === 2) {
      invoke("plugin:ntb|double_click_title_bar");
    }
  }}
  oncontextmenu={() => {
    invoke("plugin:ntb|right_click_title_bar");
  }}
  onauxclick={(event) => {
    if (event.button === 1) {
      invoke("plugin:ntb|middle_click_title_bar");
    }
  }}>
  <div class="left">
    {#if leftControls}
      <div class="window-controls">
        {#each windowControls.left as windowControl1}
          {@render windowControl(windowControl1)}
        {/each}
      </div>
    {/if}
    {@render left?.()}
  </div>
  <div class="center">{@render center?.()}</div>
  <div class="right">
    {@render right?.()}
      {#if rightControls}
      <div class="window-controls">
        {#each windowControls.right as windowControl1}
          {@render windowControl(windowControl1)}
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .title-bar {
    display: flex;
    align-items: center;

    & :global(*) {
      align-self: stretch;
      display: flex;
      align-items: center;
    }

    & > .left {
      flex: 1;
    }

    & > .right {
      flex: 1;
      justify-content: end;
    }
  }
</style>
