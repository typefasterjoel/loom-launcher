import { getCurrentWindow } from "@tauri-apps/api/window";
import { MinusIcon, SquareIcon, XIcon } from "lucide-react";
import { Button } from "../ui/button";

const appWindow = getCurrentWindow();

export default function TitleBar() {
  return (
    <nav id="title-bar" className="bg-card relative z-9999 flex h-9 w-screen">
      <h1
        className="grow px-3 py-1.5 font-semibold select-none"
        data-tauri-drag-region
      >
        Modloader
      </h1>
      <div className="flex gap-px p-1">
        <Button
          onClick={() => appWindow.minimize()}
          variant="ghost"
          size="icon-sm"
        >
          <MinusIcon />
        </Button>
        <Button
          onClick={() => appWindow.toggleMaximize()}
          variant="ghost"
          size="icon-sm"
        >
          <SquareIcon />
        </Button>
        <Button
          onClick={() => appWindow.close()}
          variant="ghost"
          size="icon-sm"
        >
          <XIcon />
        </Button>
      </div>
    </nav>
  );
}
