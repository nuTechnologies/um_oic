import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/services/api'
import type { User } from './auth'

export interface CreateUserRequest {
  email: string
  first_name: string
  last_name: string
  password: string
  org: string
  admin?: string[]
  roles?: string[]
}

export interface UpdateUserRequest {
  first_name?: string
  last_name?: string
  admin?: string[]
  roles?: string[]
  org?: string
}

export interface UserFilters {
  org?: string
  role?: string
  admin_only?: boolean
  search?: string
}

export const useUsersStore = defineStore('users', () => {
  const users = ref<User[]>([])
  const currentUser = ref<User | null>(null)
  const isLoading = ref(false)
  const filters = ref<UserFilters>({})

  const filteredUsers = computed(() => {
    let result = users.value

    if (filters.value.org) {
      result = result.filter(user => user.org === filters.value.org)
    }

    if (filters.value.role) {
      result = result.filter(user => user.roles.includes(filters.value.role!))
    }

    if (filters.value.admin_only) {
      result = result.filter(user => user.admin.length > 0)
    }

    if (filters.value.search) {
      const search = filters.value.search.toLowerCase()
      result = result.filter(user =>
        user.email.toLowerCase().includes(search) ||
        user.first_name.toLowerCase().includes(search) ||
        user.last_name.toLowerCase().includes(search) ||
        user.full_name.toLowerCase().includes(search)
      )
    }

    return result
  })

  const usersByOrg = computed(() => {
    const grouped: Record<string, User[]> = {}
    users.value.forEach(user => {
      if (!grouped[user.org]) {
        grouped[user.org] = []
      }
      grouped[user.org].push(user)
    })
    return grouped
  })

  const loadUsers = async (orgId?: string): Promise<void> => {
    isLoading.value = true
    try {
      const params = orgId ? { org: orgId } : {}
      const response = await api.get('/users', { params })
      users.value = response.data
    } catch (error) {
      console.error('Failed to load users:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  const loadUser = async (userId: string): Promise<User> => {
    try {
      const response = await api.get(`/users/${userId}`)
      currentUser.value = response.data
      return response.data
    } catch (error) {
      console.error('Failed to load user:', error)
      throw error
    }
  }

  const createUser = async (userData: CreateUserRequest): Promise<User> => {
    try {
      const response = await api.post('/users', userData)
      const newUser = response.data

      // Add to local users array
      users.value.push(newUser)

      return newUser
    } catch (error) {
      console.error('Failed to create user:', error)
      throw error
    }
  }

  const updateUser = async (userId: string, userData: UpdateUserRequest): Promise<User> => {
    try {
      const response = await api.put(`/users/${userId}`, userData)
      const updatedUser = response.data

      // Update in local users array
      const index = users.value.findIndex(user => user.id === userId)
      if (index !== -1) {
        users.value[index] = updatedUser
      }

      // Update current user if it's the same
      if (currentUser.value?.id === userId) {
        currentUser.value = updatedUser
      }

      return updatedUser
    } catch (error) {
      console.error('Failed to update user:', error)
      throw error
    }
  }

  const deleteUser = async (userId: string): Promise<void> => {
    try {
      await api.delete(`/users/${userId}`)

      // Remove from local users array
      users.value = users.value.filter(user => user.id !== userId)

      // Clear current user if it's the same
      if (currentUser.value?.id === userId) {
        currentUser.value = null
      }
    } catch (error) {
      console.error('Failed to delete user:', error)
      throw error
    }
  }

  const setFilters = (newFilters: UserFilters) => {
    filters.value = { ...filters.value, ...newFilters }
  }

  const clearFilters = () => {
    filters.value = {}
  }

  return {
    users,
    currentUser,
    isLoading,
    filters,
    filteredUsers,
    usersByOrg,
    loadUsers,
    loadUser,
    createUser,
    updateUser,
    deleteUser,
    setFilters,
    clearFilters
  }
})