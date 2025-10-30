# 🎨 Admin-App Architecture

## 📋 Overview

Die Admin-App ist eine moderne Vue.js 3 Single-Page-Application (SPA) für die Verwaltung des UM-OIC Authentication Systems. Sie bietet eine intuitive Benutzeroberfläche für Administratoren zur Verwaltung von Benutzern, Organisationen, Gruppen und OAuth2-Clients.

## 🏗️ Technical Stack

```typescript
// Core Framework
Vue 3 (Composition API)
TypeScript
Vite (Build Tool)

// State Management
Pinia (Global State)
VueUse (Utilities)

// UI Components
Tailwind CSS (Styling)
Headless UI (Accessible Components)
Heroicons (Icon System)

// HTTP & Routing
Axios (API Client)
Vue Router 4
Zod (Runtime Validation)

// Development
ESLint + Prettier
Vue TypeScript Support
```

## 📁 Project Structure

```
admin-app/
├── src/
│   ├── main.ts                 # App Entry Point
│   ├── App.vue                 # Root Component
│   ├── router/
│   │   ├── index.ts            # Router Configuration
│   │   └── guards.ts           # Navigation Guards
│   ├── stores/
│   │   ├── auth.ts             # Authentication State
│   │   ├── users.ts            # User Management
│   │   ├── organizations.ts    # Organization Management
│   │   ├── groups.ts           # Group Management
│   │   ├── clients.ts          # OAuth2 Client Management
│   │   └── system.ts           # System Status & Settings
│   ├── views/
│   │   ├── Dashboard.vue       # Main Dashboard
│   │   ├── users/
│   │   │   ├── UserList.vue    # User Overview
│   │   │   ├── UserDetail.vue  # User Details/Edit
│   │   │   └── UserCreate.vue  # Create New User
│   │   ├── organizations/
│   │   │   ├── OrgList.vue     # Organization Overview
│   │   │   └── OrgDetail.vue   # Organization Details
│   │   ├── groups/
│   │   │   ├── GroupList.vue   # Group Management
│   │   │   └── GroupDetail.vue # Group Details
│   │   ├── clients/
│   │   │   ├── ClientList.vue  # OAuth2 Clients
│   │   │   └── ClientDetail.vue# Client Configuration
│   │   ├── audit/
│   │   │   └── AuditLog.vue    # Audit Trail
│   │   └── system/
│   │       ├── Status.vue      # System Status
│   │       └── Settings.vue    # System Settings
│   ├── components/
│   │   ├── layout/
│   │   │   ├── AppLayout.vue   # Main Layout
│   │   │   ├── Sidebar.vue     # Navigation Sidebar
│   │   │   ├── Header.vue      # Top Header
│   │   │   └── Breadcrumb.vue  # Breadcrumb Navigation
│   │   ├── ui/
│   │   │   ├── DataTable.vue   # Reusable Data Table
│   │   │   ├── SearchInput.vue # Search Component
│   │   │   ├── StatusBadge.vue # Status Indicators
│   │   │   ├── LoadingSpinner.vue
│   │   │   ├── Modal.vue       # Modal Dialog
│   │   │   └── FormField.vue   # Form Input Components
│   │   ├── forms/
│   │   │   ├── UserForm.vue    # User Create/Edit Form
│   │   │   ├── GroupForm.vue   # Group Form
│   │   │   ├── ClientForm.vue  # OAuth2 Client Form
│   │   │   └── ClaimsEditor.vue# Claims Editor
│   │   └── charts/
│   │       ├── UserStats.vue   # User Statistics
│   │       └── ActivityChart.vue# Activity Visualization
│   ├── composables/
│   │   ├── useApi.ts           # API Client Composable
│   │   ├── useAuth.ts          # Authentication Logic
│   │   ├── usePagination.ts    # Pagination Logic
│   │   ├── useSearch.ts        # Search Functionality
│   │   └── useNotifications.ts # Toast Notifications
│   ├── types/
│   │   ├── api.ts              # API Response Types
│   │   ├── user.ts             # User-related Types
│   │   ├── organization.ts     # Organization Types
│   │   ├── group.ts            # Group Types
│   │   ├── client.ts           # OAuth2 Client Types
│   │   └── index.ts            # Type Exports
│   ├── utils/
│   │   ├── api.ts              # API Configuration
│   │   ├── auth.ts             # Auth Token Handling
│   │   ├── validation.ts       # Zod Schemas
│   │   ├── date.ts             # Date Utilities
│   │   └── format.ts           # Data Formatting
│   └── style.css               # Global Styles
├── public/
│   ├── favicon.ico
│   └── logo.svg
├── package.json
├── vite.config.ts
├── tailwind.config.js
├── tsconfig.json
└── .eslintrc.js
```

