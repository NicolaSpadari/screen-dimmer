import { resolve } from "path";
import { defineConfig } from "vite";
import Vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import { DirResolverHelper, dirResolver } from "vite-auto-import-resolvers";
import UnoCSS from "unocss/vite";

export default defineConfig({
    resolve: {
        alias: {
            "@": resolve(__dirname, "./src")
        }
    },
    plugins: [
        Vue({
            include: [/\.vue$/]
        }),
        UnoCSS(),
        DirResolverHelper(),
        AutoImport({
            imports: [
                "vue",
                { "@tauri-apps/api/event": ["listen"] }
            ],
            resolvers: [dirResolver()],
            dts: "src/auto-imports.d.ts"
        })
    ],
    build: {
        rollupOptions: {
            output: {
                inlineDynamicImports: true
            }
        }
    },
    server: {
        fs: {
            allow: [".."]
        },
        host: true,
        port: 8080,
        strictPort: true
    }
});
