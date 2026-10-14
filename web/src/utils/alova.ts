import { createAlova } from "alova";
import adapterFetch from "alova/fetch";
import ReactHook from "alova/react";
import { toast } from "sonner";

export const alova = createAlova({
  baseURL: "/api",
  requestAdapter: adapterFetch(),
  timeout: 5000,
  shareRequest: true,
  statesHook: ReactHook,
  cacheFor: {
    GET: 0,
    POST: 0,
    PUT: 0,
    DELETE: 0,
  },
  responded: {
    onSuccess: async (response) => {
      if (!response.ok) {
        const body = await response.json().catch(() => null);
        const message =
          typeof body?.msg === "string"
            ? body.msg
            : `请求失败 (${response.status})`;
        toast.error(response.status === 502 ? "服务器离线" : "请求失败", {
          id: `http-error-${response.status}`,
          description: message,
        });
        throw new Error(message);
      }

      return response.json();
    },
    onError: (error) => {
      toast.error("网络请求失败", {
        id: "network-error",
        description: error.message,
      });
      throw error;
    },
  },
});
