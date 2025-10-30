import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api } from '@/services/api'

export interface UserStats {
  total_users: number
  growth_percentage: number
  active_users: number
  new_users_today: number
  new_users_week: number
}

export interface SessionStats {
  active_sessions: number
  change_percentage: number
  total_sessions_today: number
  unique_users_today: number
}

export interface OrgStats {
  total_organizations: number
  growth_percentage: number
  active_organizations: number
}

export interface ClientStats {
  total_clients: number
  change_percentage: number
  active_clients: number
}

export interface ActivityData {
  date: string
  logins: number
  registrations: number
  active_users: number
}

export interface LoginDistribution {
  org: string
  count: number
  percentage: number
}

export interface RecentActivity {
  id: string
  type: 'login' | 'registration' | 'user_update' | 'client_create'
  user_email: string
  org: string
  timestamp: string
  details?: string
}

export const useStatsStore = defineStore('stats', () => {
  const userStats = ref<UserStats | null>(null)
  const sessionStats = ref<SessionStats | null>(null)
  const orgStats = ref<OrgStats | null>(null)
  const clientStats = ref<ClientStats | null>(null)
  const activityData = ref<ActivityData[]>([])
  const loginDistribution = ref<LoginDistribution[]>([])
  const recentActivities = ref<RecentActivity[]>([])

  const lastLoginTime = ref<string | null>(null)
  const failedLoginsToday = ref<number>(0)
  const newUsersThisWeek = ref<number>(0)

  const loadUserStats = async (): Promise<void> => {
    try {
      const response = await api.get('/stats/users')
      userStats.value = response.data
    } catch (error) {
      console.error('Failed to load user stats:', error)
    }
  }

  const loadSessionStats = async (): Promise<void> => {
    try {
      const response = await api.get('/stats/sessions')
      sessionStats.value = response.data
    } catch (error) {
      console.error('Failed to load session stats:', error)
    }
  }

  const loadOrgStats = async (): Promise<void> => {
    try {
      const response = await api.get('/stats/organizations')
      orgStats.value = response.data
    } catch (error) {
      console.error('Failed to load organization stats:', error)
    }
  }

  const loadClientStats = async (): Promise<void> => {
    try {
      const response = await api.get('/stats/clients')
      clientStats.value = response.data
    } catch (error) {
      console.error('Failed to load client stats:', error)
    }
  }

  const loadActivityData = async (timeRange: string): Promise<void> => {
    try {
      const response = await api.get('/stats/activity', {
        params: { range: timeRange }
      })
      activityData.value = response.data
    } catch (error) {
      console.error('Failed to load activity data:', error)
    }
  }

  const loadLoginDistribution = async (timeRange: string): Promise<void> => {
    try {
      const response = await api.get('/stats/login-distribution', {
        params: { range: timeRange }
      })
      loginDistribution.value = response.data
    } catch (error) {
      console.error('Failed to load login distribution:', error)
    }
  }

  const loadRecentActivities = async (limit: number = 10): Promise<void> => {
    try {
      const response = await api.get('/stats/recent-activities', {
        params: { limit }
      })
      recentActivities.value = response.data
    } catch (error) {
      console.error('Failed to load recent activities:', error)
    }
  }

  const loadQuickStats = async (): Promise<void> => {
    try {
      const response = await api.get('/stats/quick')
      const data = response.data

      lastLoginTime.value = data.last_login_time
      failedLoginsToday.value = data.failed_logins_today
      newUsersThisWeek.value = data.new_users_week
    } catch (error) {
      console.error('Failed to load quick stats:', error)
    }
  }

  return {
    userStats,
    sessionStats,
    orgStats,
    clientStats,
    activityData,
    loginDistribution,
    recentActivities,
    lastLoginTime,
    failedLoginsToday,
    newUsersThisWeek,
    loadUserStats,
    loadSessionStats,
    loadOrgStats,
    loadClientStats,
    loadActivityData,
    loadLoginDistribution,
    loadRecentActivities,
    loadQuickStats
  }
})