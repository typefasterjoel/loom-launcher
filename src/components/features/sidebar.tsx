import { CatIcon } from "lucide-react";
import SettingsDialog from "./settings";
import { Separator } from "../ui/separator";
import CreateInstanceDialog from "./instances/new-instance-dialog";

export default function Sidebar() {
  return (
    <nav
      id="nav-sidebar"
      className="bg-card flex h-full w-12 flex-col justify-center p-2"
    >
      <div id="nav-top" className="flex grow flex-col items-center">
        <CatIcon />
      </div>
      <div className="-ms-0.5 flex shrink-0 grow-0 flex-col gap-1">
        <CreateInstanceDialog />
        <Separator />
        <SettingsDialog />
      </div>
    </nav>
  );
}
