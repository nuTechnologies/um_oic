<template>
  <div>
    <div class="page-header">
      <h1 class="page-title">System-Konfiguration</h1>
      <p class="page-subtitle">
        System-Einstellungen und Parameter verwalten
      </p>
    </div>

    <div class="card">
      <div class="card-body">
        <div class="text-center py-12">
          <CogIcon class="mx-auto h-12 w-12 text-gray-400" />
          <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            System-Konfiguration
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Diese Funktion wird in einer zukünftigen Version implementiert.
          </p>
          <div class="mt-6">
            <router-link to="/system/status" class="btn btn-primary">
              Zum System Status
            </router-link>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { CogIcon } from '@heroicons/vue/24/outline'
</script>