<template>
  <div class="w-full" :style="{ height: height + 'px' }">
    <div v-if="loading" class="w-full h-full">
      <ChartSkeleton :height="height" />
    </div>

    <div v-else-if="!data || !Array.isArray(data) || data.length === 0" class="flex items-center justify-center h-full">
      <div class="text-center">
        <ChartBarIcon class="mx-auto h-12 w-12 text-gray-400" />
        <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
          Keine Daten verfügbar
        </h3>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Es sind keine Aktivitätsdaten für den ausgewählten Zeitraum verfügbar.
        </p>
      </div>
    </div>

    <div v-else class="w-full h-full flex flex-col">
      <!-- Chart area -->
      <div class="flex-1 flex items-end justify-between space-x-2">
        <div
          v-for="(item, index) in data"
          :key="index"
          class="flex-1 flex flex-col items-center"
        >
          <!-- Bar -->
          <div class="w-full flex flex-col space-y-1">
            <!-- Logins bar -->
            <div
              class="bg-blue-500 rounded-sm transition-all duration-300 hover:bg-blue-600"
              :style="{
                height: getBarHeight(item.logins, maxValue) + 'px',
                minHeight: item.logins > 0 ? '2px' : '0px'
              }"
              :title="`${item.logins} Logins`"
            ></div>

            <!-- Registrations bar -->
            <div
              class="bg-green-500 rounded-sm transition-all duration-300 hover:bg-green-600"
              :style="{
                height: getBarHeight(item.registrations, maxValue) + 'px',
                minHeight: item.registrations > 0 ? '2px' : '0px'
              }"
              :title="`${item.registrations} Registrierungen`"
            ></div>
          </div>
        </div>
      </div>

      <!-- X-axis labels -->
      <div class="mt-2 flex justify-between text-xs text-gray-500 dark:text-gray-400">
        <span
          v-for="(item, index) in data"
          :key="'label-' + index"
          class="text-center"
        >
          {{ formatDate(item.date) }}
        </span>
      </div>

      <!-- Legend -->
      <div class="mt-4 flex justify-center space-x-4">
        <div class="flex items-center">
          <div class="w-3 h-3 bg-blue-500 rounded-sm mr-2"></div>
          <span class="text-xs text-gray-600 dark:text-gray-400">Logins</span>
        </div>
        <div class="flex items-center">
          <div class="w-3 h-3 bg-green-500 rounded-sm mr-2"></div>
          <span class="text-xs text-gray-600 dark:text-gray-400">Registrierungen</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ChartBarIcon } from '@heroicons/vue/24/outline'
import ChartSkeleton from '@/components/ui/ChartSkeleton.vue'
import type { ActivityData } from '@/stores/stats'

interface Props {
  data?: ActivityData[]
  loading?: boolean
  height?: number
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  height: 300
})

const maxValue = computed(() => {
  if (!props.data || !Array.isArray(props.data) || props.data.length === 0) return 0

  return Math.max(
    ...props.data.map(item => Math.max(item.logins, item.registrations))
  )
})

const getBarHeight = (value: number, max: number) => {
  if (max === 0) return 0
  const chartHeight = props.height - 60 // Reserve space for labels and legend
  return Math.max(2, (value / max) * chartHeight)
}

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('de-DE', {
    day: '2-digit',
    month: '2-digit'
  })
}
</script>