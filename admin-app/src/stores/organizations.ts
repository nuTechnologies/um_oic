import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/services/api'

export interface Organization {
  id: string
  name: string
  description?: string
  domain?: string
  settings: {
    max_users?: number
    features?: string[]
    custom_claims?: Record<string, any>
  }
  created_at: string
  updated_at: string
  user_count?: number
  admin_count?: number
}

export interface CreateOrganizationRequest {
  name: string
  description?: string
  domain?: string
  settings?: {
    max_users?: number
    features?: string[]
    custom_claims?: Record<string, any>
  }
}

export interface UpdateOrganizationRequest {
  name?: string
  description?: string
  domain?: string
  settings?: {
    max_users?: number
    features?: string[]
    custom_claims?: Record<string, any>
  }
}

export interface OrganizationFilters {
  search?: string
  has_domain?: boolean
}

export const useOrganizationsStore = defineStore('organizations', () => {
  const organizations = ref<Organization[]>([])
  const currentOrganization = ref<Organization | null>(null)
  const isLoading = ref(false)
  const filters = ref<OrganizationFilters>({})

  const filteredOrganizations = computed(() => {
    let result = organizations.value

    if (filters.value.search) {
      const search = filters.value.search.toLowerCase()
      result = result.filter(org =>
        org.name.toLowerCase().includes(search) ||
        (org.description && org.description.toLowerCase().includes(search)) ||
        (org.domain && org.domain.toLowerCase().includes(search))
      )
    }

    if (filters.value.has_domain !== undefined) {
      result = result.filter(org =>
        filters.value.has_domain ? !!org.domain : !org.domain
      )
    }

    return result
  })

  const loadOrganizations = async (): Promise<void> => {
    isLoading.value = true
    try {
      const response = await api.get('/organizations')
      organizations.value = response.data
    } catch (error) {
      console.error('Failed to load organizations:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  const loadOrganization = async (orgId: string): Promise<Organization> => {
    try {
      const response = await api.get(`/organizations/${orgId}`)
      currentOrganization.value = response.data
      return response.data
    } catch (error) {
      console.error('Failed to load organization:', error)
      throw error
    }
  }

  const createOrganization = async (orgData: CreateOrganizationRequest): Promise<Organization> => {
    try {
      const response = await api.post('/organizations', orgData)
      const newOrg = response.data
      organizations.value.push(newOrg)
      return newOrg
    } catch (error) {
      console.error('Failed to create organization:', error)
      throw error
    }
  }

  const updateOrganization = async (orgId: string, orgData: UpdateOrganizationRequest): Promise<Organization> => {
    try {
      const response = await api.put(`/organizations/${orgId}`, orgData)
      const updatedOrg = response.data

      const index = organizations.value.findIndex(org => org.id === orgId)
      if (index !== -1) {
        organizations.value[index] = updatedOrg
      }

      if (currentOrganization.value?.id === orgId) {
        currentOrganization.value = updatedOrg
      }

      return updatedOrg
    } catch (error) {
      console.error('Failed to update organization:', error)
      throw error
    }
  }

  const deleteOrganization = async (orgId: string): Promise<void> => {
    try {
      await api.delete(`/organizations/${orgId}`)
      organizations.value = organizations.value.filter(org => org.id !== orgId)

      if (currentOrganization.value?.id === orgId) {
        currentOrganization.value = null
      }
    } catch (error) {
      console.error('Failed to delete organization:', error)
      throw error
    }
  }

  const setFilters = (newFilters: OrganizationFilters) => {
    filters.value = { ...filters.value, ...newFilters }
  }

  const clearFilters = () => {
    filters.value = {}
  }

  return {
    organizations,
    currentOrganization,
    isLoading,
    filters,
    filteredOrganizations,
    loadOrganizations,
    loadOrganization,
    createOrganization,
    updateOrganization,
    deleteOrganization,
    setFilters,
    clearFilters
  }
})