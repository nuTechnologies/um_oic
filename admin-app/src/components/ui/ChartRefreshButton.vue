<template>
  <button
    @click="$emit('refresh')"
    class="inline-flex items-center px-2.5 py-1.5 border border-gray-300 dark:border-gray-600 shadow-sm text-xs font-medium rounded text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
  >
    <ArrowPathIcon class="h-4 w-4" aria-hidden="true" />
    <span class="sr-only">Aktualisieren</span>
  </button>
</template>

<script setup lang="ts">
import { ArrowPathIcon } from '@heroicons/vue/24/outline'

defineEmits<{
  refresh: []
}>()
</script>