export function wait(ms: number) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms)
  })
}

export function waitForElm(selector: string, timeout = 5000): Promise<Element> {
  return new Promise((resolve, reject) => {
    const startTime = Date.now()
    const interval = setInterval(() => {
      const elm = document.querySelector(selector)

      if (elm) {
        clearInterval(interval)
        resolve(elm)
      } else if (Date.now() - startTime > timeout) {
        clearInterval(interval)
        reject(new Error(`Timeout: ${selector}`))
      }
    }, 100)
  })
}