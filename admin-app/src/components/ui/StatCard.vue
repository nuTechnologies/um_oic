<template>
  <div class="card">
    <div class="card-body">
      <div class="flex items-center">
        <div class="flex-shrink-0">
          <div
            :class="[
              'inline-flex items-center justify-center p-3 rounded-md shadow-lg',
              iconBackgroundClass
            ]"
          >
            <component
              :is="iconComponent"
              class="h-6 w-6 text-white"
              aria-hidden="true"
            />
          </div>
        </div>

        <div class="ml-5 w-0 flex-1">
          <dl>
            <dt class="text-sm font-medium text-gray-500 dark:text-gray-400 truncate">
              {{ title }}
            </dt>
            <dd>
              <div class="flex items-baseline">
                <div class="text-2xl font-semibold text-gray-900 dark:text-white">
                  <span v-if="loading" class="skeleton w-16 h-8"></span>
                  <span v-else>{{ formattedValue }}</span>
                </div>

                <div
                  v-if="change !== undefined && !loading"
                  :class="[
                    'ml-2 flex items-baseline text-sm font-semibold',
                    changeColorClass
                  ]"
                >
                  <ArrowUpIcon
                    v-if="changeType === 'increase' && change > 0"
                    class="self-center flex-shrink-0 h-4 w-4"
                    aria-hidden="true"
                  />
                  <ArrowDownIcon
                    v-else-if="changeType === 'decrease' && change > 0"
                    class="self-center flex-shrink-0 h-4 w-4"
                    aria-hidden="true"
                  />
                  <span class="sr-only">
                    {{ changeType === 'increase' ? 'Increased' : 'Decreased' }} by
                  </span>
                  {{ Math.abs(change) }}%
                </div>
              </div>
            </dd>
          </dl>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ArrowUpIcon, ArrowDownIcon } from '@heroicons/vue/24/solid'
import {
  UsersIcon,
  ComputerDesktopIcon,
  BuildingOfficeIcon,
  KeyIcon
} from '@heroicons/vue/24/outline'

interface Props {
  title: string
  value: number | string
  change?: number
  changeType?: 'increase' | 'decrease'
  icon: string
  color?: 'blue' | 'green' | 'yellow' | 'red' | 'purple' | 'orange'
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  color: 'blue',
  loading: false
})

const iconMap = {
  UsersIcon,
  ComputerDesktopIcon,
  BuildingOfficeIcon,
  KeyIcon
}

const iconComponent = computed(() => {
  return iconMap[props.icon as keyof typeof iconMap] || UsersIcon
})

const iconBackgroundClass = computed(() => {
  const colorMap = {
    blue: 'bg-blue-500',
    green: 'bg-green-500',
    yellow: 'bg-yellow-500',
    red: 'bg-red-500',
    purple: 'bg-purple-500',
    orange: 'bg-orange-500'
  }
  return colorMap[props.color]
})

const changeColorClass = computed(() => {
  if (props.change === undefined) return ''

  if (props.changeType === 'increase') {
    return props.change > 0 ? 'text-green-600' : 'text-red-600'
  } else {
    return props.change > 0 ? 'text-red-600' : 'text-green-600'
  }
})

const formattedValue = computed(() => {
  if (typeof props.value === 'number') {
    return props.value.toLocaleString()
  }
  return props.value
})
</script>