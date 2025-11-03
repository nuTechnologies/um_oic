<template>
  <div>
    <!-- Page header -->
    <div class="page-header">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="page-title">OAuth2 Clients</h1>
          <p class="page-subtitle">
            Verwalten Sie OAuth2-Anwendungen und API-Zugriffe
          </p>
        </div>
        <router-link
          to="/clients/create"
          class="btn btn-primary"
        >
          <PlusIcon class="w-4 h-4 mr-2" />
          Client erstellen
        </router-link>
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
              placeholder="Name, Beschreibung..."
            />
          </div>

          <div>
            <label for="grant-type-filter" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Grant Type
            </label>
            <select
              id="grant-type-filter"
              v-model="selectedGrantType"
              class="form-select mt-1"
            >
              <option value="">Alle Grant Types</option>
              <option value="authorization_code">Authorization Code</option>
              <option value="client_credentials">Client Credentials</option>
              <option value="implicit">Implicit</option>
              <option value="password">Password</option>
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

    <!-- Clients table -->
    <div class="card">
      <div class="card-header">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          OAuth2 Clients ({{ filteredClients.length }})
        </h3>
      </div>

      <div class="overflow-hidden">
        <div v-if="clientsStore.isLoading" class="px-6 py-8">
          <div class="space-y-4">
            <div v-for="i in 5" :key="i" class="flex items-center space-x-4">
              <div class="skeleton w-10 h-10 rounded-lg"></div>
              <div class="flex-1 space-y-2">
                <div class="skeleton w-1/3 h-4"></div>
                <div class="skeleton w-1/2 h-3"></div>
              </div>
              <div class="skeleton w-20 h-6"></div>
            </div>
          </div>
        </div>

        <div v-else-if="filteredClients.length === 0" class="px-6 py-8 text-center">
          <KeyIcon class="mx-auto h-12 w-12 text-gray-400" />
          <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            Keine OAuth2-Clients gefunden
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            {{ searchQuery || selectedGrantType
               ? 'Keine Clients entsprechen den Filterkriterien.'
               : 'Erstellen Sie den ersten OAuth2-Client.' }}
          </p>
        </div>

        <table v-else class="table">
          <thead>
            <tr>
              <th>Client</th>
              <th>Grant Types</th>
              <th>Redirect URIs</th>
              <th>Scope</th>
              <th>Erstellt</th>
              <th class="relative">
                <span class="sr-only">Aktionen</span>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="client in filteredClients" :key="client.id">
              <td>
                <div class="flex items-center">
                  <div class="w-10 h-10 bg-indigo-100 dark:bg-indigo-900 rounded-lg flex items-center justify-center mr-3">
                    <KeyIcon class="w-5 h-5 text-indigo-600 dark:text-indigo-300" />
                  </div>
                  <div>
                    <div class="text-sm font-medium text-gray-900 dark:text-white">
                      {{ client.name }}
                    </div>
                    <div v-if="client.description" class="text-sm text-gray-500 dark:text-gray-400">
                      {{ client.description }}
                    </div>
                    <div class="text-xs text-gray-400 font-mono">
                      {{ client.id }}
                    </div>
                  </div>
                </div>
              </td>
              <td>
                <div class="flex flex-wrap gap-1">
                  <Badge
                    v-for="grantType in client.grant_types"
                    :key="grantType"
                    :text="formatGrantType(grantType)"
                    color="blue"
                  />
                </div>
              </td>
              <td class="max-w-xs">
                <div v-if="client.redirect_uris.length > 0" class="space-y-1">
                  <div
                    v-for="(uri, index) in client.redirect_uris.slice(0, 2)"
                    :key="index"
                    class="text-xs font-mono text-gray-600 dark:text-gray-300 truncate"
                  >
                    {{ uri }}
                  </div>
                  <div v-if="client.redirect_uris.length > 2" class="text-xs text-gray-500">
                    +{{ client.redirect_uris.length - 2 }} weitere
                  </div>
                </div>
                <span v-else class="text-sm text-gray-500 dark:text-gray-400">
                  Keine
                </span>
              </td>
              <td>
                <div class="flex flex-wrap gap-1">
                  <Badge
                    v-for="scope in client.scope.slice(0, 3)"
                    :key="scope"
                    :text="scope"
                    color="gray"
                  />
                  <Badge
                    v-if="client.scope.length > 3"
                    :text="`+${client.scope.length - 3}`"
                    color="gray"
                  />
                </div>
              </td>
              <td class="text-sm text-gray-500 dark:text-gray-400">
                {{ formatDate(client.created_at) }}
              </td>
              <td class="text-right text-sm font-medium">
                <div class="flex justify-end space-x-2">
                  <button
                    @click="$router.push(`/clients/${client.id}`)"
                    class="btn btn-sm btn-secondary"
                  >
                    Details
                  </button>
                  <button
                    @click="rotateClientSecret(client)"
                    class="btn btn-sm btn-ghost"
                  >
                    Secret rotieren
                  </button>
                  <button
                    @click="confirmDelete(client)"
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
      v-if="clientToDelete"
      class="modal-overlay"
      @click="clientToDelete = null"
    >
      <div class="modal-panel" @click.stop>
        <div class="px-6 py-4">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">
            Client löschen
          </h3>
          <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
            Sind Sie sicher, dass Sie den OAuth2-Client <strong>{{ clientToDelete.name }}</strong> löschen möchten?
            Diese Aktion kann nicht rückgängig gemacht werden und alle Anwendungen, die diesen Client verwenden,
            werden nicht mehr funktionieren.
          </p>
        </div>
        <div class="px-6 py-4 bg-gray-50 dark:bg-gray-700 flex justify-end space-x-3">
          <button
            @click="clientToDelete = null"
            class="btn btn-secondary"
          >
            Abbrechen
          </button>
          <button
            @click="deleteClient"
            class="btn btn-error"
          >
            Löschen
          </button>
        </div>
      </div>
    </div>

    <!-- Secret rotation modal -->
    <div
      v-if="showSecretModal"
      class="modal-overlay"
      @click="showSecretModal = false"
    >
      <div class="modal-panel" @click.stop>
        <div class="px-6 py-4">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">
            Neues Client Secret
          </h3>
          <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
            Das Client Secret wurde erfolgreich rotiert. Bitte speichern Sie das neue Secret an einem sicheren Ort.
            Es wird nur einmal angezeigt.
          </p>
          <div class="mt-4 p-3 bg-gray-100 dark:bg-gray-800 rounded-lg">
            <div class="flex items-center justify-between">
              <code class="text-sm font-mono">{{ newSecret }}</code>
              <button
                @click="copySecret"
                class="btn btn-sm btn-ghost"
              >
                Kopieren
              </button>
            </div>
          </div>
        </div>
        <div class="px-6 py-4 bg-gray-50 dark:bg-gray-700 flex justify-end">
          <button
            @click="showSecretModal = false"
            class="btn btn-primary"
          >
            Verstanden
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { PlusIcon, KeyIcon } from '@heroicons/vue/24/outline'

