<template>
  <div>
    <!-- Page header -->
    <div class="page-header">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="page-title">Benutzerverwaltung</h1>
          <p class="page-subtitle">
            Verwalten Sie Benutzerkonten und Zugriffsrechte
          </p>
        </div>
        <div class="flex space-x-3">
          <router-link
            to="/users/import"
            class="btn btn-secondary"
          >
            <DocumentArrowUpIcon class="w-4 h-4 mr-2" />
            Import
          </router-link>
          <router-link
            to="/users/create"
            class="btn btn-primary"
          >
            <PlusIcon class="w-4 h-4 mr-2" />
            Benutzer erstellen
          </router-link>
        </div>
      </div>
    </div>

    <!-- Filters -->
    <div class="card mb-6">
      <div class="card-body">
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
          <div>
            <label for="search" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Suchen
            </label>
            <input
              id="search"
              type="text"
              v-model="searchQuery"
              class="form-input mt-1"
              placeholder="Name, E-Mail..."
            />
          </div>

          <div>
            <label for="org-filter" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Organisation
            </label>
            <select
              id="org-filter"
              v-model="selectedOrg"
              class="form-select mt-1"
            >
              <option value="">Alle Organisationen</option>
              <option v-for="org in organizations" :key="org" :value="org">
                {{ org }}
              </option>
            </select>
          </div>

          <div>
            <label for="role-filter" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Rolle
            </label>
            <select
              id="role-filter"
              v-model="selectedRole"
              class="form-select mt-1"
            >
              <option value="">Alle Rollen</option>
              <option value="admin">Admin</option>
              <option value="adminread">Admin (Read-only)</option>
              <option value="user">User</option>
            </select>
          </div>

          <div class="flex items-end">
            <button
              @click="clearFilters"
              class="btn btn-ghost"
            >
              Filter zurücksetzen
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Users table -->
    <div class="card">
      <div class="card-header">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          Benutzer ({{ filteredUsers.length }})
        </h3>
      </div>

      <div class="overflow-hidden">
        <div v-if="usersStore.isLoading" class="px-6 py-8">
          <div class="space-y-4">
            <div v-for="i in 5" :key="i" class="flex items-center space-x-4">
              <div class="skeleton w-10 h-10 rounded-full"></div>
              <div class="flex-1 space-y-2">
                <div class="skeleton w-1/3 h-4"></div>
                <div class="skeleton w-1/2 h-3"></div>
              </div>
              <div class="skeleton w-20 h-6"></div>
            </div>
          </div>
        </div>

        <div v-else-if="filteredUsers.length === 0" class="px-6 py-8 text-center">
          <UsersIcon class="mx-auto h-12 w-12 text-gray-400" />
          <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            Keine Benutzer gefunden
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            {{ searchQuery || selectedOrg || selectedRole
               ? 'Keine Benutzer entsprechen den Filterkriterien.'
               : 'Erstellen Sie den ersten Benutzer.' }}
          </p>
        </div>

        <table v-else class="table">
          <thead>
            <tr>
              <th>Benutzer</th>
              <th>Organisation</th>
              <th>Rollen</th>
              <th>Admin-Berechtigung</th>
              <th>Erstellt</th>
              <th class="relative">
                <span class="sr-only">Aktionen</span>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="user in filteredUsers" :key="user.id">
              <td>
                <div class="flex items-center">
                  <Avatar :user="user" size="sm" class="mr-3" />
                  <div>
                    <div class="text-sm font-medium text-gray-900 dark:text-white">
                      {{ user.full_name }}
                    </div>
                    <div class="text-sm text-gray-500 dark:text-gray-400">
                      {{ user.email }}
                    </div>
                  </div>
                </div>
              </td>
              <td class="text-sm text-gray-900 dark:text-white">
                <Badge :text="user.org" color="gray" />
              </td>
              <td>
                <div class="flex flex-wrap gap-1">
                  <Badge
                    v-for="role in user.roles"
                    :key="role"
                    :text="role"
                    :color="getRoleColor(role)"
                  />
                </div>
              </td>
              <td>
                <div v-if="user.admin.length > 0">
                  <Badge
                    v-if="user.admin.includes('all')"
                    text="Global Admin"
                    color="red"
                  />
                  <div v-else class="flex flex-wrap gap-1">
                    <Badge
                      v-for="org in user.admin.slice(0, 2)"
                      :key="org"
                      :text="org"
                      color="blue"
                    />
                    <Badge
                      v-if="user.admin.length > 2"
                      :text="`+${user.admin.length - 2}`"
                      color="gray"
                    />
                  </div>
                </div>
                <span v-else class="text-sm text-gray-500 dark:text-gray-400">
                  Keine
                </span>
              </td>
              <td class="text-sm text-gray-500 dark:text-gray-400">
                {{ formatDate(user.created_at) }}
              </td>
              <td class="text-right text-sm font-medium">
                <div class="flex justify-end space-x-2">
                  <button
                    @click="$router.push(`/users/${user.id}`)"
                    class="btn btn-sm btn-secondary"
                  >
                    Bearbeiten
                  </button>
                  <button
                    @click="showUserClaims(user)"
                    class="btn btn-sm btn-ghost"
                  >
                    Claims
                  </button>
                  <button
                    @click="confirmDelete(user)"
                    class="btn btn-sm btn-error"
                  >
                    Löschen
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Delete confirmation modal -->
    <div
      v-if="userToDelete"
      class="modal-overlay"
      @click="userToDelete = null"
    >
      <div class="modal-panel" @click.stop>
        <div class="px-6 py-4">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">
            Benutzer löschen
          </h3>
          <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
            Sind Sie sicher, dass Sie den Benutzer <strong>{{ userToDelete.full_name }}</strong> löschen möchten?
            Diese Aktion kann nicht rückgängig gemacht werden.
          </p>
        </div>
        <div class="px-6 py-4 bg-gray-50 dark:bg-gray-700 flex justify-end space-x-3">
          <button
            @click="userToDelete = null"
            class="btn btn-secondary"
          >
            Abbrechen
          </button>
          <button
            @click="deleteUser"
            class="btn btn-error"
          >
            Löschen
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import {
  PlusIcon,
  DocumentArrowUpIcon,
  UsersIcon
} from '@heroicons/vue/24/outline'

