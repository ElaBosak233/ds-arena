import { useSharedStore } from "@/stores/shared";
import { axiosInstance } from "@/utils/axios";
import { DataGrid, GridColDef, GridRenderCellParams } from "@mui/x-data-grid";
import { useEffect } from "react";

export default function Data() {
    const sharedStore = useSharedStore();

    const columns: Array<GridColDef> = [
        { field: "filename", headerName: "文件名", flex: 0.3 },
        {
            field: "accuracy",
            headerName: "正确率",
            flex: 0.2,
            renderCell: (params: GridRenderCellParams<any, number>) => {
                if (params.value) {
                    return `${params.value * 100}%`;
                }
                return "...";
            },
        },
        { field: "note", headerName: "备注", flex: 0.5 },
    ].map((col) => ({ ...col, disableColumnMenu: true, sortable: false }));

    useEffect(() => {
        const interval = setInterval(async () => {
            let count = 0;
            sharedStore?.history?.map(async (submission) => {
                if (submission.accuracy != undefined) {
                    return;
                }

                const res = (
                    await axiosInstance.get("/submissions", {
                        params: {
                            id: submission.id,
                        },
                    })
                ).data;

                count++;
                sharedStore.saveHistory(res.data);
            });

            if (!count) {
                clearInterval(interval);
            }
        }, 1000);

        return () => clearInterval(interval);
    }, [sharedStore?.history]);

    return (
        <DataGrid
            columns={columns}
            rows={sharedStore?.history}
            pageSizeOptions={[]}
            autoPageSize
            localeText={{
                noRowsLabel: "无数据",
            }}
        />
    );
}
