<template>
  <div class="space-y-3">
    <div
      v-if="alerts.length === 0"
      class="text-center py-4"
    >
      <CheckCircleIcon class="mx-auto h-8 w-8 text-green-500" />
      <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
        Keine aktiven Warnungen
      </p>
    </div>

    <div
      v-for="alert in alerts"
      :key="alert.title"
      :class="[
        'rounded-md p-3 border-l-4',
        getAlertClasses(alert.type)
      ]"
    >
      <div class="flex">
        <div class="flex-shrink-0">
          <component
            :is="getAlertIcon(alert.type)"
            class="h-5 w-5"
            aria-hidden="true"
          />
        </div>
        <div class="ml-3 flex-1">
          <h3 :class="['text-sm font-medium', getTextClasses(alert.type)]">
            {{ alert.title }}
          </h3>
          <p :class="['mt-1 text-sm', getTextClasses(alert.type)]">
            {{ alert.message }}
          </p>
          <div v-if="alert.action" class="mt-2">
            <button
              @click="alert.actionFn?.()"
              :class="[
                'text-sm font-medium underline focus:outline-none',
                getTextClasses(alert.type)
              ]"
            >
              {{ alert.action }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  CheckCircleIcon,
  ExclamationTriangleIcon,
  XCircleIcon,
  InformationCircleIcon
} from '@heroicons/vue/24/outline'

interface Alert {
  type: 'info' | 'warning' | 'error' | 'success'
  title: string
  message: string
  action?: string
  actionFn?: () => void
}

interface Props {
  alerts: Alert[]
}

defineProps<Props>()

const getAlertIcon = (type: string) => {
  switch (type) {
    case 'success':
      return CheckCircleIcon
    case 'warning':
      return ExclamationTriangleIcon
    case 'error':
      return XCircleIcon
    default:
      return InformationCircleIcon
  }
}

const getAlertClasses = (type: string) => {
  switch (type) {
    case 'success':
      return 'bg-green-50 border-green-400 dark:bg-green-900/20 dark:border-green-500'
    case 'warning':
      return 'bg-yellow-50 border-yellow-400 dark:bg-yellow-900/20 dark:border-yellow-500'
    case 'error':
      return 'bg-red-50 border-red-400 dark:bg-red-900/20 dark:border-red-500'
    default:
      return 'bg-blue-50 border-blue-400 dark:bg-blue-900/20 dark:border-blue-500'
  }
}

const getTextClasses = (type: string) => {
  switch (type) {
    case 'success':
      return 'text-green-800 dark:text-green-200'
    case 'warning':
      return 'text-yellow-800 dark:text-yellow-200'
    case 'error':
      return 'text-red-800 dark:text-red-200'
    default:
      return 'text-blue-800 dark:text-blue-200'
  }
}
</script>