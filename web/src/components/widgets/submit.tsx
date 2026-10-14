import { BotIcon, FileIcon, RefreshCcwIcon, SendIcon } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Field, FieldButton, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import type { Captcha } from "@/models/captcha";
import type { Submission } from "@/models/submission";
import { useSharedStore } from "@/storages/shared";
import type { WebResponse } from "@/types";
import { cn } from "@/utils";
import { alova } from "@/utils/alova";

function Submit() {
  const { refresh, saveHistory, setRefresh } = useSharedStore();

  const [loading, setLoading] = useState<boolean>(false);
  const [file, setFile] = useState<File | null>(null);

  const [captchaLoading, setCaptchaLoading] = useState<boolean>(false);
  const [captchaId, setCaptchaId] = useState<string>("");
  const [captchaAnswer, setCaptchaAnswer] = useState<string>("");

  const fileInputRef = useRef<HTMLInputElement | null>(null);

  function handleFileClick() {
    fileInputRef.current?.click();
  }

  function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
    if (e.target.files && e.target.files.length > 0) {
      setFile(e.target.files[0]);
    }
  }

  useEffect(() => {
    void refresh;

    let active = true;
    setCaptchaLoading(true);
    setCaptchaId("");
    setCaptchaAnswer("");

    const calculateWorker = new Worker(
      new URL("@/workers/pow.ts", import.meta.url),
      { type: "module" }
    );

    calculateWorker.onmessage = (e) => {
      if (!active) return;
      const result = e.data;
      setCaptchaAnswer(result);
      setCaptchaLoading(false);
    };

    calculateWorker.onerror = () => {
      if (!active) return;
      setCaptchaLoading(false);
      toast.error("验证计算失败，请刷新重试");
    };

    async function fetchCaptchaData() {
      try {
        const res = (await alova.Get<WebResponse<Captcha>>("/captcha")).data;
        if (!active) return;
        if (!res) {
          setCaptchaLoading(false);
          toast.error("无法获取验证信息，请刷新重试");
          return;
        }
        const [difficulty, c] = res.challenge.split("#");
        setCaptchaId(res.id);
        calculateWorker.postMessage({ c, d: Number(difficulty) });
      } catch {
        if (active) setCaptchaLoading(false);
      }
    }

    fetchCaptchaData();

    return () => {
      active = false;
      calculateWorker.terminate();
    };
  }, [refresh]);

  async function handleSubmit() {
    if (!file || !captchaId || !captchaAnswer || captchaLoading || loading) {
      return;
    }

    setLoading(true);

    const formData = new FormData();
    formData.append("file", file);
    formData.append("captcha_id", captchaId);
    formData.append("captcha_answer", captchaAnswer);
    try {
      const res = await alova.Post<WebResponse<Submission>>(
        "/submissions",
        formData
      );
      if (!res.data) {
        toast.error("服务器未返回提交记录，请重试");
        return;
      }
      saveHistory(res.data);
      setFile(null);
      if (fileInputRef.current) fileInputRef.current.value = "";
    } catch {
      // The request adapter displays errors; keep the selected file for retry.
    } finally {
      setLoading(false);
      setRefresh();
    }
  }

  return (
    <div className={cn(["flex", "gap-3", "items-center"])}>
      <Field className={cn(["flex-1/2"])}>
        <FieldIcon>
          <FileIcon />
        </FieldIcon>
        <Button
          className={cn([
            "h-12",
            "w-full",
            "rounded-l-none",
            "border",
            "justify-start",
            "bg-input",
            "hover:bg-input/80",
          ])}
          onClick={handleFileClick}
          disabled={loading}
        >
          <input
            type="file"
            accept=".csv,text/csv"
            aria-label="选择 CSV 文件"
            ref={fileInputRef}
            onChange={handleFileChange}
            className="hidden"
          />
          {file ? file.name : "选择文件"}
        </Button>
      </Field>
      <Field className={cn(["flex-1/2"])} disabled={captchaLoading || loading}>
        <FieldIcon>
          <BotIcon />
        </FieldIcon>
        <TextField value={captchaAnswer} aria-label="验证结果" readOnly />
        <FieldButton
          loading={captchaLoading}
          onClick={setRefresh}
          aria-label="刷新验证"
          icon={<RefreshCcwIcon />}
        />
      </Field>
      <Button
        size={"lg"}
        variant={"solid"}
        icon={<SendIcon />}
        loading={loading}
        disabled={!file || !captchaAnswer || captchaLoading}
        onClick={handleSubmit}
        className={cn(["h-12"])}
      >
        提交
      </Button>
    </div>
  );
}

export { Submit };
