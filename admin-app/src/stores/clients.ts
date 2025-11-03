import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/services/api'

export interface Client {
  id: string
  name: string
  description?: string
  client_secret: string
  redirect_uris: string[]
  grant_types: string[]
  response_types: string[]
  scope: string[]
  token_endpoint_auth_method: string
  created_at: string
  updated_at: string
}

export interface CreateClientRequest {
  name: string
  description?: string
  redirect_uris: string[]
  grant_types?: string[]
  response_types?: string[]
  scope?: string[]
  token_endpoint_auth_method?: string
}

export interface UpdateClientRequest {
  name?: string
  description?: string
  redirect_uris?: string[]
  grant_types?: string[]
  response_types?: string[]
  scope?: string[]
  token_endpoint_auth_method?: string
}

export interface ClientFilters {
  search?: string
  grant_type?: string
}

export const useClientsStore = defineStore('clients', () => {
  const clients = ref<Client[]>([])
  const currentClient = ref<Client | null>(null)
  const isLoading = ref(false)
  const filters = ref<ClientFilters>({})

  const filteredClients = computed(() => {
    let result = clients.value

    if (filters.value.search) {
      const search = filters.value.search.toLowerCase()
      result = result.filter(client =>
        client.name.toLowerCase().includes(search) ||
        (client.description && client.description.toLowerCase().includes(search))
      )
    }

    if (filters.value.grant_type) {
      result = result.filter(client =>
        client.grant_types.includes(filters.value.grant_type!)
      )
    }

    return result
  })

  const loadClients = async (): Promise<void> => {
    isLoading.value = true
    try {
      const response = await api.get('/clients')
      clients.value = response.data
    } catch (error) {
      console.error('Failed to load clients:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  const loadClient = async (clientId: string): Promise<Client> => {
    try {
      const response = await api.get(`/clients/${clientId}`)
      currentClient.value = response.data
      return response.data
    } catch (error) {
      console.error('Failed to load client:', error)
      throw error
    }
  }

  const createClient = async (clientData: CreateClientRequest): Promise<Client> => {
    try {
      const response = await api.post('/clients', clientData)
      const newClient = response.data
      clients.value.push(newClient)
      return newClient
    } catch (error) {
      console.error('Failed to create client:', error)
      throw error
    }
  }

  const updateClient = async (clientId: string, clientData: UpdateClientRequest): Promise<Client> => {
    try {
      const response = await api.put(`/clients/${clientId}`, clientData)
      const updatedClient = response.data

      const index = clients.value.findIndex(client => client.id === clientId)
      if (index !== -1) {
        clients.value[index] = updatedClient
      }

      if (currentClient.value?.id === clientId) {
        currentClient.value = updatedClient
      }

      return updatedClient
    } catch (error) {
      console.error('Failed to update client:', error)
      throw error
    }
  }

  const deleteClient = async (clientId: string): Promise<void> => {
    try {
      await api.delete(`/clients/${clientId}`)
      clients.value = clients.value.filter(client => client.id !== clientId)

      if (currentClient.value?.id === clientId) {
        currentClient.value = null
      }
    } catch (error) {
      console.error('Failed to delete client:', error)
      throw error
    }
  }

  const rotateSecret = async (clientId: string): Promise<string> => {
    try {
      const response = await api.post(`/clients/${clientId}/rotate-secret`)
      const newSecret = response.data.client_secret

      const index = clients.value.findIndex(client => client.id === clientId)
      if (index !== -1) {
        clients.value[index].client_secret = newSecret
      }

      if (currentClient.value?.id === clientId) {
        currentClient.value.client_secret = newSecret
      }

      return newSecret
    } catch (error) {
      console.error('Failed to rotate client secret:', error)
      throw error
    }
  }

  const setFilters = (newFilters: ClientFilters) => {
    filters.value = { ...filters.value, ...newFilters }
  }

  const clearFilters = () => {
    filters.value = {}
  }

  return {
    clients,
    currentClient,
    isLoading,
    filters,
    filteredClients,
    loadClients,
    loadClient,
    createClient,
    updateClient,
    deleteClient,
    rotateSecret,
    setFilters,
    clearFilters
  }
})