## 🔄 API Integration Analysis

### Current Admin-Service API Review

**✅ Vollständige Endpoints:**
```typescript
// Users Management
GET    /api/users              // ✅ List with query params
POST   /api/users              // ✅ Create user
GET    /api/users/:id          // ✅ Get user details
PATCH  /api/users/:id          // ✅ Update user
DELETE /api/users/:id          // ✅ Delete user
POST   /api/users/:id/reset-password // ✅ Password reset

// Organizations
GET    /api/organizations      // ✅ List organizations
GET    /api/organizations/:org/users // ✅ Org users

// Groups
GET    /api/groups             // ✅ List groups
POST   /api/groups             // ✅ Create group
GET    /api/groups/:id         // ✅ Get group
PATCH  /api/groups/:id         // ✅ Update group
DELETE /api/groups/:id         // ✅ Delete group

// OAuth2 Clients
GET    /api/clients            // ✅ List clients
POST   /api/clients            // ✅ Create client
GET    /api/clients/:id        // ✅ Get client
PATCH  /api/clients/:id        // ✅ Update client
DELETE /api/clients/:id        // ✅ Delete client
POST   /api/clients/:id/rotate-secret // ✅ Rotate secret

// System & Audit
GET    /api/system/status      // ✅ System status
POST   /api/system/reload-auth // ✅ Trigger reload
GET    /api/audit              // ✅ Query audit logs
```

**❌ Fehlende Endpoints (Ergänzungen erforderlich):**

```typescript
// Erweiterte User-Operationen
GET    /api/users/:id/sessions     // Aktive Sessions
DELETE /api/users/:id/sessions     // Logout User
POST   /api/users/:id/verify       // Verify Email
POST   /api/users/:id/mfa/enable   // Enable MFA
POST   /api/users/:id/mfa/disable  // Disable MFA

// Claims Management
GET    /api/claims/registry         // Claims Registry
PATCH  /api/claims/registry         // Update Registry
GET    /api/users/:id/claims        // User Claims
PATCH  /api/users/:id/claims        // Update User Claims

// Bulk Operations
POST   /api/users/bulk-create       // Bulk User Creation
POST   /api/users/bulk-update       // Bulk User Update
POST   /api/users/bulk-delete       // Bulk User Deletion
POST   /api/users/import            // CSV Import

// Statistics & Analytics
GET    /api/stats/users             // User Statistics
GET    /api/stats/organizations     // Org Statistics
GET    /api/stats/logins            // Login Analytics
GET    /api/stats/audit             // Audit Statistics

// Advanced Search
POST   /api/search/users            // Advanced User Search
POST   /api/search/audit            // Advanced Audit Search

// Configuration
GET    /api/config                  // System Configuration
PATCH  /api/config                  // Update Configuration
```

## 🎨 UI/UX Design Patterns

### 1. Layout Architecture

```vue
<!-- App.vue -->
<template>
  <div id="app">
    <AppLayout v-if="isAuthenticated">
      <router-view />
    </AppLayout>
    <LoginView v-else />
  </div>
</template>
```

```vue
<!-- AppLayout.vue -->
<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Sidebar -->
    <Sidebar :collapsed="sidebarCollapsed" />

    <!-- Main Content -->
    <div class="lg:pl-64">
      <!-- Header -->
      <Header @toggle-sidebar="sidebarCollapsed = !sidebarCollapsed" />

      <!-- Breadcrumb -->
      <Breadcrumb />

      <!-- Page Content -->
      <main class="py-6">
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <slot />
        </div>
      </main>
    </div>
  </div>
</template>
```

