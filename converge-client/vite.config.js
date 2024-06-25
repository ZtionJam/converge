import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

import path from 'path';
export default defineConfig(async () => ({

  resolve: {
    extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json', '.vue'],
    alias: {
      '@': path.resolve(__dirname, './src'),
    }
  },
  plugins: [vue()],


  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
