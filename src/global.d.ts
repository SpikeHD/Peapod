export {}

/* eslint-disable @typescript-eslint/no-explicit-any */
declare global {
  interface Window {
    __TAURI__: {
      core: {
        invoke: (cmd: string, args?: Record<string, any>) => Promise<any>
      }
      event: {
        listen: (event: string, handler: (event: TauriEvent) => void) => () => void
        emit: (event: string, payload: unknown) => void
        TauriEvent: {
          WINDOW_RESIZED: string
        }
      }
      shell: {
        open: (path: string) => void
      }
      app: {
        getVersion: () => Promise<string>
      }
      http: any
      webviewWindow: any
      [key: string]: unknown
    }
  }
}
