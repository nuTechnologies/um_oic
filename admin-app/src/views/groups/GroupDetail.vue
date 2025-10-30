<template>
  <div>
    <div class="page-header">
      <h1 class="page-title">Gruppe Details</h1>
      <p class="page-subtitle">
        Gruppeninformationen und Mitglieder
      </p>
    </div>

    <div class="card">
      <div class="card-body">
        <div class="text-center py-12">
          <UserGroupIcon class="mx-auto h-12 w-12 text-gray-400" />
          <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            Gruppe: {{ id }}
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Diese Funktion wird in einer zukünftigen Version implementiert.
          </p>
          <div class="mt-6">
            <router-link to="/groups" class="btn btn-primary">
              Zurück zur Übersicht
            </router-link>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { UserGroupIcon } from '@heroicons/vue/24/outline'

interface Props {
  id: string
}

defineProps<Props>()
</script>