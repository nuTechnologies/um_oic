<template>
  <div class="w-full" :style="{ height: height + 'px' }">
    <div v-if="loading" class="w-full h-full">
      <ChartSkeleton :height="height" />
    </div>

    <div v-else-if="!data || data.length === 0" class="flex items-center justify-center h-full">
      <div class="text-center">
        <ChartBarIcon class="mx-auto h-12 w-12 text-gray-400" />
        <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
          Keine Daten verfügbar
        </h3>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Es sind keine Login-Daten für den ausgewählten Zeitraum verfügbar.
        </p>
      </div>
    </div>

    <div v-else class="w-full h-full flex">
      <!-- Pie chart placeholder -->
      <div class="flex-1 flex items-center justify-center">
        <div class="relative">
          <!-- Simple pie chart representation using CSS -->
          <div class="w-48 h-48 rounded-full relative overflow-hidden shadow-lg">
            <div
              v-for="(item, index) in data"
              :key="item.org"
              class="absolute inset-0 transition-all duration-300"
              :style="getPieSliceStyle(item, index)"
            >
              <!-- Hover tooltip area -->
              <div
                class="absolute inset-0 hover:opacity-80 cursor-pointer"
                :title="`${item.org}: ${item.count} (${item.percentage}%)`"
              ></div>
            </div>
          </div>

          <!-- Center label -->
          <div class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <div class="text-lg font-bold text-gray-900 dark:text-white">
                {{ totalLogins }}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-400">
                Gesamt
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Legend -->
      <div class="w-32 flex flex-col justify-center space-y-2 ml-6">
        <div
          v-for="(item, index) in data"
          :key="'legend-' + item.org"
          class="flex items-center"
        >
          <div
            class="w-3 h-3 rounded-sm mr-2"
            :style="{ backgroundColor: getColor(index) }"
          ></div>
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900 dark:text-white truncate">
              {{ item.org }}
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400">
              {{ item.count }} ({{ item.percentage }}%)
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ChartBarIcon } from '@heroicons/vue/24/outline'
import ChartSkeleton from '@/components/ui/ChartSkeleton.vue'
import type { LoginDistribution } from '@/stores/stats'

interface Props {
  data?: LoginDistribution[]
  loading?: boolean
  height?: number
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  height: 300
})

const colors = [
  '#3B82F6', // blue-500
  '#10B981', // emerald-500
  '#F59E0B', // amber-500
  '#EF4444', // red-500
  '#8B5CF6', // violet-500
  '#06B6D4', // cyan-500
  '#F97316', // orange-500
  '#84CC16'  // lime-500
]

const totalLogins = computed(() => {
  if (!props.data) return 0
  return props.data.reduce((sum, item) => sum + item.count, 0)
})

const getColor = (index: number) => {
  return colors[index % colors.length]
}

const getPieSliceStyle = (item: LoginDistribution, index: number) => {
  // Simple implementation - for a real app, you'd use a proper charting library
  const startAngle = props.data!
    .slice(0, index)
    .reduce((sum, prev) => sum + (prev.percentage * 3.6), 0)

  const endAngle = startAngle + (item.percentage * 3.6)

  return {
    background: `conic-gradient(from ${startAngle}deg, ${getColor(index)} 0deg ${endAngle - startAngle}deg, transparent ${endAngle - startAngle}deg)`
  }
}
</script>