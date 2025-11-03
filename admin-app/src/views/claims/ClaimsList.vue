<template>
  <div>
    <!-- Page header -->
    <div class="page-header">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="page-title">Claims Verwaltung</h1>
          <p class="page-subtitle">
            Verwalten Sie verfügbare Claims und Berechtigungen
          </p>
        </div>
        <div class="flex space-x-3">
          <router-link
            to="/claims/registry"
            class="btn btn-secondary"
          >
            <CogIcon class="w-4 h-4 mr-2" />
            Registry
          </router-link>
          <button
            @click="refreshClaims"
            class="btn btn-primary"
          >
            <ArrowPathIcon class="w-4 h-4 mr-2" />
            Aktualisieren
          </button>
        </div>
      </div>
    </div>

    <!-- Claims overview -->
    <div class="grid grid-cols-1 gap-6 lg:grid-cols-3 mb-6">
      <div class="card">
        <div class="card-body">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <ShieldCheckIcon class="h-8 w-8 text-blue-600" />
            </div>
            <div class="ml-5 w-0 flex-1">
              <dl>
                <dt class="text-sm font-medium text-gray-500 dark:text-gray-400 truncate">
                  Verfügbare Claims
                </dt>
                <dd class="text-lg font-medium text-gray-900 dark:text-white">
                  {{ claimsStore.availableClaims.length }}
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
              <TagIcon class="h-8 w-8 text-green-600" />
            </div>
            <div class="ml-5 w-0 flex-1">
              <dl>
                <dt class="text-sm font-medium text-gray-500 dark:text-gray-400 truncate">
                  Claim-Typen
                </dt>
                <dd class="text-lg font-medium text-gray-900 dark:text-white">
                  {{ Object.keys(claimsStore.claimsByType).length }}
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
              <ClockIcon class="h-8 w-8 text-yellow-600" />
            </div>
            <div class="ml-5 w-0 flex-1">
              <dl>
                <dt class="text-sm font-medium text-gray-500 dark:text-gray-400 truncate">
                  Letzte Aktualisierung
                </dt>
                <dd class="text-sm font-medium text-gray-900 dark:text-white">
                  {{ claimsStore.registry?.last_updated ? formatDate(claimsStore.registry.last_updated) : '-' }}
                </dd>
              </dl>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Claims by type -->
    <div v-if="!claimsStore.isLoading && claimsStore.availableClaims.length > 0" class="space-y-6">
      <div v-for="(claims, type) in claimsStore.claimsByType" :key="type" class="card">
        <div class="card-header">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white capitalize">
            {{ type }} Claims ({{ claims.length }})
          </h3>
        </div>
        <div class="card-body">
          <div class="grid gap-4">
            <div
              v-for="claim in claims"
              :key="claim.key"
              class="border border-gray-200 dark:border-gray-700 rounded-lg p-4"
            >
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <h4 class="text-sm font-medium text-gray-900 dark:text-white font-mono">
                    {{ claim.key }}
                  </h4>
                  <p v-if="claim.description" class="mt-1 text-sm text-gray-600 dark:text-gray-400">
                    {{ claim.description }}
                  </p>
                  <div class="mt-2 flex flex-wrap gap-2">
                    <Badge
                      :text="claim.type"
                      :color="getTypeColor(claim.type)"
                    />
                    <Badge
                      v-if="claim.required"
                      text="Required"
                      color="red"
                    />
                    <Badge
                      v-if="claim.default_value !== undefined"
                      text="Has Default"
                      color="blue"
                    />
                  </div>
                  <div v-if="claim.default_value !== undefined" class="mt-2">
                    <span class="text-xs text-gray-500 dark:text-gray-400">Default: </span>
                    <code class="text-xs bg-gray-100 dark:bg-gray-800 px-1 py-0.5 rounded">
                      {{ formatValue(claim.default_value) }}
                    </code>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Loading state -->
    <div v-else-if="claimsStore.isLoading" class="card">
      <div class="card-body">
        <div class="space-y-4">
          <div v-for="i in 5" :key="i" class="flex items-start space-x-4">
            <div class="skeleton w-20 h-16"></div>
            <div class="flex-1 space-y-2">
              <div class="skeleton w-1/3 h-4"></div>
              <div class="skeleton w-2/3 h-3"></div>
              <div class="skeleton w-1/4 h-3"></div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-else class="card">
      <div class="card-body">
        <div class="text-center py-12">
          <ShieldCheckIcon class="mx-auto h-12 w-12 text-gray-400" />
          <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            Keine Claims konfiguriert
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Es sind noch keine Claims in der Registry definiert.
          </p>
          <div class="mt-6">
            <router-link
              to="/claims/registry"
              class="btn btn-primary"
            >
              Claims Registry konfigurieren
            </router-link>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import {
  ShieldCheckIcon,
  TagIcon,
  ClockIcon,
  CogIcon,
  ArrowPathIcon
} from '@heroicons/vue/24/outline'

import { useClaimsStore } from '@/stores/claims'
import Badge from '@/components/ui/Badge.vue'

const claimsStore = useClaimsStore()

const getTypeColor = (type: string) => {
  const colors: Record<string, string> = {
    'string': 'blue',
    'number': 'green',
    'boolean': 'purple',
    'array': 'yellow',
    'object': 'gray'
  }
  return colors[type] || 'gray'
}

const formatValue = (value: any) => {
  if (typeof value === 'string') return `"${value}"`
  if (typeof value === 'object') return JSON.stringify(value)
  return String(value)
}

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString('de-DE')
}

const refreshClaims = async () => {
  await claimsStore.loadClaimsRegistry()
}

onMounted(() => {
  claimsStore.loadClaimsRegistry()
})
</script>