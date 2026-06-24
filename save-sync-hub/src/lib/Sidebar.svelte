<script lang="ts">
  interface Source {
    id: string;
    platform: string;
    customPath: string;
    label: string;
    resolvedPath: string;
  }

  let {
    sources = $bindable([] as Source[]),
    loading = false,
    onBrowseSource,
    onRemoveSource,
    onRenameSource,
    onAddCustom,
    onScanAll,
  }: {
    sources: Source[];
    loading: boolean;
    onBrowseSource: (id: string) => void;
    onRemoveSource: (id: string) => void;
    onRenameSource: (id: string, label: string) => void;
    onAddCustom: () => void;
    onScanAll: () => void;
  } = $props();

  let editingId = $state<string | null>(null);
  let editValue = $state("");

  function startEdit(src: Source) {
    editingId = src.id;
    editValue = src.label;
  }

  function commitEdit() {
    if (editingId && editValue.trim()) {
      onRenameSource(editingId, editValue.trim());
    }
    editingId = null;
    editValue = "";
  }

  function onEditKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") commitEdit();
    if (e.key === "Escape") { editingId = null; editValue = ""; }
  }
</script>

<aside class="sidebar">
  <h1>Save Sync Hub</h1>

  <div class="sources">
    {#each sources as src}
      <div class="source-card">
        <div class="source-header">
          {#if editingId === src.id}
            <input
              class="edit-input"
              type="text"
              bind:value={editValue}
              onkeydown={onEditKeydown}
              onblur={commitEdit}
              autofocus
            />
          {:else}
            <span class="source-label">{src.label}</span>
            <button class="edit" onclick={() => startEdit(src)} title="Rename">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 3a2.83 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/><path d="m15 5 4 4"/></svg>
            </button>
          {/if}
          <button class="remove" onclick={() => onRemoveSource(src.id)} title="Remove">&times;</button>
        </div>
        <div class="source-path">
          <span class="path">{src.resolvedPath || "Not found"}</span>
          <button class="browse" onclick={() => onBrowseSource(src.id)}>...</button>
        </div>
      </div>
    {/each}
  </div>

  <button class="add" onclick={onAddCustom}>+ Add Path</button>

  <button class="scan" onclick={onScanAll} disabled={loading}>
    {loading ? "Scanning..." : "Scan All"}
  </button>

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
    gap: 10px;
    overflow-y: auto;
  }
  h1 {
    font-size: 1.1rem;
    font-weight: 600;
    color: #00b4d8;
    margin-bottom: 4px;
  }
  .sources {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .source-card {
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 4px;
    padding: 8px 10px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .source-header {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .source-label {
    flex: 1;
    font-size: 0.8rem;
    font-weight: 500;
    color: #ccc;
  }
  .edit-input {
    flex: 1;
    background: #111;
    border: 1px solid #00b4d8;
    color: #eee;
    padding: 2px 6px;
    border-radius: 3px;
    font-size: 0.8rem;
    outline: none;
  }
  button.edit {
    background: transparent;
    border: none;
    color: #555;
    cursor: pointer;
    padding: 2px;
    line-height: 1;
  }
  button.edit:hover {
    color: #aaa;
  }
  .remove {
    background: transparent;
    border: none;
    color: #666;
    font-size: 1rem;
    cursor: pointer;
    padding: 0 2px;
    line-height: 1;
  }
  .remove:hover {
    color: #f44336;
  }
  .source-path {
    display: flex;
    gap: 4px;
    align-items: center;
  }
  .path {
    flex: 1;
    font-size: 0.7rem;
    color: #888;
    word-break: break-all;
  }
  button {
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  button.browse {
    background: #333;
    color: #ccc;
    padding: 2px 6px;
    font-size: 0.7rem;
  }
  button.add {
    background: transparent;
    color: #888;
    border: 1px dashed #555;
    padding: 6px;
    font-size: 0.8rem;
  }
  button.add:hover {
    background: #222;
    color: #aaa;
  }
  button.scan {
    background: #00b4d8;
    color: #111;
    padding: 8px;
    font-weight: 600;
    font-size: 0.85rem;
  }
  button:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .version {
    margin-top: auto;
    font-size: 0.7rem;
    color: #555;
  }
</style>
