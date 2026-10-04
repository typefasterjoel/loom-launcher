export interface LatestVersion {
  release: string;
  snapshot: string;
}

export interface VersionEntry {
  id: string;
  type: "release" | "snapshot" | "old_beta" | "old_alpha";
  url: string;
  time: string;
  releaseTime: string;
}

export interface VersionManifest {
  latest: LatestVersion;
  versions: VersionEntry[];
}
