import { useTable } from "@tanstack/react-table";
import { useEffect } from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import type { Submission } from "@/models/submission";
import { useSharedStore } from "@/storages/shared";
import type { WebResponse } from "@/types";
import { cn } from "@/utils";
import { alova } from "@/utils/alova";
import { columns, features } from "./columns";

function History() {
  const { history, saveHistory } = useSharedStore();

  const table = useTable({
    features,
    data: history,
    columns,
    getRowId: (submission) => submission.id,
  });

  useEffect(() => {
    const pending = history.filter((submission) => submission.accuracy == null);
    if (!pending.length) return;

    let active = true;
    let timer: ReturnType<typeof setTimeout>;

    async function poll() {
      await Promise.allSettled(
        pending.map(async (submission) => {
          const res = await alova.Get<WebResponse<Submission>>("/submissions", {
            params: { id: submission.id },
          });
          if (active && res.data && res.data.accuracy != null) {
            saveHistory(res.data);
          }
        })
      );
      if (active) timer = setTimeout(poll, 1000);
    }

    timer = setTimeout(poll, 1000);
    return () => {
      active = false;
      clearTimeout(timer);
    };
  }, [history, saveHistory]);

  return (
    <ScrollArea className={cn(["border", "rounded-md", "flex-1"])}>
      <Table className={cn(["text-foreground"])}>
        <TableHeader
          className={cn([
            "sticky",
            "top-0",
            "z-2",
            "bg-muted/70",
            "backdrop-blur-md",
          ])}
        >
          {table.getHeaderGroups().map((headerGroup) => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                return (
                  <TableHead key={header.id}>
                    {!header.isPlaceholder && (
                      <table.FlexRender header={header} />
                    )}
                  </TableHead>
                );
              })}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {table.getRowModel().rows?.length ? (
            table.getRowModel().rows.map((row) => (
              <TableRow key={row.id}>
                {row.getAllCells().map((cell) => (
                  <TableCell key={cell.id}>
                    <table.FlexRender cell={cell} />
                  </TableCell>
                ))}
              </TableRow>
            ))
          ) : (
            <TableRow>
              <TableCell
                colSpan={columns.length}
                className={cn(["h-24", "text-center"])}
              >
                哎呀，好像还没有提交过呢。
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
    </ScrollArea>
  );
}

export { History };
