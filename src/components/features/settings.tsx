import {
  LanguagesIcon,
  MoonIcon,
  PcCaseIcon,
  SettingsIcon,
  SunIcon,
  SwatchBookIcon,
} from "lucide-react";
import { Button } from "../ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "../ui/dialog";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../ui/tabs";
import { RadioGroup, RadioGroupItem } from "../ui/radio-group";
import { Field, FieldContent, FieldLabel, FieldTitle } from "../ui/field";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";
import { useTheme } from "./theme-provider";
import { Theme } from "@tauri-apps/api/window";
import { Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";

const themeOptions = [
  {
    label: "Dark",
    value: "dark",
    id: "dark",
    icon: <MoonIcon className="size-3.5" />,
  },
  {
    label: "Light",
    value: "light",
    id: "light",
    icon: <SunIcon className="size-3.5" />,
  },
  {
    label: "System",
    value: "system",
    id: "system",
    icon: <PcCaseIcon className="size-3.5" />,
  },
];

export default function SettingsDialog() {
  const { theme, setTheme } = useTheme();

  return (
    <Dialog>
      <Tooltip>
        <TooltipTrigger
          render={
            <DialogTrigger
              render={
                <Button variant="ghost" size="icon-lg">
                  <SettingsIcon />
                </Button>
              }
            />
          }
        />
        <TooltipContent side="inline-end">Settings</TooltipContent>
      </Tooltip>

      <DialogContent className="h-[75vh] grid-rows-[auto_1fr_auto] gap-0 sm:max-w-[75vw]">
        <DialogHeader className="mb-4">
          <DialogTitle>Settings</DialogTitle>
          <DialogDescription className="sr-only">
            Application Settings for Modloader
          </DialogDescription>
        </DialogHeader>
        <Tabs
          orientation="vertical"
          className="-mx-4 grid min-h-0 grid-cols-[auto_1fr] gap-4 bg-none"
        >
          <TabsList className="ms-4 w-40 overflow-y-auto bg-transparent">
            <TabsTrigger value="appearance">
              <SwatchBookIcon />
              Appearance
            </TabsTrigger>
            <TabsTrigger value="language">
              <LanguagesIcon />
              Language
            </TabsTrigger>
          </TabsList>
          <div className="bg-muted/50 grow overflow-y-auto rounded-tl-2xl border-s border-t p-4">
            <TabsContent value="appearance" className="flex flex-col gap-2">
              <Card className="shadow/10 ring-0">
                <CardHeader>
                  <CardTitle>Theme</CardTitle>
                  <CardDescription>
                    Adjust the look of Modloader
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <RadioGroup
                    value={theme}
                    onValueChange={(newValue) => setTheme(newValue as Theme)}
                    className="grid-cols-3"
                  >
                    {themeOptions.map((option) => (
                      <FieldLabel key={option.id} htmlFor={option.id}>
                        <Field orientation="horizontal">
                          <FieldContent>
                            <FieldTitle>
                              {option.icon} {option.label}
                            </FieldTitle>
                          </FieldContent>
                          <RadioGroupItem value={option.value} id={option.id} />
                        </Field>
                      </FieldLabel>
                    ))}
                  </RadioGroup>
                </CardContent>
              </Card>
            </TabsContent>
          </div>
        </Tabs>
        <DialogFooter>
          <Button>Save</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
