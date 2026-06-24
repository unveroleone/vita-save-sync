<script lang="ts">
  let {
    platform = $bindable("psp"),
    customPath = $bindable(""),
    serverUrl = $bindable(""),
    apiToken = $bindable(""),
    deviceName = $bindable(""),
    resolvedPath = "",
    loading = false,
    onPlatformChange,
    onBrowse,
    onRefreshPath,
    onSaveConfig,
    onScan,
  }: {
    platform: string;
    customPath: string;
    serverUrl: string;
    apiToken: string;
    deviceName: string;
    resolvedPath: string;
    loading: boolean;
    onPlatformChange: (p: string) => void;
    onBrowse: () => void;
    onRefreshPath: () => void;
    onSaveConfig: () => void;
    onScan: () => void;
  } = $props();

  function handlePlatform(e: Event) {
    const val = (e.target as HTMLSelectElement).value;
    onPlatformChange(val);
  }
</script>

<aside class="sidebar">
  <h1>Save Sync Hub</h1>

  <section>
    <label for="platform-select">Platform</label>
    <select id="platform-select" value={platform} onchange={handlePlatform}>
      <option value="psp">PSP (PPSSPP)</option>
      <option value="retroarch">RetroArch</option>
      <option value="custom">Custom path</option>
    </select>

    {#if platform === "custom"}
      <div class="path-row">
        <input
          type="text"
          bind:value={customPath}
          placeholder="/path/to/saves"
          onchange={() => onRefreshPath()}
        />
        <button class="browse" onclick={onBrowse}>...</button>
      </div>
    {:else}
      <div class="path-info">
        <span class="path-label">Resolved path:</span>
        <span class="path-value">{resolvedPath}</span>
        <button class="browse" onclick={onBrowse}>Browse...</button>
      </div>
    {/if}
  </section>

  <button class="scan" onclick={onScan} disabled={loading}>
    {loading ? "Scanning..." : "Scan Saves"}
  </button>

  <hr />

  <section>
    <label for="server-url">Server URL</label>
    <input id="server-url" type="text" bind:value={serverUrl} placeholder="https://myserver.com" />
  </section>

  <section>
    <label for="api-token">API Token</label>
    <input id="api-token" type="password" bind:value={apiToken} placeholder="your-token" />
  </section>

  <section>
    <label for="device-name">Device Name</label>
    <input id="device-name" type="text" bind:value={deviceName} placeholder="my-pc" />
  </section>

  <button class="save" onclick={onSaveConfig}>Save Config</button>

  <div class="version">v0.1.0 · by unveroleone</div>
</aside>

<style>
  .sidebar {
    width: 280px;
    min-width: 280px;
    background: #222;
    padding: 20px 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    overflow-y: auto;
  }
  h1 {
    font-size: 1.1rem;
    font-weight: 600;
    color: #00b4d8;
    margin-bottom: 4px;
  }
  section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  label {
    font-size: 0.75rem;
    color: #999;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  select, input {
    background: #1a1a1a;
    border: 1px solid #444;
    color: #eee;
    padding: 6px 8px;
    border-radius: 4px;
    font-size: 0.85rem;
  }
  select:focus, input:focus {
    outline: none;
    border-color: #00b4d8;
  }
  .path-row {
    display: flex;
    gap: 4px;
  }
  .path-row input {
    flex: 1;
  }
  .path-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 4px;
    padding: 6px 8px;
  }
  .path-label {
    font-size: 0.65rem;
    color: #666;
    text-transform: uppercase;
  }
  .path-value {
    font-size: 0.75rem;
    color: #aaa;
    word-break: break-all;
    margin-bottom: 4px;
  }
  button {
    background: #00b4d8;
    color: #111;
    border: none;
    padding: 8px;
    border-radius: 4px;
    font-weight: 600;
    font-size: 0.85rem;
    cursor: pointer;
  }
  button:disabled {
    opacity: 0.5;
    cursor: default;
  }
  button.browse {
    background: #333;
    color: #ccc;
    padding: 4px 8px;
    font-size: 0.75rem;
    font-weight: 400;
  }
  button.save {
    background: #444;
    color: #eee;
  }
  hr {
    border: none;
    border-top: 1px solid #333;
  }
  .version {
    margin-top: auto;
    font-size: 0.7rem;
    color: #555;
  }
</style>