### 2. State Management Pattern

```typescript
// stores/users.ts
export const useUsersStore = defineStore('users', () => {
  const users = ref<User[]>([])
  const currentUser = ref<User | null>(null)
  const loading = ref(false)
  const pagination = ref({
    page: 1,
    limit: 20,
    total: 0
  })

  const fetchUsers = async (params?: UserSearchParams) => {
    loading.value = true
    try {
      const response = await api.get('/api/users', { params })
      users.value = response.data
      pagination.value.total = response.headers['x-total-count']
    } finally {
      loading.value = false
    }
  }

  const createUser = async (userData: CreateUserRequest) => {
    const response = await api.post('/api/users', userData)
    users.value.push(response.data)
    return response.data
  }

  const updateUser = async (id: string, userData: UpdateUserRequest) => {
    const response = await api.patch(`/api/users/${id}`, userData)
    const index = users.value.findIndex(u => u.id === id)
    if (index !== -1) {
      users.value[index] = response.data
    }
    return response.data
  }

  return {
    users: readonly(users),
    currentUser: readonly(currentUser),
    loading: readonly(loading),
    pagination: readonly(pagination),
    fetchUsers,
    createUser,
    updateUser
  }
})
```

### 3. Component Composition

```vue
<!-- UserList.vue -->
<template>
  <div>
    <!-- Header -->
    <div class="sm:flex sm:items-center">
      <div class="sm:flex-auto">
        <h1 class="text-2xl font-semibold text-gray-900">Users</h1>
        <p class="mt-2 text-sm text-gray-700">
          Manage user accounts and permissions
        </p>
      </div>
      <div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
        <button @click="showCreateModal = true"
                class="btn btn-primary">
          Add User
        </button>
      </div>
    </div>

    <!-- Filters -->
    <div class="mt-6">
      <UserFilters v-model="filters" @search="handleSearch" />
    </div>

    <!-- Data Table -->
    <div class="mt-6">
      <DataTable
        :data="users"
        :columns="userColumns"
        :loading="loading"
        :pagination="pagination"
        @page-change="handlePageChange"
        @sort="handleSort"
      />
    </div>

    <!-- Create Modal -->
    <Modal v-model="showCreateModal" title="Create User">
      <UserForm @submit="handleCreateUser" @cancel="showCreateModal = false" />
    </Modal>
  </div>
</template>

<script setup lang="ts">
const usersStore = useUsersStore()
const { users, loading, pagination } = storeToRefs(usersStore)

const filters = ref<UserFilters>({})
const showCreateModal = ref(false)

const userColumns = [
  { key: 'email', label: 'Email', sortable: true },
  { key: 'full_name', label: 'Name', sortable: true },
  { key: 'org', label: 'Organization', sortable: true },
  { key: 'status', label: 'Status', component: StatusBadge },
  { key: 'created_at', label: 'Created', formatter: formatDate },
  { key: 'actions', label: 'Actions', component: UserActions }
]

onMounted(() => {
  usersStore.fetchUsers()
})
</script>
```

## 🔐 Authentication & Authorization

### JWT Token Handling

```typescript
// composables/useAuth.ts
export const useAuth = () => {
  const token = ref(localStorage.getItem('auth_token'))
  const user = ref<User | null>(null)

  const login = async (credentials: LoginCredentials) => {
    // Login über auth-service
    const response = await authApi.post('/api/auth/login', credentials)

    if (response.data.success && response.data.access_token) {
      token.value = response.data.access_token
      localStorage.setItem('auth_token', token.value)
      await fetchCurrentUser()
      return true
    }
    return false
  }

  const logout = () => {
    token.value = null
    user.value = null
    localStorage.removeItem('auth_token')
    router.push('/login')
  }

  const isAdmin = computed(() => {
    return user.value?.admin?.length > 0
  })

  const canManageOrg = (org: string) => {
    return user.value?.admin?.includes('all') ||
           user.value?.admin?.includes(org)
  }

  return {
    token: readonly(token),
    user: readonly(user),
    isAdmin,
    canManageOrg,
    login,
    logout
  }
}
```

