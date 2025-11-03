<template>
  <div>
    <!-- Page header -->
    <div class="page-header">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="page-title">Benutzer bearbeiten</h1>
          <p class="page-subtitle">
            Benutzerinformationen und Claims bearbeiten
          </p>
        </div>
        <router-link
          to="/users"
          class="btn btn-secondary"
        >
          <ArrowLeftIcon class="w-4 h-4 mr-2" />
          Zurück
        </router-link>
      </div>
    </div>

    <div v-if="isLoading" class="card">
      <div class="card-body">
        <div class="text-center py-8">
          <div class="spinner mx-auto"></div>
          <p class="mt-2 text-sm text-gray-500">Benutzer wird geladen...</p>
        </div>
      </div>
    </div>

    <div v-else-if="!user" class="card">
      <div class="card-body">
        <div class="text-center py-8">
          <Ye class="mx-auto h-12 w-12 text-gray-400" />
          <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            Benutzer nicht gefunden
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Der angegebene Benutzer konnte nicht gefunden werden.
          </p>
        </div>
      </div>
    </div>

    <!-- Edit form -->
    <div v-else class="card">
      <div class="card-header">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          Benutzerinformationen
        </h3>
      </div>
      <form @submit.prevent="updateUser" class="card-body space-y-6">
        <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
          <div>
            <label for="first_name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Vorname *
            </label>
            <input
              id="first_name"
              type="text"
              v-model="form.first_name"
              required
              class="form-input mt-1"
            />
          </div>

          <div>
            <label for="last_name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Nachname *
            </label>
            <input
              id="last_name"
              type="text"
              v-model="form.last_name"
              required
              class="form-input mt-1"
            />
          </div>

          <div class="sm:col-span-2">
            <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              E-Mail-Adresse *
            </label>
            <input
              id="email"
              type="email"
              v-model="form.email"
              required
              class="form-input mt-1"
            />
          </div>

          <div>
            <label for="organization" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Organisation *
            </label>
            <select
              id="organization"
              v-model="form.organization"
              required
              class="form-select mt-1"
            >
              <option value="default">Default</option>
              <option value="group-7a">Group 7A</option>
              <option value="group-8b">Group 8B</option>
            </select>
          </div>

          <div>
            <label for="status" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Status
            </label>
            <select
              id="status"
              v-model="form.status"
              class="form-select mt-1"
            >
              <option value="active">Aktiv</option>
              <option value="inactive">Inaktiv</option>
              <option value="suspended">Gesperrt</option>
            </select>
          </div>
        </div>

        <!-- Claims -->
        <div>
          <h4 class="text-base font-medium text-gray-900 dark:text-white mb-4">
            Benutzer-Claims
          </h4>
          <div class="space-y-4">
            <div>
              <label for="roles" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Rollen
              </label>
              <div class="mt-2 grid grid-cols-2 gap-2 sm:grid-cols-4">
                <label v-for="role in availableRoles" :key="role" class="flex items-center">
                  <input
                    type="checkbox"
                    :value="role"
                    v-model="form.claims.roles"
                    class="form-checkbox"
                  />
                  <span class="ml-2 text-sm text-gray-700 dark:text-gray-300 capitalize">{{ role }}</span>
                </label>
              </div>
            </div>

            <div v-if="form.claims.roles.includes('staff')">
              <label for="employee_id" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Mitarbeiter-ID
              </label>
              <input
                id="employee_id"
                type="text"
                v-model="form.claims.employee_id"
                class="form-input mt-1"
                placeholder="EMP-001"
              />
            </div>

            <div v-if="form.claims.roles.includes('staff')">
              <label for="department" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Abteilung
              </label>
              <input
                id="department"
                type="text"
                v-model="form.claims.department"
                class="form-input mt-1"
                placeholder="IT, HR, Finance..."
              />
            </div>

            <div>
              <label for="admin_orgs" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Admin-Berechtigungen
              </label>
              <div class="mt-2 grid grid-cols-2 gap-2 sm:grid-cols-4">
                <label class="flex items-center">
                  <input
                    type="checkbox"
                    value="all"
                    v-model="form.admin"
                    class="form-checkbox"
                  />
                  <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Alle Organisationen</span>
                </label>
                <label class="flex items-center">
                  <input
                    type="checkbox"
                    value="default"
                    v-model="form.admin"
                    :disabled="form.admin.includes('all')"
                    class="form-checkbox"
                  />
                  <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Default</span>
                </label>
                <label class="flex items-center">
                  <input
                    type="checkbox"
                    value="group-7a"
                    v-model="form.admin"
                    :disabled="form.admin.includes('all')"
                    class="form-checkbox"
                  />
                  <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Group 7A</span>
                </label>
                <label class="flex items-center">
                  <input
                    type="checkbox"
                    value="group-8b"
                    v-model="form.admin"
                    :disabled="form.admin.includes('all')"
                    class="form-checkbox"
                  />
                  <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Group 8B</span>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- Password Reset -->
        <div>
          <h4 class="text-base font-medium text-gray-900 dark:text-white mb-4">
            Passwort
          </h4>
          <div class="flex items-center space-x-4">
            <button
              type="button"
              @click="resetPassword"
              :disabled="isResettingPassword"
              class="btn btn-secondary"
            >
              <span v-if="isResettingPassword">Wird zurückgesetzt...</span>
              <span v-else>Passwort zurücksetzen</span>
            </button>
            <span class="text-sm text-gray-500">
              Sendet dem Benutzer eine E-Mail mit einem neuen temporären Passwort
            </span>
          </div>
        </div>

        <!-- Actions -->
        <div class="flex justify-end space-x-3">
          <router-link
            to="/users"
            class="btn btn-secondary"
          >
            Abbrechen
          </router-link>
          <button
            type="submit"
            :disabled="isSubmitting"
            class="btn btn-primary"
          >
            <span v-if="isSubmitting">Wird gespeichert...</span>
            <span v-else>Änderungen speichern</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ArrowLeftIcon, ExclamationTriangleIcon as Ye } from '@heroicons/vue/24/outline'
