import { BotIcon, FileIcon, RefreshCcwIcon, SendIcon } from "lucide-react";
import { useEffect, useRef, useState } from "react";
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
  const sharedStore = useSharedStore();

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
    void sharedStore?.refresh;

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
      const res = (await alova.Get<WebResponse<Captcha>>("/captcha")).data;
      const d = Number(res?.challenge?.split("#")[0]);
      const c = res?.challenge?.split("#")[1];
      setCaptchaId(res?.id || "");

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
    alova
      .Post<WebResponse<Submission>>("/submissions", formData)
      .then((res) => {
        sharedStore.saveHistory(res.data!);
      })
      .finally(() => {
        setFile(null);
        setLoading(false);
        sharedStore.setRefresh();
      });
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
        >
          <input
            type="file"
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
        <TextField value={captchaAnswer} onChange={() => {}} readOnly />
        <FieldButton
          loading={captchaLoading}
          onClick={() => sharedStore.setRefresh()}
          icon={<RefreshCcwIcon />}
        />
      </Field>
      <Button
        size={"lg"}
        variant={"solid"}
        icon={<SendIcon />}
        loading={loading}
        onClick={handleSubmit}
        className={cn(["h-12"])}
      >
        提交
      </Button>
    </div>
  );
}

export { Submit };
