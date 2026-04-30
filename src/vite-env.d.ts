// Type declarations for Vite and CSS Modules
/// <reference types="vite/client" />

declare module '*.module.css' {
  const classes: { readonly [key: string]: string };
  export default classes;
}

// Type declarations for Tauri global
interface Window {
  __TAURI__?: unknown;
}
