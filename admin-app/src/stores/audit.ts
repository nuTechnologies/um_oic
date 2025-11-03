import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/services/api'

export interface AuditEvent {
  id: string
  user_id?: string
  event_type: string
  description: string
  ip_address?: string
  user_agent?: string
  metadata?: Record<string, any>
  timestamp: string
}

export interface AuditQuery {
  user_id?: string
  event_type?: string
  from?: string
  to?: string
  limit?: number
}

export const useAuditStore = defineStore('audit', () => {
  const events = ref<AuditEvent[]>([])
  const isLoading = ref(false)
  const filters = ref<AuditQuery>({})

  const filteredEvents = computed(() => {
    let result = events.value

    if (filters.value.user_id) {
      result = result.filter(event => event.user_id === filters.value.user_id)
    }

    if (filters.value.event_type) {
      result = result.filter(event => event.event_type === filters.value.event_type)
    }

    return result
  })

  const eventTypes = computed(() => {
    const types = new Set(events.value.map(event => event.event_type))
    return Array.from(types).sort()
  })

  const loadAuditEvents = async (query: AuditQuery = {}): Promise<void> => {
    isLoading.value = true
    try {
      const params = new URLSearchParams()

      if (query.user_id) params.append('user_id', query.user_id)
      if (query.event_type) params.append('event_type', query.event_type)
      if (query.from) params.append('from', query.from)
      if (query.to) params.append('to', query.to)
      if (query.limit) params.append('limit', query.limit.toString())

      const response = await api.get(`/api/audit/query?${params}`)
      events.value = response.data
    } catch (error) {
      console.error('Failed to load audit events:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  const setFilters = (newFilters: AuditQuery) => {
    filters.value = { ...filters.value, ...newFilters }
  }

  const clearFilters = () => {
    filters.value = {}
  }

  return {
    events,
    isLoading,
    filters,
    filteredEvents,
    eventTypes,
    loadAuditEvents,
    setFilters,
    clearFilters
  }
})