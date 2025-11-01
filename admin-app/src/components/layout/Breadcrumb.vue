<template>
  <nav class="flex border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 py-3" aria-label="Breadcrumb">
    <ol role="list" class="mx-auto flex w-full max-w-screen-xl space-x-4 px-4 sm:px-6 lg:px-8">
      <li class="flex">
        <div class="flex items-center">
          <router-link
            to="/dashboard"
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          >
            <HomeIcon class="h-5 w-5 flex-shrink-0" aria-hidden="true" />
            <span class="sr-only">Dashboard</span>
          </router-link>
        </div>
      </li>

      <li v-for="(item, index) in breadcrumbItems" :key="item.name" class="flex">
        <div class="flex items-center">
          <ChevronRightIcon class="h-5 w-5 flex-shrink-0 text-gray-300 dark:text-gray-600" aria-hidden="true" />
          <router-link
            v-if="item.href && index !== breadcrumbItems.length - 1"
            :to="item.href"
            class="ml-4 text-sm font-medium text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            :aria-current="index === breadcrumbItems.length - 1 ? 'page' : undefined"
          >
            {{ item.name }}
          </router-link>
          <span
            v-else
            class="ml-4 text-sm font-medium text-gray-900 dark:text-white"
            :aria-current="index === breadcrumbItems.length - 1 ? 'page' : undefined"
          >
            {{ item.name }}
          </span>
        </div>
      </li>
    </ol>
  </nav>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { HomeIcon, ChevronRightIcon } from '@heroicons/vue/24/solid'

interface BreadcrumbItem {
  name: string
  href?: string
}

const route = useRoute()

const breadcrumbItems = computed<BreadcrumbItem[]>(() => {
  const pathSegments = route.path.split('/').filter(segment => segment !== '')
  const items: BreadcrumbItem[] = []

  // Build breadcrumb based on route meta or path segments
  if (route.meta?.breadcrumb) {
    return route.meta.breadcrumb as BreadcrumbItem[]
  }

  // Generate breadcrumb from path segments
  let currentPath = ''

  for (let i = 0; i < pathSegments.length; i++) {
    const segment = pathSegments[i]
    currentPath += `/${segment}`

    // Skip dashboard as it's already represented by home icon
    if (segment === 'dashboard') continue

    const isLast = i === pathSegments.length - 1

    items.push({
      name: getBreadcrumbLabel(segment, route.params),
      href: isLast ? undefined : currentPath
    })
  }

  return items
})

const getBreadcrumbLabel = (segment: string, params: any): string => {
  // Map route segments to readable labels
  const segmentMap: Record<string, string> = {
    'users': 'Benutzer',
    'create': 'Erstellen',
    'edit': 'Bearbeiten',
    'import': 'Import',
    'organizations': 'Organisationen',
    'clients': 'OAuth2 Clients',
    'claims': 'Claims Registry',
    'audit': 'Audit-Log',
    'sessions': 'Aktive Sessions',
    'analytics': 'Login-Statistiken',
    'system': 'System',
    'status': 'Status',
    'config': 'Konfiguration',
    'profile': 'Profil'
  }

  // Check if it's a dynamic parameter (like user ID)
  if (params && Object.values(params).includes(segment)) {
    return segment // Return the actual value for IDs
  }

  return segmentMap[segment] || segment.charAt(0).toUpperCase() + segment.slice(1)
}
</script>