import { createAlova } from "alova";
import ReactHook from "alova/react";
import { toast } from "sonner";
import adapterFetch from "alova/fetch";

export const alova = createAlova({
    baseURL: "/api",
    requestAdapter: adapterFetch(),
    timeout: 5000,
    shareRequest: true,
    statesHook: ReactHook,
    cacheFor: {
        POST: 0,
        PUT: 0,
        DELETE: 0,
    },
    responded: {
        onSuccess: async (response, _method) => {
            if (response.status === 502) {
                toast.error("服务器离线", {
                    id: "502-backend-offline",
                    description: "服务器暂时无法处理请求",
                });
                return Promise.reject(response);
            }

            return response.json();
        },
    },
});
