import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/services/api'

export interface Claim {
  key: string
  value: any
  type: 'string' | 'number' | 'boolean' | 'array' | 'object'
  description?: string
  required?: boolean
  default_value?: any
}

export interface ClaimsRegistry {
  version: string
  claims: Record<string, Claim>
  last_updated: string
}

export const useClaimsStore = defineStore('claims', () => {
  const registry = ref<ClaimsRegistry | null>(null)
  const isLoading = ref(false)

  const availableClaims = computed(() => {
    if (!registry.value) return []
    return Object.entries(registry.value.claims).map(([key, claim]) => ({
      key,
      ...claim
    }))
  })

  const claimsByType = computed(() => {
    const grouped: Record<string, Claim[]> = {}
    availableClaims.value.forEach(claim => {
      if (!grouped[claim.type]) {
        grouped[claim.type] = []
      }
      grouped[claim.type].push(claim)
    })
    return grouped
  })

  const loadClaimsRegistry = async (): Promise<void> => {
    isLoading.value = true
    try {
      const response = await api.get('/api/claims/registry')
      registry.value = response.data
    } catch (error) {
      console.error('Failed to load claims registry:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  const updateClaimsRegistry = async (newRegistry: ClaimsRegistry): Promise<void> => {
    try {
      const response = await api.put('/api/claims/registry', newRegistry)
      registry.value = response.data
    } catch (error) {
      console.error('Failed to update claims registry:', error)
      throw error
    }
  }

  return {
    registry,
    isLoading,
    availableClaims,
    claimsByType,
    loadClaimsRegistry,
    updateClaimsRegistry
  }
})