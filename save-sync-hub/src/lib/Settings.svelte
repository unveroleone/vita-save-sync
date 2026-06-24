<script lang="ts">
  let {
    serverUrl = $bindable(""),
    apiToken = $bindable(""),
    deviceName = $bindable(""),
    devices = [],
    loading = false,
    message = "",
    onSaveConfig,
    onRefreshDevices,
  }: {
    serverUrl: string;
    apiToken: string;
    deviceName: string;
    devices: any[];
    loading: boolean;
    message: string;
    onSaveConfig: () => void;
    onRefreshDevices: () => void;
  } = $props();

  function formatTime(ts: string): string {
    try {
      return new Date(ts).toLocaleString();
    } catch {
      return ts;
    }
  }
</script>

<main class="content">
  <div class="header">Settings</div>

  {#if message}
    <div class="message">{message}</div>
  {/if}

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

  <hr />

  <div class="section-header">
    <span>Connected Devices</span>
    <button class="refresh" onclick={onRefreshDevices} disabled={loading}>
      {loading ? "..." : "Refresh"}
    </button>
  </div>

  {#if devices.length === 0}
    <div class="empty">No devices found. Pair a device or save your config first.</div>
  {:else}
    <div class="device-list">
      {#each devices as d}
        <div class="device-row">
          <span class="device-id">{d.device_id ?? d.deviceId}</span>
          <span class="device-time">paired {formatTime(d.paired_at ?? d.pairedAt)}</span>
        </div>
      {/each}
    </div>
  {/if}
</main>

<style>
  .content {
    flex: 1;
    padding: 20px 24px;
    overflow-y: auto;
    max-width: 480px;
  }
  .header {
    font-size: 1rem;
    font-weight: 600;
    margin-bottom: 16px;
  }
  .message {
    background: #2a2a2a;
    color: #4caf50;
    padding: 8px 12px;
    border-radius: 4px;
    margin-bottom: 12px;
    font-size: 0.85rem;
  }
  section {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 12px;
  }
  label {
    font-size: 0.75rem;
    color: #999;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  input {
    background: #1a1a1a;
    border: 1px solid #444;
    color: #eee;
    padding: 6px 8px;
    border-radius: 4px;
    font-size: 0.85rem;
  }
  input:focus {
    outline: none;
    border-color: #00b4d8;
  }
  button.save {
    background: #00b4d8;
    color: #111;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    font-weight: 600;
    font-size: 0.85rem;
    cursor: pointer;
    margin-bottom: 12px;
  }
  hr {
    border: none;
    border-top: 1px solid #333;
    margin: 16px 0;
  }
  .section-header {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
    font-weight: 500;
    margin-bottom: 8px;
  }
  .refresh {
    font-size: 0.7rem;
    padding: 2px 8px;
    border: 1px solid #444;
    background: transparent;
    color: #ccc;
    border-radius: 3px;
    cursor: pointer;
  }
  .refresh:hover {
    background: #333;
  }
  .empty {
    color: #666;
    font-size: 0.8rem;
  }
  .device-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .device-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    background: #222;
    border-radius: 4px;
  }
  .device-id {
    font-size: 0.85rem;
    font-weight: 500;
  }
  .device-time {
    font-size: 0.7rem;
    color: #888;
  }
</style>
