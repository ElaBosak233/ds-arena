import { cn } from "@/utils";
import { Button } from "@/components/ui/button";
import { useApperanceStore } from "@/storages/appearance";
import { DatabaseIcon, MoonIcon, SunIcon } from "lucide-react";

function Navbar() {
  const { theme, setTheme } = useApperanceStore();

  return (
    <header
      className={cn([
        "sticky",
        "top-0",
        "h-16",
        "bg-card/80",
        "backdrop-blur-xs",
        "select-none",
        "border-b-[1px]",
        "flex",
        "items-center",
        "z-10",
      ])}
    >
      <div
        className={cn([
          "container",
          "ml-auto",
          "mr-auto",
          "pl-5",
          "pr-5",
          "max-w-[1300px]",
          "flex",
          "items-center",
          "justify-between",
        ])}
      >
        <div className={cn(["flex", "items-center", "gap-3"])}>
          <DatabaseIcon />
          <h1 className={cn(["text-xl", "font-semibold"])}>DS Arena</h1>
        </div>
        <div className={cn(["flex", "gap-3", "items-center"])}>
          <Button
            square
            icon={theme === "dark" ? <SunIcon /> : <MoonIcon />}
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
          />
        </div>
      </div>
    </header>
  );
}

export { Navbar };
