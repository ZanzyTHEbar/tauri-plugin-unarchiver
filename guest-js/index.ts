import { invoke } from "@tauri-apps/api/tauri";

export async function unarchive(
  archivePath: string,
  targetDir?: string,
  eraseWhenDone?: boolean
): Promise<string> {
  return await invoke("plugin:unarchiver|unarchive", {
    archivePath,
    targetDir,
    eraseWhenDone,
  });
}

export async function exists(path: string): Promise<boolean> {
  return await invoke("plugin:unarchiver|exists", { path });
}
