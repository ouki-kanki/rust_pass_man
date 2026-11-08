import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev
export default defineConfig({
  plugins: [react()],
  // Prevent Vite from obscuring Tauri rust errors
  clearScreen: false,
  // Tauri expects a fixed port so it knows where to look for the frontend dev assets
  server: {
    port: 1420,
    strictPort: true,
  },
})
