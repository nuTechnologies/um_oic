export interface AppConfig {
  api: {
    baseUrl: string
    timeout: number
  }
  auth: {
    serviceUrl: string
  }
}

export const config: AppConfig = {
  api: {
    baseUrl: '/api',
    timeout: 10000
  },
  auth: {
    serviceUrl: 'https://localhost:8443'
  }
}