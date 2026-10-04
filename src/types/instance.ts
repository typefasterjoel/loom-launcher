export type ModLoader = "vanilla" | "fabric" | "quilt" | "neoforge" | "forge";

export interface Instance {
  id: string;
  name: string;
  mcVersion: string;
  modLoader: ModLoader;
  loaderVersion: string;
  iconPath?: string | null;
}

export interface CreateInstancePayload {
  name: string;
  mcVersion: string;
  modLoader: ModLoader;
  loaderVersion: string;
}
