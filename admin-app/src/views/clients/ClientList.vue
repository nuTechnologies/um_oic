<template>
  <div>
    <div class="page-header">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="page-title">OAuth2 Clients</h1>
          <p class="page-subtitle">
            Verwalten Sie OAuth2-Anwendungen und API-Zugriffe
          </p>
        </div>
        <router-link
          to="/clients/create"
          class="btn btn-primary"
        >
          <PlusIcon class="w-4 h-4 mr-2" />
          Client erstellen
        </router-link>
      </div>
    </div>

    <div class="card">
      <div class="card-body">
        <div class="text-center py-12">
          <KeyIcon class="mx-auto h-12 w-12 text-gray-400" />
          <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            OAuth2 Client-Verwaltung
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Diese Funktion wird in einer zukünftigen Version implementiert.
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { PlusIcon, KeyIcon } from '@heroicons/vue/24/outline'
</script>