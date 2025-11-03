<template>
  <div>
    <!-- Page header -->
    <div class="page-header">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="page-title">Audit-Log</h1>
          <p class="page-subtitle">
            System-Aktivitäten und Sicherheitsereignisse
          </p>
        </div>
        <button
          @click="refreshEvents"
          class="btn btn-secondary"
        >
          <ArrowPathIcon class="w-4 h-4 mr-2" />
          Aktualisieren
        </button>
      </div>
    </div>

    <!-- Filters -->
    <div class="card mb-6">
      <div class="card-body">
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
          <div>
            <label for="user-filter" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Benutzer ID
            </label>
            <input
              id="user-filter"
              type="text"
              v-model="userFilter"
              class="form-input mt-1"
              placeholder="z.B. user-123"
            />
          </div>

          <div>
            <label for="event-type-filter" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Event-Typ
            </label>
            <select
              id="event-type-filter"
              v-model="eventTypeFilter"
              class="form-select mt-1"
            >
              <option value="">Alle Event-Typen</option>
              <option v-for="type in auditStore.eventTypes" :key="type" :value="type">
                {{ formatEventType(type) }}
              </option>
            </select>
          </div>

          <div>
            <label for="limit-filter" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Anzahl
            </label>
            <select
              id="limit-filter"
              v-model="limitFilter"
              class="form-select mt-1"
            >
              <option value="50">50 Einträge</option>
              <option value="100">100 Einträge</option>
              <option value="200">200 Einträge</option>
              <option value="500">500 Einträge</option>
            </select>
          </div>

          <div class="flex items-end">
            <button
              @click="clearFilters"
              class="btn btn-ghost"
            >
              Filter zurücksetzen
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Audit events table -->
    <div class="card">
      <div class="card-header">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          Audit-Ereignisse ({{ auditStore.filteredEvents.length }})
        </h3>
      </div>

      <div class="overflow-hidden">
        <div v-if="auditStore.isLoading" class="px-6 py-8">
          <div class="space-y-4">
            <div v-for="i in 10" :key="i" class="flex items-center space-x-4">
              <div class="skeleton w-20 h-4"></div>
              <div class="flex-1 space-y-2">
                <div class="skeleton w-2/3 h-4"></div>
                <div class="skeleton w-1/2 h-3"></div>
              </div>
              <div class="skeleton w-16 h-4"></div>
            </div>
          </div>
        </div>

        <div v-else-if="auditStore.filteredEvents.length === 0" class="px-6 py-8 text-center">
          <ClockIcon class="mx-auto h-12 w-12 text-gray-400" />
          <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            Keine Audit-Ereignisse gefunden
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            {{ userFilter || eventTypeFilter
               ? 'Keine Ereignisse entsprechen den Filterkriterien.'
               : 'Es sind noch keine Audit-Ereignisse vorhanden.' }}
          </p>
        </div>

        <div v-else class="overflow-x-auto">
          <table class="table">
            <thead>
              <tr>
                <th>Zeitstempel</th>
                <th>Event-Typ</th>
                <th>Beschreibung</th>
                <th>Benutzer</th>
                <th>IP-Adresse</th>
                <th class="relative">
                  <span class="sr-only">Details</span>
                </th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="event in auditStore.filteredEvents" :key="event.id">
                <td class="font-mono text-sm">
                  <div class="text-gray-900 dark:text-white">
                    {{ formatDate(event.timestamp) }}
                  </div>
                  <div class="text-xs text-gray-500">
                    {{ formatTime(event.timestamp) }}
                  </div>
                </td>
                <td>
                  <Badge
                    :text="formatEventType(event.event_type)"
                    :color="getEventTypeColor(event.event_type)"
                  />
                </td>
                <td class="max-w-md">
                  <div class="text-sm text-gray-900 dark:text-white">
                    {{ event.description }}
                  </div>
                  <div v-if="event.user_agent" class="text-xs text-gray-500 mt-1 truncate">
                    {{ event.user_agent }}
                  </div>
                </td>
                <td class="text-sm">
                  <div v-if="event.user_id" class="font-mono text-gray-900 dark:text-white">
                    {{ event.user_id }}
                  </div>
                  <span v-else class="text-gray-500 dark:text-gray-400">
                    System
                  </span>
                </td>
                <td class="text-sm font-mono text-gray-600 dark:text-gray-300">
                  {{ event.ip_address || '-' }}
                </td>
                <td class="text-right">
                  <button
                    @click="showEventDetails(event)"
                    class="btn btn-sm btn-ghost"
                  >
                    Details
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <!-- Event details modal -->
    <div
      v-if="selectedEvent"
      class="modal-overlay"
      @click="selectedEvent = null"
    >
      <div class="modal-panel max-w-2xl" @click.stop>
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">
            Audit-Event Details
          </h3>
        </div>
        <div class="px-6 py-4 space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Event-ID
              </label>
              <p class="mt-1 text-sm font-mono text-gray-900 dark:text-white">
                {{ selectedEvent.id }}
              </p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Zeitstempel
              </label>
              <p class="mt-1 text-sm text-gray-900 dark:text-white">
                {{ formatDateTime(selectedEvent.timestamp) }}
              </p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Event-Typ
              </label>
              <p class="mt-1 text-sm text-gray-900 dark:text-white">
                {{ selectedEvent.event_type }}
              </p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Benutzer-ID
              </label>
              <p class="mt-1 text-sm font-mono text-gray-900 dark:text-white">
                {{ selectedEvent.user_id || 'System' }}
              </p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                IP-Adresse
              </label>
              <p class="mt-1 text-sm font-mono text-gray-900 dark:text-white">
                {{ selectedEvent.ip_address || '-' }}
              </p>
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Beschreibung
            </label>
            <p class="mt-1 text-sm text-gray-900 dark:text-white">
              {{ selectedEvent.description }}
            </p>
          </div>

          <div v-if="selectedEvent.user_agent">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              User Agent
            </label>
            <p class="mt-1 text-sm font-mono text-gray-900 dark:text-white break-all">
              {{ selectedEvent.user_agent }}
            </p>
          </div>

          <div v-if="selectedEvent.metadata">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Metadata
            </label>
            <pre class="mt-1 text-xs bg-gray-100 dark:bg-gray-800 p-3 rounded-md overflow-auto">{{ JSON.stringify(selectedEvent.metadata, null, 2) }}</pre>
          </div>
        </div>
        <div class="px-6 py-4 bg-gray-50 dark:bg-gray-700 flex justify-end">
          <button
            @click="selectedEvent = null"
            class="btn btn-primary"
          >
            Schließen
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { ClockIcon, ArrowPathIcon } from '@heroicons/vue/24/outline'

