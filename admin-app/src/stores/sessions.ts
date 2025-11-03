import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { ApiResponse } from './auth'

export interface ActiveSession {
  id: string
  user_id: string
  user_email: string
  user_name: string
  organization: string
  ip_address: string
  user_agent: string
  created_at: string
  last_activity: string
  expires_at: string
  is_current?: boolean
}

export const useSessionsStore = defineStore('sessions', () => {
  const sessions = ref<ActiveSession[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const activeSessions = computed(() => sessions.value)
  const sessionCount = computed(() => sessions.value.length)

  const loadActiveSessions = async () => {
    isLoading.value = true
    error.value = null

    try {
      const response = await fetch('/api/sessions/active', {
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
          'Content-Type': 'application/json'
        }
      })

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`)
      }

      const data: ApiResponse<ActiveSession[]> = await response.json()

      if (data.success && data.data) {
        sessions.value = data.data
      } else {
        throw new Error(data.message || 'Failed to load sessions')
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load active sessions'
      console.error('Failed to load active sessions:', err)
    } finally {
      isLoading.value = false
    }
  }

  const terminateSession = async (sessionId: string) => {
    try {
      const response = await fetch(`/api/sessions/${sessionId}`, {
        method: 'DELETE',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
          'Content-Type': 'application/json'
        }
      })

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`)
      }

      const data: ApiResponse<null> = await response.json()

      if (data.success) {
        // Remove session from local state
        sessions.value = sessions.value.filter(s => s.id !== sessionId)
        return true
      } else {
        throw new Error(data.message || 'Failed to terminate session')
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to terminate session'
      console.error('Failed to terminate session:', err)
      return false
    }
  }

  const getSessionsByUser = (userId: string) => {
    return sessions.value.filter(session => session.user_id === userId)
  }

  const getSessionsByOrganization = (org: string) => {
    return sessions.value.filter(session => session.organization === org)
  }

  const formatLastActivity = (timestamp: string) => {
    const date = new Date(timestamp)
    const now = new Date()
    const diffMs = now.getTime() - date.getTime()
    const diffMins = Math.floor(diffMs / 60000)
    const diffHours = Math.floor(diffMins / 60)
    const diffDays = Math.floor(diffHours / 24)

    if (diffMins < 1) return 'Gerade eben'
    if (diffMins < 60) return `vor ${diffMins} Min.`
    if (diffHours < 24) return `vor ${diffHours} Std.`
    if (diffDays === 1) return 'Gestern'
    return `vor ${diffDays} Tagen`
  }

  const isSessionExpiringSoon = (expiresAt: string) => {
    const expiry = new Date(expiresAt)
    const now = new Date()
    const diffMs = expiry.getTime() - now.getTime()
    const diffHours = diffMs / (1000 * 60 * 60)
    return diffHours < 24 && diffHours > 0
  }

  const clearError = () => {
    error.value = null
  }

  const $reset = () => {
    sessions.value = []
    isLoading.value = false
    error.value = null
  }

  return {
    // State
    sessions,
    isLoading,
    error,

    // Getters
    activeSessions,
    sessionCount,

    // Actions
    loadActiveSessions,
    terminateSession,
    getSessionsByUser,
    getSessionsByOrganization,
    formatLastActivity,
    isSessionExpiringSoon,
    clearError,
    $reset
  }
})