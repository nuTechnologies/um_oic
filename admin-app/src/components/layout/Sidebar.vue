<template>
  <div
    class="flex grow flex-col gap-y-5 overflow-y-auto bg-white dark:bg-gray-800 px-6 shadow-lg"
    :class="mobile ? '' : 'pb-4'"
  >
    <!-- Logo section -->
    <div class="flex h-16 shrink-0 items-center border-b border-gray-200 dark:border-gray-700">
      <img class="h-8 w-auto" src="/logo.svg" alt="UM-OIC" />
      <span class="ml-3 text-lg font-semibold text-gray-900 dark:text-white">
        UM-OIC Admin
      </span>
    </div>

    <!-- Navigation -->
    <nav class="flex flex-1 flex-col">
      <ul role="list" class="flex flex-1 flex-col gap-y-7">
        <!-- Main navigation items -->
        <li>
          <ul role="list" class="-mx-2 space-y-1">
            <li v-for="item in navigationItems" :key="item.title">
              <!-- Single navigation item -->
              <router-link
                v-if="!item.children"
                :to="item.href"
                :class="getNavItemClasses(item)"
                @click="handleNavClick"
              >
                <component
                  :is="getIcon(item.icon)"
                  class="h-6 w-6 shrink-0"
                  aria-hidden="true"
                />
                {{ item.title }}

                <!-- Badge for item -->
                <Badge
                  v-if="item.badge"
                  :count="getBadgeCount(item.badge)"
                  :color="item.badge.color"
                  class="ml-auto"
                />
              </router-link>

              <!-- Collapsible navigation group -->
              <Disclosure v-else v-slot="{ open }" as="div">
                <DisclosureButton
                  :class="getGroupButtonClasses(item)"
                >
                  <component
                    :is="getIcon(item.icon)"
                    class="h-6 w-6 shrink-0"
                    aria-hidden="true"
                  />
                  {{ item.title }}
                  <ChevronRightIcon
                    :class="[
                      open ? 'rotate-90 text-gray-500' : 'text-gray-400',
                      'ml-auto h-5 w-5 shrink-0 transition-transform'
                    ]"
                    aria-hidden="true"
                  />
                </DisclosureButton>

                <DisclosurePanel as="ul" class="mt-1 px-2">
                  <li v-for="subItem in item.children" :key="subItem.title">
                    <router-link
                      :to="subItem.href"
                      :class="getSubItemClasses(subItem)"
                      @click="handleNavClick"
                    >
                      {{ subItem.title }}
                    </router-link>
                  </li>
                </DisclosurePanel>
              </Disclosure>
            </li>
          </ul>
        </li>

        <!-- Bottom section -->
        <li class="mt-auto">
          <div class="border-t border-gray-200 dark:border-gray-700 pt-4">
            <!-- User profile section -->
            <div class="flex items-center gap-x-3 px-2 py-3">
              <Avatar
                :user="authStore.user"
                size="sm"
                class="h-8 w-8"
              />
              <div class="flex-1 text-sm leading-6">
                <p class="font-semibold text-gray-900 dark:text-white">
                  {{ authStore.user?.full_name }}
                </p>
                <p class="text-gray-600 dark:text-gray-400 truncate">
                  {{ authStore.user?.email }}
                </p>
              </div>
            </div>

            <!-- User actions -->
            <div class="mt-2 space-y-1">
              <router-link
                to="/profile"
                class="group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 hover:bg-gray-50 dark:text-gray-300 dark:hover:bg-gray-700 transition-colors"
                @click="handleNavClick"
              >
                <UserIcon class="h-5 w-5 shrink-0" aria-hidden="true" />
                Profil
              </router-link>

              <button
                @click="handleLogout"
                class="group flex w-full gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 hover:bg-gray-50 dark:text-gray-300 dark:hover:bg-gray-700 transition-colors"
              >
                <ArrowLeftOnRectangleIcon class="h-5 w-5 shrink-0" aria-hidden="true" />
                Abmelden
              </button>
            </div>
          </div>
        </li>
      </ul>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { Disclosure, DisclosureButton, DisclosurePanel } from '@headlessui/vue'
import {
  ChevronRightIcon,
  UserIcon,
  ArrowLeftOnRectangleIcon
} from '@heroicons/vue/24/outline'
import {
  HomeIcon,
  UsersIcon,
  BuildingOfficeIcon,
  UserGroupIcon,
  KeyIcon,
  ShieldCheckIcon,
  ClockIcon,
  CogIcon
} from '@heroicons/vue/24/outline'

import { useAuthStore } from '@/stores/auth'
import { useUsersStore } from '@/stores/users'
import { useGroupsStore } from '@/stores/groups'
import { useSystemStore } from '@/stores/system'
import Avatar from '@/components/ui/Avatar.vue'
import Badge from '@/components/ui/Badge.vue'

interface Props {
  mobile?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  mobile: false
})

const emit = defineEmits<{
  close: []
}>()

