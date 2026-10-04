import { Button } from "@/components/ui/button";
import { ButtonGroup } from "@/components/ui/button-group";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Combobox,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxInput,
  ComboboxItem,
  ComboboxList,
} from "@/components/ui/combobox";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Field, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import {
  Item,
  ItemContent,
  ItemDescription,
  ItemTitle,
} from "@/components/ui/item";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useCreateInstance } from "@/hooks/use-instances";
import { useMinecraftVersion } from "@/hooks/use-minecraft-version";
import { ModLoader } from "@/types/instance";
import { VersionEntry } from "@/types/meta";
import { invoke } from "@tauri-apps/api/core";
import { PackagePlusIcon } from "lucide-react";
import { useEffect, useState, type SubmitEvent } from "react";

const MOD_LOADERS: { label: string; value: ModLoader }[] = [
  { label: "Vanilla", value: "vanilla" },
  { label: "Fabric", value: "fabric" },
  { label: "Quilt", value: "quilt" },
  { label: "NeoForge", value: "neoforge" },
  { label: "Forge", value: "forge" },
];

export default function CreateInstanceDialog() {
  const [name, setName] = useState("");
  const [mcVersion, setMcVersion] = useState<string | null>("");
  const [modLoader, setModLoader] = useState<ModLoader>("vanilla");
  const [showSnapshots, setShowSnapshots] = useState(false);

  const { data: manifest, isLoading: isLoadingVersions } =
    useMinecraftVersion();
  const createInstanceMutation = useCreateInstance();

  const availableVersions =
    manifest?.versions.filter(
      (version) => showSnapshots || version.type === "release",
    ) ?? [];

  useEffect(() => {
    if (manifest?.latest.release && !mcVersion) {
      setMcVersion(manifest.latest.release);
    }
  }, [manifest, mcVersion]);

  const handleSubmit = async (e: SubmitEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!name.trim() || !mcVersion) return;

    try {
      const resolvedLoaderVersion = await invoke<string>(
        "get_latest_loader_version",
        { loader: modLoader, gameVersion: mcVersion },
      );

      await createInstanceMutation.mutateAsync({
        name: name.trim(),
        mcVersion: mcVersion,
        modLoader: modLoader,
        loaderVersion: resolvedLoaderVersion,
      });

      setName("");
    } catch (err) {
      console.error("Error creating instance:", err);
    }
  };

  return (
    <Dialog data-slot="app-create-instance-dialog">
      <Tooltip>
        <TooltipTrigger
          render={
            <DialogTrigger
              render={
                <Button size="icon-lg">
                  <PackagePlusIcon />
                </Button>
              }
            />
          }
        />
        <TooltipContent side="inline-end">Create Instance</TooltipContent>
      </Tooltip>

      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>New Instance</DialogTitle>
          <DialogDescription>Create a new Minecraft instance</DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="flex flex-col gap-3">
          <Field>
            <FieldLabel>Name</FieldLabel>
            <Input
              id="instance-name"
              required
              placeholder="Instance Name"
              onChange={(e) => setName(e.target.value)}
            />
          </Field>
          <Field>
            <FieldLabel>Loader</FieldLabel>
            <ButtonGroup className="w-full">
              {MOD_LOADERS.map((loader) => (
                <Button
                  key={loader.value}
                  onClick={() => setModLoader(loader.value)}
                  variant={loader.value === modLoader ? "default" : "outline"}
                  className="grow"
                >
                  {loader.label}
                </Button>
              ))}
            </ButtonGroup>
          </Field>
          <Field>
            <FieldLabel>Minecraft Version</FieldLabel>
            <div className="flex items-center gap-2">
              <Combobox
                items={availableVersions}
                id="instance-mc-version"
                value={mcVersion}
                onValueChange={setMcVersion}
                disabled={isLoadingVersions}
              >
                <ComboboxInput placeholder="Select a Version of Minecraft" />
                <ComboboxContent className="sm:max-w-3xs">
                  <ComboboxEmpty>No Minecraft Version Found</ComboboxEmpty>
                  <ComboboxList>
                    {(version: VersionEntry) => (
                      <ComboboxItem key={version.id} value={version.id}>
                        <Item size="xs" className="p-0">
                          <ItemContent>
                            <ItemTitle className="whitespace-nowrap">
                              {version.id}
                            </ItemTitle>
                            {showSnapshots && (
                              <ItemDescription>{version.type}</ItemDescription>
                            )}
                          </ItemContent>
                        </Item>
                      </ComboboxItem>
                    )}
                  </ComboboxList>
                </ComboboxContent>
              </Combobox>
              <div className="flex items-center gap-1">
                <Checkbox
                  id="instance-show-snapshots"
                  onCheckedChange={setShowSnapshots}
                />
                <FieldLabel htmlFor="instance-show-snapshots">
                  Show Snapshots
                </FieldLabel>
              </div>
            </div>
          </Field>

          <DialogFooter>
            <Button variant="ghost" render={<DialogClose />} />
            <Button
              type="submit"
              disabled={
                createInstanceMutation.isPending ||
                !name.trim() ||
                isLoadingVersions
              }
            >
              {createInstanceMutation.isPending ? "Creating..." : "Create"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
