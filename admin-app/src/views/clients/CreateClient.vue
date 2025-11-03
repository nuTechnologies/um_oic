<template>
  <div>
    <!-- Page header -->
    <div class="page-header">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="page-title">Neuen Client erstellen</h1>
          <p class="page-subtitle">
            OAuth2/OIDC Client-Anwendung registrieren
          </p>
        </div>
        <router-link
          to="/clients"
          class="btn btn-secondary"
        >
          <ArrowLeftIcon class="w-4 h-4 mr-2" />
          Zurück
        </router-link>
      </div>
    </div>

    <!-- Create form -->
    <div class="card">
      <div class="card-header">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          Client-Konfiguration
        </h3>
      </div>
      <form @submit.prevent="createClient" class="card-body space-y-6">
        <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
          <div>
            <label for="client_id" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Client-ID *
            </label>
            <input
              id="client_id"
              type="text"
              v-model="form.client_id"
              required
              class="form-input mt-1"
              placeholder="my-app-client"
            />
          </div>

          <div>
            <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Anwendungsname *
            </label>
            <input
              id="name"
              type="text"
              v-model="form.name"
              required
              class="form-input mt-1"
              placeholder="My Application"
            />
          </div>

          <div class="sm:col-span-2">
            <label for="client_type" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Client-Typ *
            </label>
            <select
              id="client_type"
              v-model="form.client_type"
              required
              class="form-select mt-1"
            >
              <option value="">Client-Typ wählen</option>
              <option value="public">Public (SPA, Mobile Apps)</option>
              <option value="confidential">Confidential (Server-side Apps)</option>
            </select>
          </div>
        </div>

        <!-- Redirect URIs -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Redirect URIs *
          </label>
          <div class="space-y-2">
            <div v-for="(uri, index) in form.redirect_uris" :key="index" class="flex items-center space-x-2">
              <input
                type="url"
                v-model="form.redirect_uris[index]"
                required
                class="form-input flex-1"
                placeholder="https://example.com/auth/callback"
              />
              <button
                type="button"
                @click="removeRedirectUri(index)"
                class="btn btn-sm btn-ghost text-red-600"
              >
                Entfernen
              </button>
            </div>
            <button
              type="button"
              @click="addRedirectUri"
              class="btn btn-sm btn-secondary"
            >
              URI hinzufügen
            </button>
          </div>
        </div>

        <!-- Scopes -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Erlaubte Scopes *
          </label>
          <div class="grid grid-cols-2 gap-2 sm:grid-cols-4">
            <label v-for="scope in availableScopes" :key="scope" class="flex items-center">
              <input
                type="checkbox"
                :value="scope"
                v-model="form.allowed_scopes"
                class="form-checkbox"
              />
              <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">{{ scope }}</span>
            </label>
          </div>
        </div>

        <!-- Grant Types -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Grant Types *
          </label>
          <div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
            <label v-for="grant in availableGrants" :key="grant" class="flex items-center">
              <input
                type="checkbox"
                :value="grant"
                v-model="form.grant_types"
                class="form-checkbox"
              />
              <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">{{ grant }}</span>
            </label>
          </div>
        </div>

        <!-- Additional settings -->
        <div class="space-y-4">
          <div class="flex items-center">
            <input
              id="require_pkce"
              type="checkbox"
              v-model="form.require_pkce"
              class="form-checkbox"
            />
            <label for="require_pkce" class="ml-2 text-sm text-gray-700 dark:text-gray-300">
              PKCE erforderlich (empfohlen für public clients)
            </label>
          </div>
        </div>

        <!-- Actions -->
        <div class="flex justify-end space-x-3">
          <router-link
            to="/clients"
            class="btn btn-secondary"
          >
            Abbrechen
          </router-link>
          <button
            type="submit"
            :disabled="isSubmitting"
            class="btn btn-primary"
          >
            <span v-if="isSubmitting">Wird erstellt...</span>
            <span v-else>Client erstellen</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ArrowLeftIcon } from '@heroicons/vue/24/outline'
import { api } from '@/services/api'

const router = useRouter()
const isSubmitting = ref(false)

const availableScopes = ['openid', 'profile', 'email', 'roles', 'api:read', 'api:write', 'admin']
const availableGrants = ['authorization_code', 'client_credentials', 'refresh_token']

const form = reactive({
  client_id: '',
  name: '',
  client_type: '',
  redirect_uris: [''],
  allowed_scopes: [],
  grant_types: [],
  require_pkce: true
})

const addRedirectUri = () => {
  form.redirect_uris.push('')
}

const removeRedirectUri = (index: number) => {
  if (form.redirect_uris.length > 1) {
    form.redirect_uris.splice(index, 1)
  }
}

const createClient = async () => {
  isSubmitting.value = true
  try {
    const payload = {
      ...form,
      redirect_uris: form.redirect_uris.filter(uri => uri.trim() !== '')
    }

    await api.post('/api/clients', payload)
    router.push('/clients')
  } catch (error) {
    console.error('Failed to create client:', error)
    alert('Fehler beim Erstellen des Clients')
  } finally {
    isSubmitting.value = false
  }
}
</script>