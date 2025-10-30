<template>
  <div class="sticky top-0 z-40 flex h-16 shrink-0 items-center gap-x-4 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 px-4 shadow-sm sm:gap-x-6 sm:px-6 lg:px-8">
    <!-- Mobile menu button -->
    <button
      type="button"
      class="-m-2.5 p-2.5 text-gray-700 dark:text-gray-300 lg:hidden"
      @click="$emit('toggleMobileMenu')"
    >
      <span class="sr-only">Seitenleiste öffnen</span>
      <Bars3Icon class="h-6 w-6" aria-hidden="true" />
    </button>

    <!-- Separator -->
    <div class="h-6 w-px bg-gray-200 dark:bg-gray-700 lg:hidden" aria-hidden="true" />

    <div class="flex flex-1 gap-x-4 self-stretch lg:gap-x-6">
      <!-- Search -->
      <form class="relative flex flex-1" action="#" method="GET">
        <label for="search-field" class="sr-only">Suchen</label>
        <MagnifyingGlassIcon
          class="pointer-events-none absolute inset-y-0 left-0 h-full w-5 text-gray-400"
          aria-hidden="true"
        />
        <input
          id="search-field"
          class="block h-full w-full border-0 py-0 pl-8 pr-0 text-gray-900 dark:text-white placeholder:text-gray-400 focus:ring-0 sm:text-sm bg-transparent"
          placeholder="Benutzer, Organisationen, Clients suchen..."
          type="search"
          name="search"
          v-model="searchQuery"
          @input="handleSearch"
        />
      </form>

      <div class="flex items-center gap-x-4 lg:gap-x-6">
        <!-- Notifications -->
        <button
          type="button"
          class="-m-2.5 p-2.5 text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 relative"
        >
          <span class="sr-only">Benachrichtigungen anzeigen</span>
          <BellIcon class="h-6 w-6" aria-hidden="true" />
          <span
            v-if="notificationCount > 0"
            class="absolute -top-0.5 -right-0.5 h-4 w-4 rounded-full bg-red-500 text-xs text-white flex items-center justify-center"
          >
            {{ notificationCount > 9 ? '9+' : notificationCount }}
          </span>
        </button>

        <!-- Dark mode toggle -->
        <button
          @click="toggleDarkMode"
          class="-m-2.5 p-2.5 text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
        >
          <span class="sr-only">{{ isDark ? 'Light mode' : 'Dark mode' }}</span>
          <SunIcon v-if="isDark" class="h-6 w-6" aria-hidden="true" />
          <MoonIcon v-else class="h-6 w-6" aria-hidden="true" />
        </button>

        <!-- Separator -->
        <div class="hidden lg:block lg:h-6 lg:w-px lg:bg-gray-200 dark:lg:bg-gray-700" aria-hidden="true" />

        <!-- Profile dropdown -->
        <Menu as="div" class="relative">
          <MenuButton class="-m-1.5 flex items-center p-1.5">
            <span class="sr-only">Benutzermenü öffnen</span>
            <Avatar
              :user="authStore.user"
              size="sm"
              class="h-8 w-8"
            />
            <span class="hidden lg:flex lg:items-center">
              <span class="ml-4 text-sm font-semibold leading-6 text-gray-900 dark:text-white" aria-hidden="true">
                {{ authStore.user?.full_name }}
              </span>
              <ChevronDownIcon class="ml-2 h-5 w-5 text-gray-400" aria-hidden="true" />
            </span>
          </MenuButton>
          <transition
            enter-active-class="transition ease-out duration-100"
            enter-from-class="transform opacity-0 scale-95"
            enter-to-class="transform opacity-100 scale-100"
            leave-active-class="transition ease-in duration-75"
            leave-from-class="transform opacity-100 scale-100"
            leave-to-class="transform opacity-0 scale-95"
          >
            <MenuItems class="absolute right-0 z-10 mt-2.5 w-32 origin-top-right rounded-md bg-white dark:bg-gray-800 py-2 shadow-lg ring-1 ring-gray-900/5 dark:ring-gray-700 focus:outline-none">
              <MenuItem v-slot="{ active }">
                <router-link
                  to="/profile"
                  :class="[
                    active ? 'bg-gray-50 dark:bg-gray-700' : '',
                    'block px-3 py-1 text-sm leading-6 text-gray-900 dark:text-white'
                  ]"
                >
                  Profil
                </router-link>
              </MenuItem>
              <MenuItem v-slot="{ active }">
                <button
                  @click="handleLogout"
                  :class="[
                    active ? 'bg-gray-50 dark:bg-gray-700' : '',
                    'block w-full text-left px-3 py-1 text-sm leading-6 text-gray-900 dark:text-white'
                  ]"
                >
                  Abmelden
                </button>
              </MenuItem>
            </MenuItems>
          </transition>
        </Menu>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Menu, MenuButton, MenuItem, MenuItems } from '@headlessui/vue'
import {
  Bars3Icon,
  BellIcon,
  ChevronDownIcon,
  MagnifyingGlassIcon,
  SunIcon,
  MoonIcon
} from '@heroicons/vue/24/outline'

import { useAuthStore } from '@/stores/auth'
import { useThemeStore } from '@/stores/theme'
import Avatar from '@/components/ui/Avatar.vue'

defineEmits<{
  toggleMobileMenu: []
}>()

const router = useRouter()
const authStore = useAuthStore()
const themeStore = useThemeStore()

const searchQuery = ref('')
const notificationCount = ref(0)

const isDark = computed(() => themeStore.isDark)

const toggleDarkMode = () => {
  themeStore.toggleDarkMode()
}

const handleSearch = () => {
  if (searchQuery.value.length > 2) {
    router.push({
      name: 'Search',
      query: { q: searchQuery.value }
    })
  }
}

const handleLogout = async () => {
  await authStore.logout()
}

onMounted(() => {
  // Load notification count
  // notificationCount.value = await notificationStore.getUnreadCount()
})
</script>