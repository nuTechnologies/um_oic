<template>
  <div>
    <!-- Page header -->
    <div class="page-header">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="page-title">Aktive Sessions</h1>
          <p class="page-subtitle">
            Übersicht über aktuelle Benutzersitzungen
          </p>
        </div>
        <button
          @click="refreshSessions"
          class="btn btn-primary"
        >
          <ArrowPathIcon class="w-4 h-4 mr-2" />
          Aktualisieren
        </button>
      </div>
    </div>

    <!-- Sessions stats -->
    <div class="grid grid-cols-1 gap-6 sm:grid-cols-3 mb-6">
      <div class="card">
        <div class="card-body">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <ComputerDesktopIcon class="h-8 w-8 text-green-600" />
            </div>
            <div class="ml-5 w-0 flex-1">
              <dl>
                <dt class="text-sm font-medium text-gray-500 dark:text-gray-400 truncate">
                  Aktive Sessions
                </dt>
                <dd class="text-lg font-medium text-gray-900 dark:text-white">
                  {{ activeSessions.length }}
                </dd>
              </dl>
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="card-body">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <UsersIcon class="h-8 w-8 text-blue-600" />
            </div>
            <div class="ml-5 w-0 flex-1">
              <dl>
                <dt class="text-sm font-medium text-gray-500 dark:text-gray-400 truncate">
                  Unique Users
                </dt>
                <dd class="text-lg font-medium text-gray-900 dark:text-white">
                  {{ uniqueUsers }}
                </dd>
              </dl>
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="card-body">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <DevicePhoneMobileIcon class="h-8 w-8 text-purple-600" />
            </div>
            <div class="ml-5 w-0 flex-1">
              <dl>
                <dt class="text-sm font-medium text-gray-500 dark:text-gray-400 truncate">
                  Mobile Sessions
                </dt>
                <dd class="text-lg font-medium text-gray-900 dark:text-white">
                  {{ mobileSessions }}
                </dd>
              </dl>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Sessions table -->
    <div class="card">
      <div class="card-header">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          Aktive Sessions ({{ activeSessions.length }})
        </h3>
      </div>

      <div class="overflow-hidden">
        <div v-if="isLoading" class="px-6 py-8">
          <div class="space-y-4">
            <div v-for="i in 5" :key="i" class="flex items-center space-x-4">
              <div class="skeleton w-10 h-10 rounded-full"></div>
              <div class="flex-1 space-y-2">
                <div class="skeleton w-1/3 h-4"></div>
                <div class="skeleton w-1/2 h-3"></div>
              </div>
              <div class="skeleton w-20 h-6"></div>
            </div>
          </div>
        </div>

        <div v-else-if="activeSessions.length === 0" class="px-6 py-8 text-center">
          <ComputerDesktopIcon class="mx-auto h-12 w-12 text-gray-400" />
          <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            Keine aktiven Sessions
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Es sind derzeit keine aktiven Benutzersitzungen vorhanden.
          </p>
        </div>

        <table v-else class="table">
          <thead>
            <tr>
              <th>Benutzer</th>
              <th>Session ID</th>
              <th>IP-Adresse</th>
              <th>User Agent</th>
              <th>Login-Zeit</th>
              <th>Letzte Aktivität</th>
              <th class="relative">
                <span class="sr-only">Aktionen</span>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="session in activeSessions" :key="session.id">
              <td>
                <div class="flex items-center">
                  <Avatar :user="{ email: session.user_email, full_name: session.user_name }" size="sm" class="mr-3" />
                  <div>
                    <div class="text-sm font-medium text-gray-900 dark:text-white">
                      {{ session.user_name }}
                    </div>
                    <div class="text-sm text-gray-500 dark:text-gray-400">
                      {{ session.user_email }}
                    </div>
                  </div>
                </div>
              </td>
              <td class="font-mono text-sm text-gray-600 dark:text-gray-300">
                {{ session.id.substring(0, 8) }}...
              </td>
              <td class="text-sm font-mono text-gray-900 dark:text-white">
                {{ session.ip_address }}
              </td>
              <td class="max-w-xs">
                <div class="text-sm text-gray-900 dark:text-white truncate">
                  {{ getBrowserName(session.user_agent) }}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  {{ getOSName(session.user_agent) }}
                </div>
              </td>
              <td class="text-sm text-gray-900 dark:text-white">
                {{ formatDateTime(session.created_at) }}
              </td>
              <td class="text-sm text-gray-900 dark:text-white">
                <div>{{ formatDateTime(session.last_activity) }}</div>
                <div class="text-xs text-gray-500">
                  {{ sessionsStore.formatLastActivity(session.last_activity) }}
                </div>
              </td>
              <td class="text-right text-sm font-medium">
                <button
                  @click="terminateSession(session)"
                  class="btn btn-sm btn-error"
                >
                  Beenden
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import {
  ComputerDesktopIcon,
  UsersIcon,
  DevicePhoneMobileIcon,
  ArrowPathIcon
} from '@heroicons/vue/24/outline'

import Avatar from '@/components/ui/Avatar.vue'
import { useSessionsStore } from '@/stores/sessions'

const sessionsStore = useSessionsStore()

const activeSessions = computed(() => sessionsStore.activeSessions)
const isLoading = computed(() => sessionsStore.isLoading)


const uniqueUsers = computed(() => {
  const userIds = new Set(activeSessions.value.map(s => s.user_id))
  return userIds.size
})

const mobileSessions = computed(() => {
  return activeSessions.value.filter(s =>
    s.user_agent.toLowerCase().includes('mobile') ||
    s.user_agent.toLowerCase().includes('android') ||
    s.user_agent.toLowerCase().includes('iphone')
  ).length
})

const getBrowserName = (userAgent: string) => {
  if (userAgent.includes('Chrome')) return 'Chrome'
  if (userAgent.includes('Firefox')) return 'Firefox'
  if (userAgent.includes('Safari')) return 'Safari'
  if (userAgent.includes('Edge')) return 'Edge'
  return 'Unknown'
}

const getOSName = (userAgent: string) => {
  if (userAgent.includes('Windows')) return 'Windows'
  if (userAgent.includes('Macintosh')) return 'macOS'
  if (userAgent.includes('Linux')) return 'Linux'
  if (userAgent.includes('Android')) return 'Android'
  if (userAgent.includes('iPhone')) return 'iOS'
  return 'Unknown'
}

const formatDateTime = (dateStr: string) => {
  return new Date(dateStr).toLocaleString('de-DE')
}

const getTimeAgo = (dateStr: string) => {
  const now = new Date()
  const then = new Date(dateStr)
  const diffMs = now.getTime() - then.getTime()
  const diffMins = Math.floor(diffMs / (1000 * 60))

  if (diffMins < 1) return 'Gerade eben'
  if (diffMins < 60) return `vor ${diffMins} Min`

  const diffHours = Math.floor(diffMins / 60)
  if (diffHours < 24) return `vor ${diffHours} Std`

  const diffDays = Math.floor(diffHours / 24)
  return `vor ${diffDays} Tag${diffDays !== 1 ? 'en' : ''}`
}

const refreshSessions = async () => {
  await sessionsStore.loadActiveSessions()
}

const terminateSession = async (session: any) => {
  const success = await sessionsStore.terminateSession(session.id)
  if (success) {
    // Session was successfully terminated and removed from store
  } else {
    console.error('Failed to terminate session')
  }
}

onMounted(() => {
  refreshSessions()
})
</script>