import { useUsersStore } from '@/stores/users'
import type { User } from '@/stores/auth'
import Avatar from '@/components/ui/Avatar.vue'
import Badge from '@/components/ui/Badge.vue'

const usersStore = useUsersStore()

const searchQuery = ref('')
const selectedOrg = ref('')
const selectedRole = ref('')
const userToDelete = ref<User | null>(null)

const filteredUsers = computed(() => {
  let users = usersStore.users

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    users = users.filter(user =>
      user.full_name.toLowerCase().includes(query) ||
      user.email.toLowerCase().includes(query)
    )
  }

  if (selectedOrg.value) {
    users = users.filter(user => user.org === selectedOrg.value)
  }

  if (selectedRole.value) {
    users = users.filter(user => user.roles.includes(selectedRole.value))
  }

  return users
})

const organizations = computed(() => {
  const orgs = new Set(usersStore.users.map(user => user.org))
  return Array.from(orgs).sort()
})

const getRoleColor = (role: string) => {
  switch (role) {
    case 'admin':
      return 'red'
    case 'adminread':
      return 'yellow'
    default:
      return 'blue'
  }
}

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString('de-DE')
}

const clearFilters = () => {
  searchQuery.value = ''
  selectedOrg.value = ''
  selectedRole.value = ''
}

const confirmDelete = (user: User) => {
  userToDelete.value = user
}

const deleteUser = async () => {
  if (!userToDelete.value) return

  try {
    await usersStore.deleteUser(userToDelete.value.id)
    userToDelete.value = null
  } catch (error) {
    console.error('Failed to delete user:', error)
    // Handle error (show toast notification, etc.)
  }
}

const showUserClaims = (user: User) => {
  // Navigate to claims view for this user
  // For now, just log - can be extended later
  console.log('Show claims for user:', user.email)
  // Could navigate to a claims modal or dedicated page
}

// Watch for filter changes and update store filters
watch([searchQuery, selectedOrg, selectedRole], () => {
  usersStore.setFilters({
    search: searchQuery.value || undefined,
    org: selectedOrg.value || undefined,
    role: selectedRole.value || undefined
  })
})

onMounted(() => {
  usersStore.loadUsers()
})
</script>