import { useClientsStore } from '@/stores/clients'
import type { Client } from '@/stores/clients'
import Badge from '@/components/ui/Badge.vue'

const clientsStore = useClientsStore()

const searchQuery = ref('')
const selectedGrantType = ref('')
const clientToDelete = ref<Client | null>(null)
const showSecretModal = ref(false)
const newSecret = ref('')

const filteredClients = computed(() => {
  let clients = clientsStore.clients

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    clients = clients.filter(client =>
      client.name.toLowerCase().includes(query) ||
      (client.description && client.description.toLowerCase().includes(query))
    )
  }

  if (selectedGrantType.value) {
    clients = clients.filter(client =>
      client.grant_types.includes(selectedGrantType.value)
    )
  }

  return clients
})

const formatGrantType = (grantType: string) => {
  const typeMap: Record<string, string> = {
    'authorization_code': 'Auth Code',
    'client_credentials': 'Client Creds',
    'implicit': 'Implicit',
    'password': 'Password'
  }
  return typeMap[grantType] || grantType
}

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString('de-DE')
}

const clearFilters = () => {
  searchQuery.value = ''
  selectedGrantType.value = ''
}

const confirmDelete = (client: Client) => {
  clientToDelete.value = client
}

const deleteClient = async () => {
  if (!clientToDelete.value) return

  try {
    await clientsStore.deleteClient(clientToDelete.value.id)
    clientToDelete.value = null
  } catch (error) {
    console.error('Failed to delete client:', error)
  }
}

const rotateClientSecret = async (client: Client) => {
  try {
    const secret = await clientsStore.rotateSecret(client.id)
    newSecret.value = secret
    showSecretModal.value = true
  } catch (error) {
    console.error('Failed to rotate client secret:', error)
  }
}

const copySecret = async () => {
  try {
    await navigator.clipboard.writeText(newSecret.value)
  } catch (error) {
    console.error('Failed to copy secret:', error)
  }
}

watch([searchQuery, selectedGrantType], () => {
  clientsStore.setFilters({
    search: searchQuery.value || undefined,
    grant_type: selectedGrantType.value || undefined
  })
})

onMounted(() => {
  clientsStore.loadClients()
})
</script>