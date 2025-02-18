import { DataGrid, GridColDef } from "@mui/x-data-grid";

export default function Data() {
    const columns: Array<GridColDef> = [
        { field: "filename", headerName: "文件名", flex: 0.3 },
        { field: "accuracy", headerName: "正确率", flex: 0.2 },
        { field: "note", headerName: "备注", flex: 0.5 },
    ].map((col) => ({ ...col, disableColumnMenu: true, sortable: false }));

    return (
        <DataGrid
            columns={columns}
            pageSizeOptions={[]}
            localeText={{
                noRowsLabel: "无数据",
            }}
        />
    );
}
