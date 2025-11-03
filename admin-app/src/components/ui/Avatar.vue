<template>
  <div
    :class="[
      'inline-flex items-center justify-center rounded-full bg-gray-500 overflow-hidden',
      sizeClasses
    ]"
  >
    <img
      v-if="user?.avatar_url"
      :src="user.avatar_url"
      :alt="user.full_name || user.email"
      class="h-full w-full object-cover"
    />
    <span
      v-else
      :class="[
        'font-medium text-white',
        textSizeClasses
      ]"
    >
      {{ initials }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface User {
  full_name?: string
  first_name?: string
  last_name?: string
  email: string
  avatar_url?: string
}

interface Props {
  user?: User | null
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
}

const props = withDefaults(defineProps<Props>(), {
  size: 'md'
})

const sizeClasses = computed(() => {
  switch (props.size) {
    case 'xs':
      return 'h-6 w-6'
    case 'sm':
      return 'h-8 w-8'
    case 'md':
      return 'h-10 w-10'
    case 'lg':
      return 'h-12 w-12'
    case 'xl':
      return 'h-16 w-16'
    default:
      return 'h-10 w-10'
  }
})

const textSizeClasses = computed(() => {
  switch (props.size) {
    case 'xs':
      return 'text-xs'
    case 'sm':
      return 'text-sm'
    case 'md':
      return 'text-base'
    case 'lg':
      return 'text-lg'
    case 'xl':
      return 'text-xl'
    default:
      return 'text-base'
  }
})

const initials = computed(() => {
  if (!props.user) return '?'

  if (props.user.first_name && props.user.last_name) {
    return `${props.user.first_name.charAt(0)}${props.user.last_name.charAt(0)}`.toUpperCase()
  }

  if (props.user.full_name) {
    const names = props.user.full_name.split(' ')
    if (names.length >= 2) {
      return `${names[0].charAt(0)}${names[names.length - 1].charAt(0)}`.toUpperCase()
    }
    return names[0].charAt(0).toUpperCase()
  }

  return props.user.email?.charAt(0).toUpperCase() || '?'
})
</script>