const route = useRoute()
const authStore = useAuthStore()
const usersStore = useUsersStore()
const groupsStore = useGroupsStore()
const systemStore = useSystemStore()

// Icon mapping
const iconMap = {
  HomeIcon,
  UsersIcon,
  BuildingOfficeIcon,
  UserGroupIcon,
  KeyIcon,
  ShieldCheckIcon,
  ClockIcon,
  CogIcon
}

// Navigation items configuration
const navigationItems = [
  {
    title: 'Dashboard',
    icon: 'HomeIcon',
    href: '/dashboard',
    exact: true
  },
  {
    title: 'Benutzerverwaltung',
    icon: 'UsersIcon',
    children: [
      {
        title: 'Alle Benutzer',
        href: '/users',
        description: 'Benutzer verwalten und bearbeiten'
      },
      {
        title: 'Benutzer erstellen',
        href: '/users/create',
        description: 'Neuen Benutzer anlegen'
      },
      {
        title: 'Bulk-Import',
        href: '/users/import',
        description: 'CSV-Import für mehrere Benutzer'
      }
    ]
  },
  {
    title: 'Organisationen',
    icon: 'BuildingOfficeIcon',
    children: [
      {
        title: 'Übersicht',
        href: '/organizations',
        description: 'Alle Organisationen anzeigen'
      }
    ]
  },
  {
    title: 'Gruppen',
    icon: 'UserGroupIcon',
    href: '/groups',
    badge: {
      count: 'activeGroups',
      color: 'blue'
    }
  },
  {
    title: 'OAuth2 Clients',
    icon: 'KeyIcon',
    children: [
      {
        title: 'Client-Übersicht',
        href: '/clients',
        description: 'OAuth2-Anwendungen verwalten'
      },
      {
        title: 'Client erstellen',
        href: '/clients/create',
        description: 'Neue OAuth2-Anwendung registrieren'
      }
    ]
  },
  {
    title: 'Berechtigung & Claims',
    icon: 'ShieldCheckIcon',
    children: [
      {
        title: 'Claims Registry',
        href: '/claims',
        description: 'Verfügbare Claims verwalten'
      }
    ]
  },
  {
    title: 'Aktivitäten',
    icon: 'ClockIcon',
    children: [
      {
        title: 'Audit-Log',
        href: '/audit',
        description: 'System-Aktivitäten anzeigen'
      },
      {
        title: 'Aktive Sessions',
        href: '/sessions',
        description: 'Aktuelle Benutzersitzungen'
      },
      {
        title: 'Login-Statistiken',
        href: '/analytics',
        description: 'Anmelde-Analysen'
      }
    ]
  },
  {
    title: 'System',
    icon: 'CogIcon',
    children: [
      {
        title: 'Status',
        href: '/system/status',
        description: 'System-Gesundheit',
        badge: {
          status: 'healthStatus',
          colors: {
            healthy: 'green',
            degraded: 'yellow',
            unhealthy: 'red'
          }
        }
      },
      {
        title: 'Konfiguration',
        href: '/system/config',
        description: 'System-Einstellungen'
      }
    ]
  }
]

// Helper functions
const getIcon = (iconName: string) => iconMap[iconName as keyof typeof iconMap]

const isCurrentRoute = (href: string, exact = false) => {
  if (exact) {
    return route.path === href
  }
  return route.path.startsWith(href)
}

const isGroupActive = (children: any[]) => {
  return children.some(child => isCurrentRoute(child.href))
}

const getNavItemClasses = (item: any) => [
  isCurrentRoute(item.href, item.exact)
    ? 'bg-blue-50 text-blue-600 dark:bg-blue-900/50 dark:text-blue-400'
    : 'text-gray-700 hover:text-blue-600 hover:bg-gray-50 dark:text-gray-300 dark:hover:bg-gray-700',
  'group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold transition-colors'
]

const getGroupButtonClasses = (item: any) => [
  isGroupActive(item.children)
    ? 'bg-blue-50 text-blue-600 dark:bg-blue-900/50 dark:text-blue-400'
    : 'text-gray-700 hover:text-blue-600 hover:bg-gray-50 dark:text-gray-300 dark:hover:bg-gray-700',
  'flex items-center w-full text-left rounded-md p-2 gap-x-3 text-sm leading-6 font-semibold transition-colors'
]

const getSubItemClasses = (subItem: any) => [
  isCurrentRoute(subItem.href)
    ? 'bg-blue-50 text-blue-600 dark:bg-blue-900/50 dark:text-blue-400'
    : 'text-gray-600 hover:text-blue-600 hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-gray-700',
  'block rounded-md py-2 pr-2 pl-9 text-sm leading-6 transition-colors'
]

const getBadgeCount = (badge: any) => {
  switch (badge.count) {
    case 'activeGroups':
      return groupsStore.activeGroupsCount
    default:
      return 0
  }
}

const handleNavClick = () => {
  if (props.mobile) {
    emit('close')
  }
}

const handleLogout = async () => {
  await authStore.logout()
  if (props.mobile) {
    emit('close')
  }
}
</script>