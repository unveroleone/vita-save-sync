<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";

  let {
    entries = [],
    loading = false,
    message = "",
    onRefresh,
    onDownload,
    onRestore,
    onDelete,
  }: {
    entries: any[];
    loading: boolean;
    message: string;
    onRefresh: () => void;
    onDownload: (titleId: string) => void;
    onRestore: (titleId: string) => void;
    onDelete: (titleId: string) => void;
  } = $props();

  function iconUrl(path: string | null | undefined): string | null {
    if (!path) return null;
    try { return convertFileSrc(path); } catch { return null; }
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return bytes + " B";
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + " KB";
    return (bytes / (1024 * 1024)).toFixed(1) + " MB";
  }

  function formatTime(ts: string): string {
    try {
      // Handle old underscore format (2026-06-24_23-39-00) and ISO (2026-06-24T23:39:00Z).
      const normalized = ts.replace(/_/g, "T") + (ts.includes("Z") ? "" : "Z");
      const d = new Date(normalized);
      if (isNaN(d.getTime())) return ts;
      return d.toLocaleString();
    } catch {
      return ts;
    }
  }
</script>

<main class="content">
  <div class="header">
    Cloud Saves
    <button class="refresh" onclick={onRefresh} disabled={loading}>
      {loading ? "Loading..." : "Refresh"}
    </button>
    {#if entries.length > 0}
      <span class="count">{entries.length} entries</span>
    {/if}
  </div>

  {#if message}
    <div class="message" class:error={message.startsWith("Error") || message.startsWith("Delete failed")}>{message}</div>
  {/if}

  {#if entries.length === 0 && !loading}
    <div class="empty">No saves on the server. Upload from the Local tab first.</div>
  {:else}
    <div class="list">
      {#each entries as entry}
        <div class="row">
          {#if entry.icon_path}
            <div class="icon-wrap">
              <img src={iconUrl(entry.icon_path)} alt="" class="icon" />
            </div>
          {/if}
          <div class="info">
            <span class="name">{entry.display_name}</span>
            <span class="meta">
              {formatSize(entry.size)} · {formatTime(entry.timestamp)}
              {#if entry.version_count > 0}
                · {entry.version_count} version{entry.version_count !== 1 ? "s" : ""}
              {/if}
              · from {entry.uploaded_by}
            </span>
          </div>
          <div class="actions">
            <button
              class="download"
              onclick={() => onDownload(entry.title_id)}
            >Download</button>
            <button
              class="restore"
              onclick={() => onRestore(entry.title_id)}
            >Restore</button>
            <button
              class="delete-btn"
              onclick={() => onDelete(entry.title_id)}
            >Delete</button>
          </div>
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
  }
  .header {
    font-size: 1rem;
    font-weight: 600;
    margin-bottom: 12px;
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .count {
    font-size: 0.75rem;
    color: #666;
    font-weight: 400;
  }
  .refresh {
    font-size: 0.75rem;
    padding: 3px 10px;
    border: 1px solid #444;
    background: transparent;
    color: #ccc;
    border-radius: 3px;
    cursor: pointer;
  }
  .refresh:hover {
    background: #333;
  }
  .refresh:disabled {
    opacity: 0.5;
  }
  .message {
    background: #2a2a2a;
    color: #00b4d8;
    padding: 8px 12px;
    border-radius: 4px;
    margin-bottom: 12px;
    font-size: 0.85rem;
  }
  .message.error {
    color: #f44336;
    background: #2a1a1a;
  }
  .empty {
    color: #666;
    margin-top: 40px;
    text-align: center;
  }
  .list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    background: #222;
    border-radius: 4px;
  }
  .row:hover {
    background: #2a2a2a;
  }
  .icon-wrap {
    width: 48px;
    min-width: 48px;
    height: 32px;
    border-radius: 2px;
    overflow: hidden;
    background: #111;
  }
  .icon {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .name {
    font-weight: 500;
    font-size: 0.9rem;
  }
  .meta {
    font-size: 0.75rem;
    color: #888;
  }
  .actions {
    display: flex;
    gap: 6px;
  }
  .actions button {
    padding: 4px 10px;
    border: 1px solid #444;
    background: transparent;
    color: #ccc;
    border-radius: 3px;
    font-size: 0.75rem;
    cursor: pointer;
    white-space: nowrap;
  }
  .actions button:hover {
    background: #333;
  }
  .download { border-color: #ff7700 !important; color: #ff7700 !important; }
  .restore { border-color: #4caf50 !important; color: #4caf50 !important; }
  .delete-btn { border-color: #f44336 !important; color: #f44336 !important; }
</style>
