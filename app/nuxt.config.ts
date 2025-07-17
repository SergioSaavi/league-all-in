// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from '@tailwindcss/vite'
import { resolve } from 'path'

export default defineNuxtConfig({
  ssr: false,
  compatibilityDate: '2025-07-15',
  modules: [
    'shadcn-nuxt',
  ],
  shadcn: {
    prefix: '',
    componentDir: './components/ui',
  },
  // (optional) Enable the Nuxt devtools
  devtools: { enabled: true },
  css: ['~/assets/css/tailwind.css'],
  alias: {
    '@': resolve(__dirname, './'),
    '~': resolve(__dirname, './'),
    '@/components': resolve(__dirname, './components'),
    '@/lib': resolve(__dirname, './lib'),
  },

  vite: {
    plugins: [
      tailwindcss(),
    ],
  },

  ignore: ['**/src-tauri/**'],
})
