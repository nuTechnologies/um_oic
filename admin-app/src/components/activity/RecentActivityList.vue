<template>
  <div class="flow-root">
    <div v-if="loading" class="space-y-3 p-6">
      <div v-for="i in 5" :key="i" class="flex items-center space-x-3">
        <div class="skeleton w-8 h-8 rounded-full"></div>
        <div class="flex-1 space-y-2">
          <div class="skeleton w-3/4 h-4"></div>
          <div class="skeleton w-1/2 h-3"></div>
        </div>
      </div>
    </div>

    <ul v-else-if="activities.length > 0" role="list" class="-mb-8">
      <li v-for="(activity, activityIdx) in displayedActivities" :key="activity.id">
        <div class="relative pb-8">
          <span
            v-if="activityIdx !== displayedActivities.length - 1"
            class="absolute left-5 top-5 -ml-px h-full w-0.5 bg-gray-200 dark:bg-gray-700"
            aria-hidden="true"
          />
          <div class="relative flex items-start space-x-3">
            <div class="relative">
              <div
                :class="[
                  'flex h-10 w-10 items-center justify-center rounded-full ring-8 ring-white dark:ring-gray-800',
                  getActivityColor(activity.type)
                ]"
              >
                <component
                  :is="getActivityIcon(activity.type)"
                  class="h-5 w-5 text-white"
                  aria-hidden="true"
                />
              </div>
            </div>
            <div class="min-w-0 flex-1">
              <div>
                <div class="text-sm">
                  <span class="font-medium text-gray-900 dark:text-white">
                    {{ activity.user_email }}
                  </span>
                </div>
                <p class="mt-0.5 text-sm text-gray-500 dark:text-gray-400">
                  {{ getActivityDescription(activity) }}
                </p>
              </div>
              <div class="mt-2 text-sm text-gray-700 dark:text-gray-300">
                <p>{{ formatTimestamp(activity.timestamp) }}</p>
              </div>
            </div>
          </div>
        </div>
      </li>
    </ul>

    <div v-else class="p-6 text-center">
      <ClockIcon class="mx-auto h-12 w-12 text-gray-400" />
      <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
        Keine Aktivitäten
      </h3>
      <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
        Es wurden noch keine Aktivitäten aufgezeichnet.
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import {
  UserPlusIcon,
  ArrowRightOnRectangleIcon,
  PencilIcon,
  KeyIcon,
  ClockIcon
} from '@heroicons/vue/24/outline'
import type { RecentActivity } from '@/stores/stats'

interface Props {
  activities: RecentActivity[]
  loading?: boolean
  maxItems?: number
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  maxItems: 10
})

const displayedActivities = computed(() => {
  return props.activities.slice(0, props.maxItems)
})

const getActivityIcon = (type: string) => {
  switch (type) {
    case 'login':
      return ArrowRightOnRectangleIcon
    case 'registration':
      return UserPlusIcon
    case 'user_update':
      return PencilIcon
    case 'client_create':
      return KeyIcon
    default:
      return ClockIcon
  }
}

const getActivityColor = (type: string) => {
  switch (type) {
    case 'login':
      return 'bg-green-500'
    case 'registration':
      return 'bg-blue-500'
    case 'user_update':
      return 'bg-yellow-500'
    case 'client_create':
      return 'bg-purple-500'
    default:
      return 'bg-gray-500'
  }
}

const getActivityDescription = (activity: RecentActivity) => {
  switch (activity.type) {
    case 'login':
      return `hat sich angemeldet (${activity.org})`
    case 'registration':
      return `wurde registriert in ${activity.org}`
    case 'user_update':
      return `wurde aktualisiert ${activity.details ? `- ${activity.details}` : ''}`
    case 'client_create':
      return `hat OAuth2-Client erstellt ${activity.details ? `- ${activity.details}` : ''}`
    default:
      return activity.details || 'Unbekannte Aktivität'
  }
}

const formatTimestamp = (timestamp: string) => {
  const date = new Date(timestamp)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / (1000 * 60))
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60))
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

  if (diffMins < 1) {
    return 'Gerade eben'
  } else if (diffMins < 60) {
    return `vor ${diffMins} Min.`
  } else if (diffHours < 24) {
    return `vor ${diffHours} Std.`
  } else if (diffDays === 1) {
    return 'Gestern'
  } else if (diffDays < 7) {
    return `vor ${diffDays} Tagen`
  } else {
    return date.toLocaleDateString('de-DE', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric'
    })
  }
}
</script>