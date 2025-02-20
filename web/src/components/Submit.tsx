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
import { useEffect, useState } from "react";
import { useSharedStore } from "@/stores/shared";
import { axiosInstance } from "@/utils/axios";

export default function Submit() {
    const sharedStore = useSharedStore();

    const [loading, setLoading] = useState<boolean>(false);
    const [file, setFile] = useState<File | null>(null);

    const [captchaLoading, setCaptchaLoading] = useState<boolean>(false);
    const [captchaId, setCaptchaId] = useState<string>("");
    const [captchaAnswer, setCaptchaAnswer] = useState<string>("");

    useEffect(() => {
        const calculateWorker = new Worker(
            new URL("@/workers/pow.ts", import.meta.url),
            { type: "module" }
        );

        calculateWorker.onmessage = (e) => {
            const result = e.data;
            setCaptchaAnswer(result);
            setCaptchaLoading(false);
        };

        async function fetchCaptchaData() {
            setCaptchaLoading(true);
            const res = (await axiosInstance.get("/captcha")).data;
            const d = Number(res.data?.challenge?.split("#")[0]);
            const c = res.data?.challenge?.split("#")[1];
            setCaptchaId(res.data?.id);

            calculateWorker.postMessage({ c, d });
        }

        fetchCaptchaData();

        return () => {
            calculateWorker.terminate();
        };
    }, [sharedStore.refresh]);

    function handleSubmit() {
        if (!file || !captchaAnswer) {
            return;
        }

        setLoading(true);

        const formData = new FormData();
        formData.append("file", file);
        formData.append("captcha_id", captchaId!);
        formData.append("captcha_answer", captchaAnswer);
        axiosInstance
            .post("/submissions", formData)
            .then((res) => {
                const r = res.data;
                sharedStore.saveHistory(r.data);
            })
            .finally(() => {
                setFile(null);
                setLoading(false);
                sharedStore.setRefresh();
            });
    }

    return (
        <Stack direction={"row"} spacing={2}>
            <MuiFileInput
                value={file}
                onChange={(value) => setFile(value)}
                placeholder={"请选择文件"}
                disabled={loading}
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
                disabled={captchaLoading || loading}
                value={captchaAnswer}
                onChange={() => {}}
                aria-readonly
                slotProps={{
                    input: {
                        startAdornment: (
                            <InputAdornment position={"start"}>
                                <SmartToyIcon />
                            </InputAdornment>
                        ),
                        endAdornment: (
                            <IconButton
                                onClick={() => sharedStore.setRefresh()}
                            >
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
                loading={loading}
                sx={{
                    width: "10%",
                }}
                onClick={handleSubmit}
            >
                提交
            </Button>
        </Stack>
    );
}
