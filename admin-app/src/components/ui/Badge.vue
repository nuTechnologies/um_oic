<template>
  <span
    v-if="shouldShow"
    :class="[
      'inline-flex items-center rounded-full px-2 py-1 text-xs font-medium ring-1 ring-inset',
      colorClasses
    ]"
  >
    {{ displayText }}
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  count?: number | string
  color?: 'gray' | 'red' | 'yellow' | 'green' | 'blue' | 'indigo' | 'purple' | 'pink'
  text?: string
  dot?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  color: 'gray',
  dot: false
})

const shouldShow = computed(() => {
  if (props.text) return true
  if (props.dot) return true
  if (typeof props.count === 'number') return props.count > 0
  if (typeof props.count === 'string') return props.count.length > 0
  return false
})

const displayText = computed(() => {
  if (props.text) return props.text
  if (props.dot) return ''
  if (typeof props.count === 'number') {
    return props.count > 99 ? '99+' : props.count.toString()
  }
  return props.count || ''
})

const colorClasses = computed(() => {
  const colorMap = {
    gray: 'bg-gray-50 text-gray-600 ring-gray-500/10 dark:bg-gray-400/10 dark:text-gray-400 dark:ring-gray-400/20',
    red: 'bg-red-50 text-red-700 ring-red-600/10 dark:bg-red-400/10 dark:text-red-400 dark:ring-red-400/20',
    yellow: 'bg-yellow-50 text-yellow-800 ring-yellow-600/20 dark:bg-yellow-400/10 dark:text-yellow-500 dark:ring-yellow-400/20',
    green: 'bg-green-50 text-green-700 ring-green-600/20 dark:bg-green-500/10 dark:text-green-400 dark:ring-green-500/20',
    blue: 'bg-blue-50 text-blue-700 ring-blue-700/10 dark:bg-blue-400/10 dark:text-blue-400 dark:ring-blue-400/30',
    indigo: 'bg-indigo-50 text-indigo-700 ring-indigo-700/10 dark:bg-indigo-400/10 dark:text-indigo-400 dark:ring-indigo-400/30',
    purple: 'bg-purple-50 text-purple-700 ring-purple-700/10 dark:bg-purple-400/10 dark:text-purple-400 dark:ring-purple-400/30',
    pink: 'bg-pink-50 text-pink-700 ring-pink-700/10 dark:bg-pink-400/10 dark:text-pink-400 dark:ring-pink-400/20'
  }

  return colorMap[props.color]
})
</script>