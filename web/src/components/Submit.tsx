import {
    TextField,
    IconButton,
    Button,
    Stack,
    InputAdornment,
} from "@mui/material";
import { MuiFileInput } from "mui-file-input";
import RefreshIcon from "@mui/icons-material/Refresh";
import SendIcon from "@mui/icons-material/Send";
import SmartToyIcon from "@mui/icons-material/SmartToy";
import InsertDriveFileIcon from "@mui/icons-material/InsertDriveFile";
import { useState } from "react";

export default function Submit() {
    const [value, setValue] = useState<File | null>(null);
    const [captcha, setCaptcha] = useState<string>();

    return (
        <Stack direction={"row"} spacing={2}>
            <MuiFileInput
                value={value}
                onChange={(value) => setValue(value)}
                placeholder={"请选择文件"}
                InputProps={{
                    startAdornment: (
                        <InputAdornment position={"start"}>
                            <InsertDriveFileIcon />
                        </InputAdornment>
                    ),
                }}
                sx={{
                    width: "45%",
                }}
            />
            <TextField
                placeholder={"验证码"}
                slotProps={{
                    input: {
                        startAdornment: (
                            <InputAdornment position={"start"}>
                                <SmartToyIcon />
                            </InputAdornment>
                        ),
                        endAdornment: (
                            <IconButton>
                                <RefreshIcon />
                            </IconButton>
                        ),
                    },
                }}
                sx={{
                    width: "45%",
                }}
            />
            <Button
                variant={"contained"}
                size={"large"}
                startIcon={<SendIcon />}
                sx={{
                    width: "10%",
                }}
            >
                提交
            </Button>
        </Stack>
    );
}
