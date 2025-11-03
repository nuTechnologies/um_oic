<template>
  <div>
    <!-- Page header -->
    <div class="page-header">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="page-title">Organisationen</h1>
          <p class="page-subtitle">
            Verwalten Sie Organisationen und deren Einstellungen
          </p>
        </div>
        <button
          @click="showCreateModal = true"
          class="btn btn-primary"
        >
          <PlusIcon class="w-4 h-4 mr-2" />
          Organisation erstellen
        </button>
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
              placeholder="Name, Beschreibung, Domain..."
            />
          </div>

          <div>
            <label for="domain-filter" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Domain-Status
            </label>
            <select
              id="domain-filter"
              v-model="domainFilter"
              class="form-select mt-1"
            >
              <option value="">Alle</option>
              <option value="true">Mit Domain</option>
              <option value="false">Ohne Domain</option>
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

    <!-- Organizations table -->
    <div class="card">
      <div class="card-header">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          Organisationen ({{ filteredOrganizations.length }})
        </h3>
      </div>

      <div class="overflow-hidden">
        <div v-if="organizationsStore.isLoading" class="px-6 py-8">
          <div class="space-y-4">
            <div v-for="i in 5" :key="i" class="flex items-center space-x-4">
              <div class="skeleton w-12 h-12 rounded-lg"></div>
              <div class="flex-1 space-y-2">
                <div class="skeleton w-1/3 h-4"></div>
                <div class="skeleton w-1/2 h-3"></div>
              </div>
              <div class="skeleton w-20 h-6"></div>
            </div>
          </div>
        </div>

        <div v-else-if="filteredOrganizations.length === 0" class="px-6 py-8 text-center">
          <BuildingOfficeIcon class="mx-auto h-12 w-12 text-gray-400" />
          <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            Keine Organisationen gefunden
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            {{ searchQuery || domainFilter
               ? 'Keine Organisationen entsprechen den Filterkriterien.'
               : 'Erstellen Sie die erste Organisation.' }}
          </p>
        </div>

        <table v-else class="table">
          <thead>
            <tr>
              <th>Organisation</th>
              <th>Domain</th>
              <th>Benutzer</th>
              <th>Features</th>
              <th>Erstellt</th>
              <th class="relative">
                <span class="sr-only">Aktionen</span>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="org in filteredOrganizations" :key="org.id">
              <td>
                <div class="flex items-center">
                  <div class="w-12 h-12 bg-blue-100 dark:bg-blue-900 rounded-lg flex items-center justify-center mr-3">
                    <BuildingOfficeIcon class="w-6 h-6 text-blue-600 dark:text-blue-300" />
                  </div>
                  <div>
                    <div class="text-sm font-medium text-gray-900 dark:text-white">
                      {{ org.name }}
                    </div>
                    <div v-if="org.description" class="text-sm text-gray-500 dark:text-gray-400">
                      {{ org.description }}
                    </div>
                    <div class="text-xs text-gray-400 font-mono">
                      {{ org.id }}
                    </div>
                  </div>
                </div>
              </td>
              <td>
                <div v-if="org.domain" class="text-sm font-mono text-gray-900 dark:text-white">
                  {{ org.domain }}
                </div>
                <span v-else class="text-sm text-gray-500 dark:text-gray-400">
                  Keine Domain
                </span>
              </td>
              <td>
                <div class="text-sm text-gray-900 dark:text-white">
                  {{ org.user_count || 0 }} Benutzer
                  <div v-if="org.admin_count" class="text-xs text-gray-500">
                    {{ org.admin_count }} Admins
                  </div>
                  <div v-if="org.settings?.max_users" class="text-xs text-gray-400">
                    Max: {{ org.settings.max_users }}
                  </div>
                </div>
              </td>
              <td>
                <div v-if="org.settings?.features?.length" class="flex flex-wrap gap-1">
                  <Badge
                    v-for="feature in org.settings.features.slice(0, 2)"
                    :key="feature"
                    :text="feature"
                    color="green"
                  />
                  <Badge
                    v-if="org.settings.features.length > 2"
                    :text="`+${org.settings.features.length - 2}`"
                    color="gray"
                  />
                </div>
                <span v-else class="text-sm text-gray-500 dark:text-gray-400">
                  Standard
                </span>
              </td>
              <td class="text-sm text-gray-500 dark:text-gray-400">
                {{ formatDate(org.created_at) }}
              </td>
              <td class="text-right text-sm font-medium">
                <div class="flex justify-end space-x-2">
                  <button
                    @click="$router.push(`/organizations/${org.id}`)"
                    class="btn btn-sm btn-secondary"
                  >
                    Details
                  </button>
                  <button
                    @click="editOrganization(org)"
                    class="btn btn-sm btn-ghost"
                  >
                    Bearbeiten
                  </button>
                  <button
                    @click="confirmDelete(org)"
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

    <!-- Create organization modal -->
    <div
      v-if="showCreateModal"
      class="modal-overlay"
      @click="showCreateModal = false"
    >
      <div class="modal-panel max-w-2xl" @click.stop>
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">
            Organisation erstellen
          </h3>
        </div>
        <form @submit.prevent="createOrganization" class="px-6 py-4 space-y-4">
          <div>
            <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Name *
            </label>
            <input
              id="name"
              v-model="createForm.name"
              type="text"
              required
              class="form-input mt-1"
              placeholder="z.B. Acme Corporation"
            />
          </div>

          <div>
            <label for="description" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Beschreibung
            </label>
            <textarea
              id="description"
              v-model="createForm.description"
              rows="3"
              class="form-textarea mt-1"
              placeholder="Optionale Beschreibung der Organisation"
            ></textarea>
          </div>

          <div>
            <label for="domain" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Domain
            </label>
            <input
              id="domain"
              v-model="createForm.domain"
              type="text"
              class="form-input mt-1"
              placeholder="z.B. acme.com"
            />
          </div>

          <div>
            <label for="max-users" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Maximale Benutzeranzahl
            </label>
            <input
              id="max-users"
              v-model="createForm.max_users"
              type="number"
              min="1"
              class="form-input mt-1"
              placeholder="z.B. 100"
            />
          </div>
        </form>
        <div class="px-6 py-4 bg-gray-50 dark:bg-gray-700 flex justify-end space-x-3">
          <button
            @click="showCreateModal = false"
            class="btn btn-secondary"
          >
            Abbrechen
          </button>
          <button
            @click="createOrganization"
            :disabled="!createForm.name"
            class="btn btn-primary"
          >
            Erstellen
          </button>
        </div>
      </div>
    </div>

    <!-- Delete confirmation modal -->
    <div
      v-if="orgToDelete"
      class="modal-overlay"
      @click="orgToDelete = null"
    >
      <div class="modal-panel" @click.stop>
        <div class="px-6 py-4">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">
            Organisation löschen
          </h3>
          <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
            Sind Sie sicher, dass Sie die Organisation <strong>{{ orgToDelete.name }}</strong> löschen möchten?
            Alle Benutzer und Daten dieser Organisation werden ebenfalls gelöscht.
            Diese Aktion kann nicht rückgängig gemacht werden.
          </p>
        </div>
        <div class="px-6 py-4 bg-gray-50 dark:bg-gray-700 flex justify-end space-x-3">
          <button
            @click="orgToDelete = null"
            class="btn btn-secondary"
          >
            Abbrechen
          </button>
          <button
            @click="deleteOrganization"
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
import { useRouter } from 'vue-router'
import { PlusIcon, BuildingOfficeIcon } from '@heroicons/vue/24/outline'