### API Client Configuration

```typescript
// utils/api.ts
const apiClient = axios.create({
  baseURL: '/api',
  timeout: 10000,
})

// Request Interceptor - Add Auth Token
apiClient.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('auth_token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => Promise.reject(error)
)

// Response Interceptor - Handle Auth Errors
apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Token expired or invalid
      localStorage.removeItem('auth_token')
      router.push('/login')
    }
    return Promise.reject(error)
  }
)
```

## 📊 Data Management Patterns

### Real-time Updates

```typescript
// composables/useRealtime.ts
export const useRealtime = () => {
  const systemStore = useSystemStore()

  const pollSystemStatus = () => {
    setInterval(async () => {
      await systemStore.fetchSystemStatus()
    }, 30000) // Poll every 30 seconds
  }

  const refreshData = async () => {
    // Trigger data reload across stores
    await Promise.all([
      useUsersStore().fetchUsers(),
      useOrganizationsStore().fetchOrganizations(),
      useSystemStore().fetchSystemStatus()
    ])
  }

  return {
    pollSystemStatus,
    refreshData
  }
}
```

### Error Handling

```typescript
// composables/useNotifications.ts
export const useNotifications = () => {
  const notifications = ref<Notification[]>([])

  const addNotification = (notification: Omit<Notification, 'id'>) => {
    const id = Date.now().toString()
    notifications.value.push({ ...notification, id })

    if (notification.type !== 'error') {
      setTimeout(() => {
        removeNotification(id)
      }, 5000)
    }
  }

  const handleApiError = (error: any) => {
    const message = error.response?.data?.message || 'An error occurred'
    addNotification({
      type: 'error',
      title: 'Error',
      message
    })
  }

  return {
    notifications: readonly(notifications),
    addNotification,
    handleApiError
  }
}
```

## 🚀 Build & Deployment

### Development Workflow

```bash
# Development Server
cd admin-app
npm run dev

# Type Checking
npm run type-check

# Linting
npm run lint

# Production Build
npm run build  # Outputs to ../data/web/mgmt/
```

### Environment Configuration

```typescript
// vite.config.ts - Environment-specific builds
export default defineConfig(({ mode }) => ({
  define: {
    __APP_VERSION__: JSON.stringify(process.env.npm_package_version),
    __API_BASE_URL__: mode === 'development'
      ? JSON.stringify('http://localhost:8001')
      : JSON.stringify('/api')
  },
  // ...
}))
```

## 🎯 Performance Optimizations

### Code Splitting

```typescript
// router/index.ts - Route-based splitting
const routes = [
  {
    path: '/users',
    component: () => import('../views/users/UserList.vue')
  },
  {
    path: '/organizations',
    component: () => import('../views/organizations/OrgList.vue')
  }
]
```

### Lazy Loading

```vue
<!-- Lazy load heavy components -->
<template>
  <Suspense>
    <UserChart v-if="showChart" />
    <template #fallback>
      <LoadingSpinner />
    </template>
  </Suspense>
</template>

<script setup lang="ts">
const UserChart = defineAsyncComponent(
  () => import('../components/charts/UserChart.vue')
)
</script>
```

## 📋 Required API Extensions

### Missing Endpoints für vollständige Admin-App

1. **Claims Registry Management:**
```typescript
GET    /api/claims/registry         // Get claims registry
PATCH  /api/claims/registry         // Update claims registry
```

2. **User Session Management:**
```typescript
GET    /api/users/:id/sessions      // Get active sessions
DELETE /api/users/:id/sessions      // Force logout
```

3. **Bulk Operations:**
```typescript
POST   /api/users/bulk              // Bulk user operations
POST   /api/users/import            // CSV import
```

4. **Enhanced Statistics:**
```typescript
GET    /api/stats/users             // User statistics
GET    /api/stats/organizations     // Organization statistics
GET    /api/stats/activity          // Activity metrics
```

Diese Architektur bietet eine moderne, skalierbare und wartbare Admin-Oberfläche für das UM-OIC System mit vollständiger TypeScript-Unterstützung und best practices für Vue.js 3 Entwicklung.