import { ScrollArea } from "@/components/ui/scroll-area";
import { cn } from "@/utils";
import { Navbar } from "@/components/widgets/navbar";
import { History } from "@/components/widgets/history";
import { useApperanceStore } from "@/storages/appearance";
import { useEffect } from "react";
import { Submit } from "@/components/widgets/submit";

function App() {
  const { theme } = useApperanceStore();

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove("light", "dark");

    root.classList.add(theme);
  }, [theme]);

  return (
    <ScrollArea
      className={cn([
        "relative",
        "w-screen",
        "h-screen",
        "m-0",
        "overflow-auto",
      ])}
    >
      <Navbar />
      <main
        className={cn([
          "px-10",
          "py-5",
          "flex",
          "flex-col",
          "min-h-[calc(100vh-64px)]",
          "gap-5",
        ])}
      >
        <Submit />
        <History />
      </main>
    </ScrollArea>
  );
}

export default App;
