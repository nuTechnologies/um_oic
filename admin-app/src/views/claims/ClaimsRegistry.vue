<template>
  <div>
    <div class="page-header">
      <h1 class="page-title">Claims Registry</h1>
      <p class="page-subtitle">
        Zentrale Verwaltung der Claims-Registry
      </p>
    </div>

    <div class="card">
      <div class="card-body">
        <div class="text-center py-12">
          <ShieldCheckIcon class="mx-auto h-12 w-12 text-gray-400" />
          <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            Claims Registry
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Diese Funktion wird in einer zukünftigen Version implementiert.
          </p>
          <div class="mt-6">
            <router-link to="/claims" class="btn btn-primary">
              Zurück zur Übersicht
            </router-link>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ShieldCheckIcon } from '@heroicons/vue/24/outline'
</script>