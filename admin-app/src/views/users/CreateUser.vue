<template>
  <div>
    <!-- Page header -->
    <div class="page-header">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="page-title">Neuen Benutzer erstellen</h1>
          <p class="page-subtitle">
            Fügen Sie einen neuen Benutzer zum System hinzu
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

    <!-- Create form -->
    <div class="card">
      <div class="card-header">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          Benutzerinformationen
        </h3>
      </div>
      <form @submit.prevent="createUser" class="card-body space-y-6">
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
              placeholder="Max"
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
              placeholder="Mustermann"
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
              placeholder="max.mustermann@example.com"
            />
          </div>

          <div>
            <label for="org" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Organisation *
            </label>
            <select
              id="org"
              v-model="form.org"
              required
              class="form-select mt-1"
            >
              <option value="">Organisation wählen</option>
              <option v-for="org in availableOrganizations" :key="org.id" :value="org.id">
                {{ org.name }}
              </option>
              <!-- Fallback options if no organizations loaded -->
              <option v-if="availableOrganizations.length === 0" value="default">Default</option>
            </select>
          </div>

          <div>
            <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Passwort *
            </label>
            <input
              id="password"
              type="password"
              v-model="form.password"
              required
              minlength="8"
              class="form-input mt-1"
              placeholder="Mindestens 8 Zeichen"
            />
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
            <span v-if="isSubmitting">Wird erstellt...</span>
            <span v-else>Benutzer erstellen</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ArrowLeftIcon } from '@heroicons/vue/24/outline'
import { api } from '@/services/api'

const router = useRouter()
const isSubmitting = ref(false)
const availableOrganizations = ref([])

const availableRoles = ['master', 'editor', 'staff', 'guardian']

const form = reactive({
  first_name: '',
  last_name: '',
  email: '',
  org: '',
  password: '',
  claims: {
    roles: [] as string[],
    employee_id: '',
    department: '',
    cohorts: [] as string[]
  }
})

const loadOrganizations = async () => {
  try {
    const response = await api.get('/api/organizations')
    availableOrganizations.value = response.data
  } catch (error) {
    console.error('Failed to load organizations:', error)
  }
}

const createUser = async () => {
  isSubmitting.value = true
  try {
    const payload = {
      ...form,
      claims: {
        ...form.claims,
        ...(form.claims.employee_id && { employee_id: form.claims.employee_id }),
        ...(form.claims.department && { department: form.claims.department })
      }
    }

    await api.post('/api/users', payload)
    router.push('/users')
  } catch (error) {
    console.error('Failed to create user:', error)
    alert('Fehler beim Erstellen des Benutzers')
  } finally {
    isSubmitting.value = false
  }
}

onMounted(() => {
  loadOrganizations()
})
</script>