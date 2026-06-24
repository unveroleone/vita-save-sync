<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import Sidebar from "./lib/Sidebar.svelte";
  import SaveList from "./lib/SaveList.svelte";

  let platform = $state("psp");
  let customPath = $state("");
  let resolvedPath = $state("");
  let saves = $state<any[]>([]);
  let loading = $state(false);
  let message = $state("");

  let serverUrl = $state("");
  let apiToken = $state("");
  let deviceName = $state("");

  async function loadConfig() {
    try {
      const c = await invoke("load_config_cmd");
      serverUrl = c.server_url;
      apiToken = c.api_token;
      deviceName = c.device_name;
    } catch (e) {
      console.error("Load config failed:", e);
    }
  }

  async function refreshPath() {
    try {
      resolvedPath = await invoke("resolve_platform_path", {
        platform,
        customPath: platform === "custom" ? customPath : null,
      });
    } catch {
      resolvedPath = "Not found";
    }
  }

  async function browseFolder() {
    const selected = await open({ directory: true, multiple: false, title: "Select save folder" });
    if (selected && typeof selected === "string") {
      customPath = selected;
      resolvedPath = selected;
    }
  }

  async function handlePlatformChange(p: string) {
    platform = p;
    await refreshPath();
  }

  async function handleSaveConfig() {
    try {
      await invoke("save_config_cmd", {
        config: { server_url: serverUrl, api_token: apiToken, device_name: deviceName },
      });
      message = "Config saved.";
      setTimeout(() => (message = ""), 2000);
    } catch (e: any) {
      message = "Error: " + e;
    }
  }

  async function scan() {
    loading = true;
    message = "";
    try {
      saves = await invoke("scan_saves", {
        platform,
        customPath: platform === "custom" ? customPath : null,
      });
      if (saves.length === 0) message = "No saves found.";
    } catch (e: any) {
      message = "Error: " + e;
    } finally {
      loading = false;
    }
  }

  async function handleBackup(titleId: string, sourcePath: string) {
    message = "Uploading...";
    try {
      const res = await invoke("backup_and_upload", { titleId, sourcePath });
      message = res as string;
      await scan();
    } catch (e: any) {
      message = "Error: " + e;
    }
  }

  async function handleDownload(titleId: string) {
    message = "Downloading...";
    try {
      const res = await invoke("download_only", { titleId });
      message = "Downloaded to: " + res;
    } catch (e: any) {
      message = "Error: " + e;
    }
  }

  async function handleRestore(titleId: string, targetPath: string) {
    message = "Downloading & restoring...";
    try {
      const res = await invoke("download_and_restore", { titleId, targetPath });
      message = res as string;
    } catch (e: any) {
      message = "Error: " + e;
    }
  }

  loadConfig();
  refreshPath();
</script>

<div class="app">
  <Sidebar
    bind:platform={platform}
    bind:customPath={customPath}
    bind:serverUrl={serverUrl}
    bind:apiToken={apiToken}
    bind:deviceName={deviceName}
    {resolvedPath}
    {loading}
    onPlatformChange={handlePlatformChange}
    onBrowse={browseFolder}
    onRefreshPath={refreshPath}
    onSaveConfig={handleSaveConfig}
    onScan={scan}
  />
  <SaveList
    {saves}
    {platform}
    {message}
    {resolvedPath}
    onBackup={handleBackup}
    onDownload={handleDownload}
    onRestore={handleRestore}
  />
</div>

<style>
  .app { display: flex; height: 100vh; }
</style>
