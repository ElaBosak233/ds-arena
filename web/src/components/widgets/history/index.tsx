import {
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  useReactTable,
} from "@tanstack/react-table";
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
import { columns } from "./columns";

function History() {
  const { history, saveHistory } = useSharedStore();

  const table = useReactTable<Submission>({
    data: history,
    columns,
    getCoreRowModel: getCoreRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
  });

  useEffect(() => {
    const interval = setInterval(async () => {
      let count = 0;
      history?.map(async (submission) => {
        if (submission.accuracy) {
          return;
        }

        const res = await alova.Get<WebResponse<Submission>>("/submissions", {
          params: {
            id: submission.id,
          },
        });

        count++;
        saveHistory(res.data!);
      });

      if (!count) {
        clearInterval(interval);
      }
    }, 1000);

    return () => clearInterval(interval);
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
                    {!header.isPlaceholder &&
                      flexRender(
                        header.column.columnDef.header,
                        header.getContext()
                      )}
                  </TableHead>
                );
              })}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {table.getRowModel().rows?.length ? (
            table.getRowModel().rows.map((row, index) => (
              <TableRow
                key={index}
                data-state={row.getIsSelected() && "selected"}
              >
                {row.getVisibleCells().map((cell) => (
                  <TableCell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
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
