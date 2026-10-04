import { VersionManifest } from "@/types/meta";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export const metaKeys = {
  versions: ["minecraft-versions"] as const,
};

export function useMinecraftVersion(forceRefresh = false) {
  return useQuery({
    queryKey: metaKeys.versions,
    queryFn: async () => {
      return await invoke<VersionManifest>("get_minecraft_versions", {
        forceRefresh,
      });
    },
    staleTime: 1000 * 60 * 60 * 6,
  });
}
