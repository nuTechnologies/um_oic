<template>
  <div
    v-if="shouldShow"
    :class="[
      'rounded-md p-4',
      bannerClass
    ]"
  >
    <div class="flex">
      <div class="flex-shrink-0">
        <component
          :is="statusIcon"
          class="h-5 w-5"
          aria-hidden="true"
        />
      </div>
      <div class="ml-3">
        <h3 :class="['text-sm font-medium', textClass]">
          {{ statusTitle }}
        </h3>
        <div :class="['mt-2 text-sm', textClass]">
          <p>{{ statusMessage }}</p>
        </div>
        <div v-if="showActions" class="mt-4">
          <div class="-mx-2 -my-1.5 flex">
            <button
              v-if="status?.auth_data_stale"
              @click="$emit('reload')"
              type="button"
              :class="[
                'px-2 py-1.5 rounded-md text-sm font-medium focus:outline-none focus:ring-2 focus:ring-offset-2',
                buttonClass
              ]"
            >
              Auth-Service neu laden
            </button>
            <router-link
              to="/system/status"
              :class="[
                'ml-3 px-2 py-1.5 rounded-md text-sm font-medium focus:outline-none focus:ring-2 focus:ring-offset-2',
                linkClass
              ]"
            >
              Details anzeigen
            </router-link>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import {
  CheckCircleIcon,
  ExclamationTriangleIcon,
  XCircleIcon,
  InformationCircleIcon
} from '@heroicons/vue/24/outline'
import type { SystemStatus } from '@/stores/system'

interface Props {
  status: SystemStatus | null
}

const props = defineProps<Props>()

defineEmits<{
  reload: []
}>()

const shouldShow = computed(() => {
  if (!props.status) return false
  return props.status.status !== 'healthy' || props.status.auth_data_stale
})

const statusType = computed(() => {
  if (!props.status) return 'unknown'

  if (props.status.status === 'unhealthy') return 'error'
  if (props.status.status === 'degraded' || props.status.auth_data_stale) return 'warning'
  if (props.status.status === 'healthy' && !props.status.auth_data_stale) return 'success'

  return 'info'
})

const statusIcon = computed(() => {
  switch (statusType.value) {
    case 'success':
      return CheckCircleIcon
    case 'warning':
      return ExclamationTriangleIcon
    case 'error':
      return XCircleIcon
    default:
      return InformationCircleIcon
  }
})

const bannerClass = computed(() => {
  switch (statusType.value) {
    case 'success':
      return 'bg-green-50 dark:bg-green-900/50'
    case 'warning':
      return 'bg-yellow-50 dark:bg-yellow-900/50'
    case 'error':
      return 'bg-red-50 dark:bg-red-900/50'
    default:
      return 'bg-blue-50 dark:bg-blue-900/50'
  }
})

const textClass = computed(() => {
  switch (statusType.value) {
    case 'success':
      return 'text-green-800 dark:text-green-200'
    case 'warning':
      return 'text-yellow-800 dark:text-yellow-200'
    case 'error':
      return 'text-red-800 dark:text-red-200'
    default:
      return 'text-blue-800 dark:text-blue-200'
  }
})

const buttonClass = computed(() => {
  switch (statusType.value) {
    case 'warning':
      return 'bg-yellow-100 text-yellow-800 hover:bg-yellow-200 focus:ring-yellow-600 dark:bg-yellow-800 dark:text-yellow-100 dark:hover:bg-yellow-700'
    case 'error':
      return 'bg-red-100 text-red-800 hover:bg-red-200 focus:ring-red-600 dark:bg-red-800 dark:text-red-100 dark:hover:bg-red-700'
    default:
      return 'bg-blue-100 text-blue-800 hover:bg-blue-200 focus:ring-blue-600 dark:bg-blue-800 dark:text-blue-100 dark:hover:bg-blue-700'
  }
})

const linkClass = computed(() => {
  switch (statusType.value) {
    case 'warning':
      return 'bg-transparent text-yellow-800 hover:bg-yellow-100 focus:ring-yellow-600 dark:text-yellow-200 dark:hover:bg-yellow-800'
    case 'error':
      return 'bg-transparent text-red-800 hover:bg-red-100 focus:ring-red-600 dark:text-red-200 dark:hover:bg-red-800'
    default:
      return 'bg-transparent text-blue-800 hover:bg-blue-100 focus:ring-blue-600 dark:text-blue-200 dark:hover:bg-blue-800'
  }
})

const statusTitle = computed(() => {
  if (!props.status) return 'System Status unbekannt'

  if (props.status.status === 'unhealthy') {
    return 'System-Fehler erkannt'
  }

  if (props.status.auth_data_stale) {
    return 'Auth-Service Daten veraltet'
  }

  if (props.status.status === 'degraded') {
    return 'Eingeschränkte Funktionalität'
  }

  return 'System funktionsfähig'
})

const statusMessage = computed(() => {
  if (!props.status) return 'Der System-Status konnte nicht abgerufen werden.'

  if (props.status.status === 'unhealthy') {
    return 'Ein oder mehrere kritische Services sind nicht verfügbar. Bitte prüfen Sie den System-Status.'
  }

  if (props.status.auth_data_stale) {
    return 'Die Authentifizierungsdaten sind nicht aktuell. Ein Neustart des Auth-Service wird empfohlen.'
  }

  if (props.status.status === 'degraded') {
    return 'Einige Services laufen mit eingeschränkter Funktionalität.'
  }

  return 'Alle Services funktionieren normal.'
})

const showActions = computed(() => {
  return props.status && (props.status.status !== 'healthy' || props.status.auth_data_stale)
})
</script>