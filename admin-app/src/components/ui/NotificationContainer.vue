<template>
  <div class="fixed top-4 right-4 z-50 space-y-2">
    <!-- Notifications will be rendered here -->
    <div v-for="notification in notifications" :key="notification.id" class="notification">
      <div
        :class="[
          'rounded-md p-4 shadow-lg border-l-4 max-w-sm',
          getNotificationClasses(notification.type)
        ]"
      >
        <div class="flex">
          <div class="flex-shrink-0">
            <component
              :is="getNotificationIcon(notification.type)"
              class="h-5 w-5"
              aria-hidden="true"
            />
          </div>
          <div class="ml-3 flex-1">
            <p :class="['text-sm font-medium', getTextClasses(notification.type)]">
              {{ notification.title }}
            </p>
            <p v-if="notification.message" :class="['mt-1 text-sm', getTextClasses(notification.type)]">
              {{ notification.message }}
            </p>
          </div>
          <div class="ml-4 flex-shrink-0">
            <button
              @click="removeNotification(notification.id)"
              :class="['rounded-md inline-flex focus:outline-none focus:ring-2 focus:ring-offset-2', getButtonClasses(notification.type)]"
            >
              <XMarkIcon class="h-5 w-5" aria-hidden="true" />
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import {
  CheckCircleIcon,
  ExclamationTriangleIcon,
  XCircleIcon,
  InformationCircleIcon,
  XMarkIcon
} from '@heroicons/vue/24/outline'

interface Notification {
  id: string
  type: 'success' | 'warning' | 'error' | 'info'
  title: string
  message?: string
  timeout?: number
}

const notifications = ref<Notification[]>([])

const getNotificationIcon = (type: string) => {
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

const getNotificationClasses = (type: string) => {
  switch (type) {
    case 'success':
      return 'bg-green-50 border-green-400 dark:bg-green-900/20'
    case 'warning':
      return 'bg-yellow-50 border-yellow-400 dark:bg-yellow-900/20'
    case 'error':
      return 'bg-red-50 border-red-400 dark:bg-red-900/20'
    default:
      return 'bg-blue-50 border-blue-400 dark:bg-blue-900/20'
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

const getButtonClasses = (type: string) => {
  switch (type) {
    case 'success':
      return 'text-green-400 hover:text-green-600 focus:ring-green-600'
    case 'warning':
      return 'text-yellow-400 hover:text-yellow-600 focus:ring-yellow-600'
    case 'error':
      return 'text-red-400 hover:text-red-600 focus:ring-red-600'
    default:
      return 'text-blue-400 hover:text-blue-600 focus:ring-blue-600'
  }
}

const removeNotification = (id: string) => {
  const index = notifications.value.findIndex(n => n.id === id)
  if (index > -1) {
    notifications.value.splice(index, 1)
  }
}

// Auto-remove notifications after timeout
const addNotification = (notification: Notification) => {
  notifications.value.push(notification)

  if (notification.timeout !== 0) {
    setTimeout(() => {
      removeNotification(notification.id)
    }, notification.timeout || 5000)
  }
}

// Expose methods for global use
defineExpose({
  addNotification,
  removeNotification
})
</script>