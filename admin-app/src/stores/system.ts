import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api } from '@/services/api'

export interface SystemStatus {
  status: 'healthy' | 'degraded' | 'unhealthy'
  auth_service: boolean
  admin_service: boolean
  auth_data_stale: boolean
  last_updated: string
  services: {
    [key: string]: {
      status: 'up' | 'down' | 'degraded'
      response_time?: number
      last_check: string
    }
  }
}

export const useSystemStore = defineStore('system', () => {
  const systemStatus = ref<SystemStatus | null>(null)
  const isLoading = ref(false)
  const isReloading = ref(false)

  const loadSystemStatus = async (): Promise<void> => {
    isLoading.value = true
    try {
      const response = await api.get('/system/status')
      systemStatus.value = response.data
    } catch (error) {
      console.error('Failed to load system status:', error)
      systemStatus.value = {
        status: 'unhealthy',
        auth_service: false,
        admin_service: false,
        auth_data_stale: true,
        last_updated: new Date().toISOString(),
        services: {}
      }
    } finally {
      isLoading.value = false
    }
  }

  const reloadAuthService = async (): Promise<void> => {
    isReloading.value = true
    try {
      await api.post('/system/reload-auth')
      // Refresh system status after reload
      await loadSystemStatus()
    } catch (error) {
      console.error('Failed to reload auth service:', error)
      throw error
    } finally {
      isReloading.value = false
    }
  }

  return {
    systemStatus,
    isLoading,
    isReloading,
    loadSystemStatus,
    reloadAuthService
  }
})