import { api } from '@/services/api'

const route = useRoute()
const router = useRouter()
const userId = route.params.id as string

const user = ref(null)
const isLoading = ref(false)
const isSubmitting = ref(false)
const isResettingPassword = ref(false)

const availableRoles = ['master', 'editor', 'staff', 'guardian']

const form = reactive({
  first_name: '',
  last_name: '',
  email: '',
  organization: '',
  status: 'active',
  admin: [] as string[],
  claims: {
    roles: [] as string[],
    employee_id: '',
    department: '',
    cohorts: [] as string[]
  }
})

const loadUser = async () => {
  isLoading.value = true
  try {
    const response = await api.get(`/api/users/${userId}`)
    user.value = response.data

    // Populate form
    Object.assign(form, {
      first_name: response.data.first_name,
      last_name: response.data.last_name,
      email: response.data.email,
      organization: response.data.org,
      status: response.data.status || 'active',
      admin: response.data.admin || [],
      claims: {
        roles: response.data.claims?.roles || [],
        employee_id: response.data.claims?.employee_id || '',
        department: response.data.claims?.department || '',
        cohorts: response.data.claims?.cohorts || []
      }
    })
  } catch (error) {
    console.error('Failed to load user:', error)
  } finally {
    isLoading.value = false
  }
}

const updateUser = async () => {
  isSubmitting.value = true
  try {
    const payload = {
      first_name: form.first_name,
      last_name: form.last_name,
      email: form.email,
      org: form.organization,
      status: form.status,
      admin: form.admin,
      claims: {
        ...form.claims,
        ...(form.claims.employee_id && { employee_id: form.claims.employee_id }),
        ...(form.claims.department && { department: form.claims.department })
      }
    }

    await api.patch(`/api/users/${userId}`, payload)
    router.push('/users')
  } catch (error) {
    console.error('Failed to update user:', error)
    alert('Fehler beim Speichern der Änderungen')
  } finally {
    isSubmitting.value = false
  }
}

const resetPassword = async () => {
  if (!confirm('Sind Sie sicher, dass Sie das Passwort zurücksetzen möchten?')) {
    return
  }

  isResettingPassword.value = true
  try {
    await api.post(`/api/users/${userId}/reset-password`)
    alert('Passwort wurde erfolgreich zurückgesetzt. Der Benutzer erhält eine E-Mail mit den neuen Zugangsdaten.')
  } catch (error) {
    console.error('Failed to reset password:', error)
    alert('Fehler beim Zurücksetzen des Passworts')
  } finally {
    isResettingPassword.value = false
  }
}

onMounted(() => {
  loadUser()
})
</script>