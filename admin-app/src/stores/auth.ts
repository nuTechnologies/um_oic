import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/services/api'

export interface User {
  id: string
  email: string
  first_name: string
  last_name: string
  full_name: string
  admin: string[] // Organizations or ["all"]
  org: string
  roles: string[]
  created_at: string
  updated_at: string
  avatar_url?: string
}

export interface LoginCredentials {
  email: string
  password: string
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const isLoading = ref(false)
  const token = ref<string | null>(localStorage.getItem('auth_token'))

  const isAuthenticated = computed(() => !!user.value && !!token.value)
  const isAdmin = computed(() => user.value?.admin.includes('all') || false)
  const adminOrgs = computed(() => user.value?.admin || [])

  const setToken = (newToken: string | null) => {
    token.value = newToken
    if (newToken) {
      localStorage.setItem('auth_token', newToken)
      api.setAuthToken(newToken)
    } else {
      localStorage.removeItem('auth_token')
      api.setAuthToken(null)
    }
  }

  const setUser = (newUser: User | null) => {
    user.value = newUser
  }

  const login = async (credentials: LoginCredentials): Promise<void> => {
    isLoading.value = true
    try {
      const response = await api.post('/auth/login', credentials)

      setToken(response.data.access_token)
      setUser(response.data.user)
    } catch (error) {
      setToken(null)
      setUser(null)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  const logout = async (): Promise<void> => {
    try {
      if (token.value) {
        await api.post('/auth/logout')
      }
    } catch (error) {
      console.warn('Logout request failed:', error)
    } finally {
      setToken(null)
      setUser(null)
    }
  }

  const checkAuth = async (): Promise<void> => {
    if (!token.value) {
      return
    }

    isLoading.value = true
    try {
      api.setAuthToken(token.value)
      const response = await api.get('/api/auth/me')
      setUser(response.data)
    } catch (error) {
      console.warn('Auth check failed:', error)
      setToken(null)
      setUser(null)
    } finally {
      isLoading.value = false
    }
  }

  const refreshToken = async (): Promise<void> => {
    if (!token.value) {
      throw new Error('No token to refresh')
    }

    try {
      const response = await api.post('/auth/refresh')
      setToken(response.data.access_token)
    } catch (error) {
      setToken(null)
      setUser(null)
      throw error
    }
  }

  // Check if user has admin access to specific organization
  const hasAdminAccess = (orgId?: string): boolean => {
    if (!user.value) return false
    if (user.value.admin.includes('all')) return true
    if (orgId && user.value.admin.includes(orgId)) return true
    return false
  }

  // Check if user has specific role
  const hasRole = (role: string): boolean => {
    if (!user.value) return false
    return user.value.roles.includes(role)
  }

  // Initialize auth on store creation
  if (token.value) {
    api.setAuthToken(token.value)
  }

  return {
    user,
    isLoading,
    token,
    isAuthenticated,
    isAdmin,
    adminOrgs,
    login,
    logout,
    checkAuth,
    refreshToken,
    hasAdminAccess,
    hasRole,
    setToken,
    setUser
  }
})