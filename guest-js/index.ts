import { invoke } from "@tauri-apps/api/tauri";

export async function unarchive(
  archivePath: string,
  targetDir?: string
): Promise<string> {
  return await invoke("plugin:unarchiver|unarchive", {
    archivePath,
    targetDir,
  });
}

export async function exists(path: string): Promise<boolean> {
  return await invoke("plugin:unarchiver|exists", { path });
}
