import { paraglideVitePlugin } from "@inlang/paraglide-js";
import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

export default defineConfig({
  plugins: [
    sveltekit(),
    paraglideVitePlugin({ project: "./project.inlang", outdir: "./src/lib/paraglide" })
  ],
  server: {
    proxy: {
      "/api": { target: "http://127.0.0.1:3035", changeOrigin: true }
    }
  }
});
