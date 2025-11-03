<template>
  <div>
    <!-- Page header -->
    <div class="page-header">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="page-title">Claims Registry Editor</h1>
          <p class="page-subtitle">
            Bearbeiten Sie die verfügbaren Claims und deren Definitionen
          </p>
        </div>
        <div class="flex space-x-3">
          <router-link
            to="/claims"
            class="btn btn-secondary"
          >
            <ArrowLeftIcon class="w-4 h-4 mr-2" />
            Zurück
          </router-link>
          <button
            @click="saveRegistry"
            :disabled="isSaving"
            class="btn btn-primary"
          >
            <span v-if="isSaving">Wird gespeichert...</span>
            <span v-else>Registry speichern</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Registry Editor -->
    <div class="card">
      <div class="card-header">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          Claims Definitionen
        </h3>
      </div>
      <div class="card-body">
        <div v-if="isLoading" class="text-center py-8">
          <div class="spinner mx-auto"></div>
          <p class="mt-2 text-sm text-gray-500">Registry wird geladen...</p>
        </div>

        <div v-else class="space-y-6">
          <!-- Add new claim -->
          <div class="border border-dashed border-gray-300 dark:border-gray-700 rounded-lg p-4">
            <h4 class="text-sm font-medium text-gray-900 dark:text-white mb-4">
              Neuen Claim hinzufügen
            </h4>
            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
              <input
                v-model="newClaim.key"
                type="text"
                placeholder="Claim-Schlüssel"
                class="form-input"
              />
              <select v-model="newClaim.type" class="form-select">
                <option value="">Typ wählen</option>
                <option value="string">String</option>
                <option value="number">Number</option>
                <option value="boolean">Boolean</option>
                <option value="array">Array</option>
                <option value="object">Object</option>
              </select>
              <input
                v-model="newClaim.description"
                type="text"
                placeholder="Beschreibung"
                class="form-input"
              />
              <button
                @click="addClaim"
                :disabled="!newClaim.key || !newClaim.type"
                class="btn btn-primary"
              >
                Hinzufügen
              </button>
            </div>
          </div>

          <!-- Existing claims -->
          <div class="space-y-4">
            <div
              v-for="(claim, key) in registryData.claims"
              :key="key"
              class="border border-gray-200 dark:border-gray-700 rounded-lg p-4"
            >
              <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
                <div class="space-y-4">
                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                      Claim-Schlüssel
                    </label>
                    <input
                      :value="key"
                      disabled
                      class="form-input mt-1 bg-gray-50 dark:bg-gray-800"
                    />
                  </div>

                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                      Typ
                    </label>
                    <select v-model="claim.type" class="form-select mt-1">
                      <option value="string">String</option>
                      <option value="number">Number</option>
                      <option value="boolean">Boolean</option>
                      <option value="array">Array</option>
                      <option value="object">Object</option>
                    </select>
                  </div>

                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                      Beschreibung
                    </label>
                    <textarea
                      v-model="claim.description"
                      rows="2"
                      class="form-textarea mt-1"
                    ></textarea>
                  </div>
                </div>

                <div class="space-y-4">
                  <div class="space-y-2">
                    <label class="flex items-center">
                      <input
                        type="checkbox"
                        v-model="claim.required"
                        class="form-checkbox"
                      />
                      <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Required</span>
                    </label>

                    <label class="flex items-center">
                      <input
                        type="checkbox"
                        v-model="claim.default_allowed"
                        class="form-checkbox"
                      />
                      <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Default Allowed</span>
                    </label>

                    <label class="flex items-center">
                      <input
                        type="checkbox"
                        v-model="claim.sensitive"
                        class="form-checkbox"
                      />
                      <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Sensitive</span>
                    </label>

                    <label class="flex items-center">
                      <input
                        type="checkbox"
                        v-model="claim.admin_only"
                        class="form-checkbox"
                      />
                      <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Admin Only</span>
                    </label>
                  </div>

                  <div v-if="claim.type === 'array' && claim.items">
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                      Array Items (JSON)
                    </label>
                    <textarea
                      :value="JSON.stringify(claim.items, null, 2)"
                      @input="updateClaimItems(key, $event)"
                      rows="3"
                      class="form-textarea mt-1 font-mono text-xs"
                    ></textarea>
                  </div>

                  <div class="flex justify-end">
                    <button
                      @click="removeClaim(key)"
                      class="btn btn-sm btn-error"
                    >
                      Claim entfernen
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ArrowLeftIcon } from '@heroicons/vue/24/outline'
import { useClaimsStore } from '@/stores/claims'

const claimsStore = useClaimsStore()
const isLoading = ref(false)
const isSaving = ref(false)

const registryData = reactive({
  version: "1.0",
  claims: {} as Record<string, any>,
  last_updated: ""
})

const newClaim = reactive({
  key: '',
  type: '',
  description: ''
})

const loadRegistry = async () => {
  isLoading.value = true
  try {
    await claimsStore.loadClaimsRegistry()
    if (claimsStore.registry) {
      Object.assign(registryData, claimsStore.registry)
    }
  } catch (error) {
    console.error('Failed to load registry:', error)
  } finally {
    isLoading.value = false
  }
}

const addClaim = () => {
  if (!newClaim.key || !newClaim.type) return

  registryData.claims[newClaim.key] = {
    type: newClaim.type,
    description: newClaim.description,
    default_allowed: true,
    required: false,
    sensitive: false,
    admin_only: false,
    ...(newClaim.type === 'array' && { items: { type: 'string' } })
  }

  // Reset form
  newClaim.key = ''
  newClaim.type = ''
  newClaim.description = ''
}

const removeClaim = (key: string) => {
  if (confirm(`Sind Sie sicher, dass Sie den Claim "${key}" entfernen möchten?`)) {
    delete registryData.claims[key]
  }
}

const updateClaimItems = (key: string, event: Event) => {
  const target = event.target as HTMLTextAreaElement
  try {
    registryData.claims[key].items = JSON.parse(target.value)
  } catch (error) {
    console.warn('Invalid JSON for claim items:', error)
  }
}

const saveRegistry = async () => {
  isSaving.value = true
  try {
    const payload = {
      ...registryData,
      last_updated: new Date().toISOString()
    }

    await claimsStore.updateClaimsRegistry(payload)
    alert('Registry erfolgreich gespeichert!')
  } catch (error) {
    console.error('Failed to save registry:', error)
    alert('Fehler beim Speichern der Registry')
  } finally {
    isSaving.value = false
  }
}

onMounted(() => {
  loadRegistry()
})
</script>