import { defineConfig } from "vite";
import React from "@vitejs/plugin-react-swc";
import path from "path";

export default defineConfig({
    plugins: [
        React({
            jsxImportSource: "@emotion/react",
        }),
    ],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "src"),
        },
    },
});