import { useAuditStore } from '@/stores/audit'
import type { AuditEvent } from '@/stores/audit'
import Badge from '@/components/ui/Badge.vue'

const auditStore = useAuditStore()

const userFilter = ref('')
const eventTypeFilter = ref('')
const limitFilter = ref('50')
const selectedEvent = ref<AuditEvent | null>(null)

const formatEventType = (type: string) => {
  const typeMap: Record<string, string> = {
    'user_login': 'Benutzer-Login',
    'user_logout': 'Benutzer-Logout',
    'user_created': 'Benutzer erstellt',
    'user_updated': 'Benutzer geändert',
    'user_deleted': 'Benutzer gelöscht',
    'password_changed': 'Passwort geändert',
    'admin_action': 'Admin-Aktion',
    'system_error': 'System-Fehler',
    'api_access': 'API-Zugriff',
    'auth_failed': 'Auth-Fehler'
  }
  return typeMap[type] || type
}

const getEventTypeColor = (type: string) => {
  if (type.includes('error') || type.includes('failed')) return 'red'
  if (type.includes('login') || type.includes('created')) return 'green'
  if (type.includes('updated') || type.includes('changed')) return 'yellow'
  if (type.includes('deleted') || type.includes('logout')) return 'orange'
  return 'blue'
}

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString('de-DE')
}

const formatTime = (dateStr: string) => {
  return new Date(dateStr).toLocaleTimeString('de-DE')
}

const formatDateTime = (dateStr: string) => {
  return new Date(dateStr).toLocaleString('de-DE')
}

const clearFilters = () => {
  userFilter.value = ''
  eventTypeFilter.value = ''
  limitFilter.value = '50'
}

const refreshEvents = async () => {
  await loadEvents()
}

const showEventDetails = (event: AuditEvent) => {
  selectedEvent.value = event
}

const loadEvents = async () => {
  await auditStore.loadAuditEvents({
    user_id: userFilter.value || undefined,
    event_type: eventTypeFilter.value || undefined,
    limit: parseInt(limitFilter.value)
  })
}

watch([userFilter, eventTypeFilter, limitFilter], () => {
  auditStore.setFilters({
    user_id: userFilter.value || undefined,
    event_type: eventTypeFilter.value || undefined,
    limit: parseInt(limitFilter.value)
  })
  loadEvents()
})

onMounted(() => {
  loadEvents()
})
</script>