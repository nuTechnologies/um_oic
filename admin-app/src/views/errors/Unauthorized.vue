<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
    <div class="sm:mx-auto sm:w-full sm:max-w-md">
      <div class="text-center">
        <ShieldExclamationIcon class="mx-auto h-24 w-24 text-red-500" />
        <h1 class="mt-4 text-2xl font-bold text-gray-900 dark:text-white">
          Zugriff verweigert
        </h1>
        <p class="mt-2 text-gray-600 dark:text-gray-400">
          Sie haben keine Berechtigung, auf diese Seite zuzugreifen.
        </p>
      </div>

      <div class="mt-8 flex justify-center space-x-4">
        <router-link
          to="/dashboard"
          class="btn btn-primary"
        >
          Zum Dashboard
        </router-link>
        <button
          @click="logout"
          class="btn btn-secondary"
        >
          Abmelden
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ShieldExclamationIcon } from '@heroicons/vue/24/outline'
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()

const logout = async () => {
  await authStore.logout()
}
</script>