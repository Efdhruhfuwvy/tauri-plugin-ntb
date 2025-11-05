<script>
  import Greet from './lib/Greet.svelte'
  import TitleBar from 'tauri-plugin-ntb-api/svelte'

  let dialog;
</script>
<TitleBar>
  {#snippet left()}
    Left
  {/snippet}
  {#snippet center()}
    Center
  {/snippet}
  {#snippet right()}
    Right
  {/snippet}
</TitleBar>
<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <Greet />
  </div>

  <div class="row">
    <button onclick={() => dialog.showModal()}>Show Dialog</button>
  </div>

  <dialog bind:this={dialog}>
    <TitleBar dialog={dialog}>
      {#snippet left()}
        Left
      {/snippet}
      {#snippet center()}
        Center
      {/snippet}
      {#snippet right()}
        Right
      {/snippet}
  </TitleBar>
  </dialog>

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
