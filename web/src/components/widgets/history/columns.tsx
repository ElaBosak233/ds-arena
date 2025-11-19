import type { ColumnDef } from "@tanstack/react-table";
import type { Submission } from "@/models/submission";

const columns: ColumnDef<Submission>[] = [
  {
    accessorKey: "filename",
    header: "文件名",
  },
  {
    accessorKey: "accuracy",
    header: "正确率",
    cell: ({ row }) => {
      if (row.original.accuracy) {
        return `${row.original.accuracy * 100}%`;
      }
      return "...";
    },
  },
  {
    accessorKey: "note",
    header: "备注",
  },
];

export { columns };