import { useOrganizationsStore } from '@/stores/organizations'
import type { Organization } from '@/stores/organizations'
import Badge from '@/components/ui/Badge.vue'

const organizationsStore = useOrganizationsStore()
const router = useRouter()

const searchQuery = ref('')
const domainFilter = ref('')
const showCreateModal = ref(false)
const orgToDelete = ref<Organization | null>(null)

const createForm = ref({
  name: '',
  description: '',
  domain: '',
  max_users: null as number | null
})

const filteredOrganizations = computed(() => {
  let orgs = organizationsStore.organizations

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    orgs = orgs.filter(org =>
      org.name.toLowerCase().includes(query) ||
      (org.description && org.description.toLowerCase().includes(query)) ||
      (org.domain && org.domain.toLowerCase().includes(query))
    )
  }

  if (domainFilter.value) {
    const hasDomain = domainFilter.value === 'true'
    orgs = orgs.filter(org => hasDomain ? !!org.domain : !org.domain)
  }

  return orgs
})

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString('de-DE')
}

const clearFilters = () => {
  searchQuery.value = ''
  domainFilter.value = ''
}

const createOrganization = async () => {
  if (!createForm.value.name) return

  try {
    const orgData = {
      name: createForm.value.name,
      description: createForm.value.description || undefined,
      domain: createForm.value.domain || undefined,
      settings: createForm.value.max_users ? {
        max_users: createForm.value.max_users
      } : undefined
    }

    await organizationsStore.createOrganization(orgData)

    // Reset form and close modal
    createForm.value = {
      name: '',
      description: '',
      domain: '',
      max_users: null
    }
    showCreateModal.value = false
  } catch (error) {
    console.error('Failed to create organization:', error)
  }
}

const editOrganization = (org: Organization) => {
  // Navigate to detail page or open edit modal
  // For now, navigate to detail page
  router.push(`/organizations/${org.id}`)
}

const confirmDelete = (org: Organization) => {
  orgToDelete.value = org
}

const deleteOrganization = async () => {
  if (!orgToDelete.value) return

  try {
    await organizationsStore.deleteOrganization(orgToDelete.value.id)
    orgToDelete.value = null
  } catch (error) {
    console.error('Failed to delete organization:', error)
  }
}

watch([searchQuery, domainFilter], () => {
  organizationsStore.setFilters({
    search: searchQuery.value || undefined,
    has_domain: domainFilter.value ? domainFilter.value === 'true' : undefined
  })
})

onMounted(() => {
  organizationsStore.loadOrganizations()
})
</script>