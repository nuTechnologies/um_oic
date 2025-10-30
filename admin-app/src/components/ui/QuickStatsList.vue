<template>
  <dl class="space-y-3">
    <div
      v-for="stat in stats"
      :key="stat.label"
      class="flex items-center justify-between"
    >
      <dt class="flex items-center text-sm text-gray-600 dark:text-gray-400">
        <component
          :is="getIcon(stat.icon)"
          :class="[
            'mr-2 h-4 w-4 flex-shrink-0',
            getIconColor(stat.color)
          ]"
          aria-hidden="true"
        />
        {{ stat.label }}
      </dt>
      <dd :class="['text-sm font-medium', getValueColor(stat.color)]">
        {{ stat.value }}
      </dd>
    </div>
  </dl>
</template>

<script setup lang="ts">
import {
  ClockIcon,
  ExclamationTriangleIcon,
  UserPlusIcon,
  ChartBarIcon
} from '@heroicons/vue/24/outline'

interface QuickStat {
  label: string
  value: string | number
  icon: string
  color?: 'gray' | 'red' | 'green' | 'blue' | 'yellow'
}

interface Props {
  stats: QuickStat[]
}

defineProps<Props>()

const iconMap = {
  ClockIcon,
  ExclamationTriangleIcon,
  UserPlusIcon,
  ChartBarIcon
}

const getIcon = (iconName: string) => {
  return iconMap[iconName as keyof typeof iconMap] || ClockIcon
}

const getIconColor = (color?: string) => {
  switch (color) {
    case 'red':
      return 'text-red-500'
    case 'green':
      return 'text-green-500'
    case 'blue':
      return 'text-blue-500'
    case 'yellow':
      return 'text-yellow-500'
    default:
      return 'text-gray-400'
  }
}

const getValueColor = (color?: string) => {
  switch (color) {
    case 'red':
      return 'text-red-600 dark:text-red-400'
    case 'green':
      return 'text-green-600 dark:text-green-400'
    case 'blue':
      return 'text-blue-600 dark:text-blue-400'
    case 'yellow':
      return 'text-yellow-600 dark:text-yellow-400'
    default:
      return 'text-gray-900 dark:text-white'
  }
}
</script>