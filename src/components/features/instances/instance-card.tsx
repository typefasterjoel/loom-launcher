import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { cn } from "@/lib/utils";
import { Instance, ModLoader } from "@/types/instance";
import {
  AnvilIcon,
  BlocksIcon,
  BoxIcon,
  EllipsisVertical,
  JoystickIcon,
  PawPrintIcon,
  PlayIcon,
  ScrollIcon,
} from "lucide-react";
import { ReactElement } from "react";

interface InterfaceCardProps {
  instance: Instance;
  onPlay?: (instance: Instance) => void;
}

type ModLoaderSettings = {
  icon: ReactElement<SVGElement>;
  badgeColor: string;
  name: string;
};

const modLoaders: Record<ModLoader, ModLoaderSettings> = {
  vanilla: {
    icon: <BoxIcon />,
    badgeColor: "text-emerald-400 bg-emerald-500/20",
    name: "Vanilla",
  },
  fabric: {
    icon: <ScrollIcon />,
    badgeColor: "text-blue-400 bg-blue-500/20",
    name: "Fabric",
  },
  quilt: {
    icon: <BlocksIcon />,
    badgeColor: "text-purple-400 bg-purple-500/20",
    name: "Quilt",
  },
  neoforge: {
    icon: <PawPrintIcon />,
    badgeColor: "text-amber-400 bg-amber-500/20",
    name: "NeoForge",
  },
  forge: {
    icon: <AnvilIcon />,
    badgeColor: "text-red-400 bg-red-500/20",
    name: "Forge",
  },
};

export default function InstanceCard({ instance, onPlay }: InterfaceCardProps) {
  return (
    <Card>
      <CardContent className="flex items-center gap-3">
        <div
          data-slot="modpack-icon"
          className="bg-primary/20 size-18 shrink-0 grow-0 rounded-md"
        ></div>
        <div className="flex grow flex-col gap-1" data-slot="instance-info">
          <h3 className="mb-1 text-3xl font-semibold">{instance.name}</h3>
          <div className="flex items-center gap-2">
            <span
              className={cn(
                "inline-flex items-center gap-1 rounded px-2 py-1 text-xs [&_svg]:size-3",
                modLoaders[instance.modLoader].badgeColor,
              )}
            >
              {modLoaders[instance.modLoader].icon}
              {modLoaders[instance.modLoader].name}
            </span>
            <span className="bg-muted inline-block size-2 rounded-full"></span>
            <span className="text-muted-foreground inline-flex items-center gap-1 text-xs [&_svg]:size-3">
              <JoystickIcon />
              Minecraft {instance.mcVersion}
            </span>
          </div>
        </div>
        <div
          className="flex shrink-0 grow-0 gap-2 self-start"
          data-slot="isntance-actions"
        >
          <Button onClick={() => onPlay?.(instance)} size="lg">
            <PlayIcon /> Play
          </Button>
          <DropdownMenu>
            <DropdownMenuTrigger
              render={
                <Button variant="ghost" size="icon-lg">
                  <EllipsisVertical />
                </Button>
              }
            />
          </DropdownMenu>
        </div>
      </CardContent>
    </Card>
  );
}
