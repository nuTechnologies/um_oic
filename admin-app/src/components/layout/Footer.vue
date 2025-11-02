<template>
  <footer class="fixed bottom-0 left-0 right-0 bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 z-10">
    <div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
      <div class="md:flex md:items-center md:justify-between">
        <!-- Left side - Copyright -->
        <div class="flex justify-center md:order-2">
          <p class="text-center text-sm text-gray-500 dark:text-gray-400">
            &copy; {{ currentYear }} UM-OIC Admin. Alle Rechte vorbehalten.
          </p>
        </div>

        <!-- Right side - Links -->
        <div class="mt-4 flex justify-center space-x-6 md:order-3 md:mt-0">
          <a
            href="#"
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          >
            <span class="sr-only">System Status</span>
            <router-link to="/system/status" class="text-sm">
              System Status
            </router-link>
          </a>

          <a
            href="#"
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          >
            <span class="sr-only">Dokumentation</span>
            <span class="text-sm">Hilfe</span>
          </a>

          <div class="flex items-center space-x-2">
            <span class="text-sm text-gray-400">Version:</span>
            <span class="text-sm font-mono text-gray-600 dark:text-gray-300">
              {{ version }}
            </span>
          </div>
        </div>

        <!-- Left side - System health indicator -->
        <div class="mt-4 flex justify-center md:order-1 md:mt-0">
          <div class="flex items-center space-x-2">
            <div
              class="h-2 w-2 rounded-full"
              :class="{
                'bg-green-500': systemHealth === 'healthy',
                'bg-yellow-500': systemHealth === 'degraded',
                'bg-red-500': systemHealth === 'unhealthy',
                'bg-gray-400': systemHealth === 'unknown'
              }"
            ></div>
            <span class="text-sm text-gray-500 dark:text-gray-400">
              {{ systemHealthText }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useSystemStore } from '@/stores/system'

const systemStore = useSystemStore()

const version = ref('1.0.0')
const currentYear = new Date().getFullYear()

const systemHealth = computed(() => {
  return systemStore.systemStatus?.status || 'unknown'
})

const systemHealthText = computed(() => {
  switch (systemHealth.value) {
    case 'healthy':
      return 'Alle Systeme funktionsfähig'
    case 'degraded':
      return 'Eingeschränkte Funktionalität'
    case 'unhealthy':
      return 'Systemfehler'
    default:
      return 'Status unbekannt'
  }
})

onMounted(async () => {
  // Load system status periodically
  await systemStore.loadSystemStatus()

  // Set up periodic status checks
  setInterval(() => {
    systemStore.loadSystemStatus()
  }, 30000) // Check every 30 seconds
})
</script>