# 🎨 Admin-App UI Design & Views

## 📱 Layout Architecture

### Main Layout Structure

```vue
<!-- AppLayout.vue -->
<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <!-- Fixed Sidebar -->
    <Sidebar
      :collapsed="sidebarCollapsed"
      :mobile-open="mobileMenuOpen"
      @close="mobileMenuOpen = false"
    />

    <!-- Main Content Area -->
    <div class="lg:pl-64 flex flex-col min-h-screen">
      <!-- Top Header -->
      <Header
        @toggle-sidebar="sidebarCollapsed = !sidebarCollapsed"
        @toggle-mobile="mobileMenuOpen = !mobileMenuOpen"
      />

      <!-- Breadcrumb Navigation -->
      <Breadcrumb class="border-b border-gray-200 dark:border-gray-700" />

      <!-- Page Content -->
      <main class="flex-1 py-6">
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <!-- Page Header -->
          <PageHeader v-if="$slots.header">
            <slot name="header" />
          </PageHeader>

          <!-- Main Content -->
          <div class="mt-6">
            <slot />
          </div>
        </div>
      </main>

      <!-- Footer -->
      <Footer />
    </div>

    <!-- Mobile Menu Overlay -->
    <div
      v-if="mobileMenuOpen"
      class="fixed inset-0 z-40 lg:hidden bg-black bg-opacity-50"
      @click="mobileMenuOpen = false"
    />
  </div>
</template>
```

## 🧭 Navigation Menu Structure

### Sidebar Menu Configuration

```typescript
// config/navigation.ts
export const navigationItems = [
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
      },
      {
        title: 'Organisation erstellen',
        href: '/organizations/create',
        description: 'Neue Organisation anlegen'
      }
    ]
  },
  {
    title: 'Gruppen',
    icon: 'UserGroupIcon',
    href: '/groups',
    badge: { count: 'groupsStore.activeGroups', color: 'blue' }
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
      },
      {
        title: 'Berechtigungen',
        href: '/permissions',
        description: 'Rollen und Berechtigungen'
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
          status: 'systemStore.healthStatus',
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
      },
      {
        title: 'Backup & Export',
        href: '/system/backup',
        description: 'Daten sichern und exportieren'
      }
    ]
  }
]
```

### Sidebar Component

```vue
<!-- components/layout/Sidebar.vue -->
<template>
  <div class="hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-64 lg:flex-col">
    <!-- Sidebar Background -->
    <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-white dark:bg-gray-800 px-6 shadow-lg">
      <!-- Logo Section -->
      <div class="flex h-16 shrink-0 items-center border-b border-gray-200 dark:border-gray-700">
        <img class="h-8 w-auto" src="/logo.svg" alt="UM-OIC" />
        <span class="ml-3 text-lg font-semibold text-gray-900 dark:text-white">
          UM-OIC Admin
        </span>
      </div>

      <!-- Navigation -->
      <nav class="flex flex-1 flex-col">
        <ul role="list" class="flex flex-1 flex-col gap-y-7">
          <!-- Main Navigation -->
          <li>
            <ul role="list" class="-mx-2 space-y-1">
              <li v-for="item in navigationItems" :key="item.title">
                <!-- Single Item -->
                <router-link
                  v-if="!item.children"
                  :to="item.href"
                  :class="[
                    $route.path === item.href
                      ? 'bg-blue-50 text-blue-600 dark:bg-blue-900/50 dark:text-blue-400'
                      : 'text-gray-700 hover:text-blue-600 hover:bg-gray-50 dark:text-gray-300 dark:hover:bg-gray-700',
                    'group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold'
                  ]"
                >
                  <component
                    :is="item.icon"
                    class="h-6 w-6 shrink-0"
                    aria-hidden="true"
                  />
                  {{ item.title }}

                  <!-- Badge -->
                  <Badge
                    v-if="item.badge"
                    :count="item.badge.count"
                    :color="item.badge.color"
                    class="ml-auto"
                  />
                </router-link>

                <!-- Collapsible Group -->
                <Disclosure v-else as="div" v-slot="{ open }">
                  <DisclosureButton
                    :class="[
                      isGroupActive(item.children)
                        ? 'bg-blue-50 text-blue-600 dark:bg-blue-900/50'
                        : 'text-gray-700 hover:text-blue-600 hover:bg-gray-50 dark:text-gray-300',
                      'flex items-center w-full text-left rounded-md p-2 gap-x-3 text-sm leading-6 font-semibold'
                    ]"
                  >
                    <component :is="item.icon" class="h-6 w-6 shrink-0" />
                    {{ item.title }}
                    <ChevronRightIcon
                      :class="[
                        open ? 'rotate-90 text-gray-500' : 'text-gray-400',
                        'ml-auto h-5 w-5 shrink-0'
                      ]"
                    />
                  </DisclosureButton>

                  <DisclosurePanel as="ul" class="mt-1 px-2">
                    <li v-for="subItem in item.children" :key="subItem.title">
                      <router-link
                        :to="subItem.href"
                        :class="[
                          $route.path === subItem.href
                            ? 'bg-blue-50 text-blue-600 dark:bg-blue-900/50'
                            : 'text-gray-600 hover:text-blue-600 hover:bg-gray-50 dark:text-gray-400',
                          'block rounded-md py-2 pr-2 pl-9 text-sm leading-6'
                        ]"
                      >
                        {{ subItem.title }}
                      </router-link>
                    </li>
                  </DisclosurePanel>
                </Disclosure>
              </li>
            </ul>
          </li>

          <!-- Bottom Section -->
          <li class="mt-auto">
            <div class="border-t border-gray-200 dark:border-gray-700 pt-4">
              <!-- User Info -->
              <div class="flex items-center gap-x-3 px-2 py-3">
                <img
                  class="h-8 w-8 rounded-full bg-gray-50"
                  :src="authStore.user?.avatar || '/default-avatar.svg'"
                  :alt="authStore.user?.full_name"
                />
                <div class="flex-1 text-sm leading-6">
                  <p class="font-semibold text-gray-900 dark:text-white">
                    {{ authStore.user?.full_name }}
                  </p>
                  <p class="text-gray-600 dark:text-gray-400">
                    {{ authStore.user?.email }}
                  </p>
                </div>
              </div>

              <!-- User Actions -->
              <div class="mt-2 space-y-1">
                <router-link
                  to="/profile"
                  class="group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 hover:bg-gray-50 dark:text-gray-300 dark:hover:bg-gray-700"
                >
                  <UserIcon class="h-6 w-6 shrink-0" />
                  Profil
                </router-link>

                <button
                  @click="authStore.logout()"
                  class="group flex w-full gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 hover:bg-gray-50 dark:text-gray-300 dark:hover:bg-gray-700"
                >
                  <ArrowLeftOnRectangleIcon class="h-6 w-6 shrink-0" />
                  Abmelden
                </button>
              </div>
            </div>
          </li>
        </ul>
      </nav>
    </div>
  </div>
</template>
```

## 📄 View Structure & Pages

### 1. Dashboard (Hauptseite)

```vue
<!-- views/Dashboard.vue -->
<template>
  <div>
    <!-- Welcome Section -->
    <div class="mb-8">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        Willkommen zurück, {{ authStore.user?.first_name }}
      </h1>
      <p class="mt-1 text-sm text-gray-600 dark:text-gray-400">
        Hier ist eine Übersicht über Ihr UM-OIC System
      </p>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4 mb-8">
      <StatCard
        title="Gesamt Benutzer"
        :value="stats.totalUsers"
        :change="stats.userGrowth"
        icon="UsersIcon"
        color="blue"
      />
      <StatCard
        title="Aktive Sessions"
        :value="stats.activeSessions"
        :change="stats.sessionChange"
        icon="ComputerDesktopIcon"
        color="green"
      />
      <StatCard
        title="Organisationen"
        :value="stats.organizations"
        :change="stats.orgGrowth"
        icon="BuildingOfficeIcon"
        color="purple"
      />
      <StatCard
        title="OAuth2 Clients"
        :value="stats.oauthClients"
        :change="stats.clientChange"
        icon="KeyIcon"
        color="orange"
      />
    </div>

    <!-- Charts Section -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
      <!-- User Activity Chart -->
      <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">
          Benutzer-Aktivität (7 Tage)
        </h3>
        <UserActivityChart :data="activityData" />
      </div>

      <!-- Login Distribution -->
      <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">
          Login-Verteilung nach Organisation
        </h3>
        <OrgLoginChart :data="loginDistribution" />
      </div>
    </div>

    <!-- Recent Activity -->
    <div class="bg-white dark:bg-gray-800 shadow rounded-lg">
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-medium text-gray-900 dark:text-white">
          Letzte Aktivitäten
        </h3>
      </div>
      <RecentActivityList :activities="recentActivities" />
    </div>
  </div>
</template>
```

### 2. User Management Views

```vue
<!-- views/users/UserList.vue -->
<template>
  <div>
    <!-- Page Header -->
    <div class="sm:flex sm:items-center sm:justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
          Benutzerverwaltung
        </h1>
        <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
          Verwalten Sie Benutzerkonten und Berechtigungen
        </p>
      </div>
      <div class="mt-4 sm:mt-0 flex space-x-3">
        <button
          @click="showImportModal = true"
          class="btn btn-secondary"
        >
          <ArrowUpTrayIcon class="w-4 h-4 mr-2" />
          Import
        </button>
        <router-link
          to="/users/create"
          class="btn btn-primary"
        >
          <PlusIcon class="w-4 h-4 mr-2" />
          Benutzer erstellen
        </router-link>
      </div>
    </div>

    <!-- Filters & Search -->
    <div class="mt-6 bg-white dark:bg-gray-800 shadow rounded-lg p-6">
      <UserFilters
        v-model:search="filters.search"
        v-model:organization="filters.organization"
        v-model:status="filters.status"
        v-model:role="filters.role"
        @apply="applyFilters"
        @reset="resetFilters"
      />
    </div>

    <!-- Data Table -->
    <div class="mt-6">
      <DataTable
        :columns="userColumns"
        :data="paginatedUsers"
        :loading="loading"
        :pagination="pagination"
        :selection="selectedUsers"
        @page-change="handlePageChange"
        @sort="handleSort"
        @select="handleSelection"
      >
        <!-- Custom column templates -->
        <template #status="{ row }">
          <StatusBadge :status="row.status" />
        </template>

        <template #admin="{ row }">
          <AdminBadge :admin-scopes="row.admin" />
        </template>

        <template #actions="{ row }">
          <UserActions
            :user="row"
            @edit="editUser"
            @delete="deleteUser"
            @reset-password="resetPassword"
            @toggle-status="toggleStatus"
          />
        </template>
      </DataTable>
    </div>

    <!-- Bulk Actions -->
    <BulkActions
      v-if="selectedUsers.length > 0"
      :selected-count="selectedUsers.length"
      @bulk-edit="showBulkEditModal = true"
      @bulk-delete="showBulkDeleteModal = true"
      @bulk-export="exportUsers"
    />

    <!-- Modals -->
    <ImportUsersModal v-model="showImportModal" />
    <BulkEditModal v-model="showBulkEditModal" :users="selectedUsers" />
    <BulkDeleteModal v-model="showBulkDeleteModal" :users="selectedUsers" />
  </div>
</template>
```

### 3. User Detail/Edit View

```vue
<!-- views/users/UserDetail.vue -->
<template>
  <div>
    <!-- Header with Breadcrumb -->
    <div class="mb-6">
      <nav class="flex" aria-label="Breadcrumb">
        <ol class="flex items-center space-x-4">
          <li>
            <router-link to="/users" class="text-gray-400 hover:text-gray-500">
              Benutzer
            </router-link>
          </li>
          <li>
            <ChevronRightIcon class="h-5 w-5 text-gray-400" />
          </li>
          <li class="text-gray-900 dark:text-white font-medium">
            {{ user?.full_name }}
          </li>
        </ol>
      </nav>

      <div class="mt-4 flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <Avatar :user="user" size="lg" />
          <div>
            <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
              {{ user?.full_name }}
            </h1>
            <p class="text-gray-600 dark:text-gray-400">{{ user?.email }}</p>
            <div class="flex items-center space-x-2 mt-1">
              <StatusBadge :status="user?.status" />
              <AdminBadge v-if="user?.admin?.length" :admin-scopes="user.admin" />
            </div>
          </div>
        </div>

        <UserActionMenu :user="user" />
      </div>
    </div>

    <!-- Tabs -->
    <TabGroup>
      <TabList class="border-b border-gray-200 dark:border-gray-700 mb-6">
        <Tab
          v-for="tab in tabs"
          :key="tab.key"
          v-slot="{ selected }"
          class="tab-button"
          :class="{ 'tab-active': selected }"
        >
          <component :is="tab.icon" class="w-4 h-4 mr-2" />
          {{ tab.label }}
        </Tab>
      </TabList>

      <TabPanels>
        <!-- General Info -->
        <TabPanel>
          <UserGeneralInfo
            :user="user"
            :editable="canEdit"
            @update="updateUser"
          />
        </TabPanel>

        <!-- Claims & Permissions -->
        <TabPanel>
          <UserClaimsEditor
            :user="user"
            :claims-registry="claimsRegistry"
            :editable="canEdit"
            @update="updateUserClaims"
          />
        </TabPanel>

        <!-- Active Sessions -->
        <TabPanel>
          <UserSessions
            :user-id="user?.id"
            @force-logout="forceLogout"
          />
        </TabPanel>

        <!-- Activity Log -->
        <TabPanel>
          <UserActivityLog :user-id="user?.id" />
        </TabPanel>
      </TabPanels>
    </TabGroup>
  </div>
</template>
```

### 4. Organization Views

```vue
<!-- views/organizations/OrgList.vue -->
<template>
  <div>
    <!-- Header -->
    <div class="sm:flex sm:items-center sm:justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
          Organisationen
        </h1>
        <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
          Übersicht aller Organisationen im System
        </p>
      </div>
    </div>

    <!-- Organization Grid -->
    <div class="mt-6 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
      <OrganizationCard
        v-for="org in organizations"
        :key="org.name"
        :organization="org"
        @click="goToOrganization"
        @manage-users="manageOrgUsers"
      />
    </div>

    <!-- Empty State -->
    <EmptyState
      v-if="organizations.length === 0"
      icon="BuildingOfficeIcon"
      title="Keine Organisationen"
      description="Es wurden noch keine Organisationen angelegt."
    />
  </div>
</template>
```

### 5. System Views

```vue
<!-- views/system/Status.vue -->
<template>
  <div>
    <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
      System Status
    </h1>

    <!-- Health Overview -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
      <HealthCard
        title="Auth Service"
        :status="systemStatus.authService"
        :last-check="systemStatus.lastAuthCheck"
      />
      <HealthCard
        title="Admin Service"
        :status="systemStatus.adminService"
        :last-check="systemStatus.lastAdminCheck"
      />
      <HealthCard
        title="Data Consistency"
        :status="systemStatus.dataConsistency"
        :stale="systemStatus.authDataStale"
      />
    </div>

    <!-- System Metrics -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
      <MetricsCard
        title="Performance Metrics"
        :metrics="performanceMetrics"
      />
      <MetricsCard
        title="Resource Usage"
        :metrics="resourceMetrics"
      />
    </div>

    <!-- Service Actions -->
    <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">
        Service-Aktionen
      </h3>
      <div class="space-y-3">
        <ActionButton
          @click="reloadAuthService"
          :loading="reloading"
          variant="primary"
        >
          Auth-Service neu laden
        </ActionButton>
        <ActionButton
          @click="validateDataConsistency"
          :loading="validating"
          variant="secondary"
        >
          Datenkonsistenz prüfen
        </ActionButton>
      </div>
    </div>
  </div>
</template>
```

## 🎨 Design System

### Color Palette

```css
/* Tailwind Config Extension */
module.exports = {
  theme: {
    extend: {
      colors: {
        // Primary Brand Colors
        primary: {
          50: '#eff6ff',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
          900: '#1e3a8a'
        },
        // Status Colors
        success: {
          50: '#f0fdf4',
          500: '#22c55e',
          600: '#16a34a'
        },
        warning: {
          50: '#fefce8',
          500: '#eab308',
          600: '#ca8a04'
        },
        error: {
          50: '#fef2f2',
          500: '#ef4444',
          600: '#dc2626'
        }
      }
    }
  }
}
```

### Component Variants

```vue
<!-- Example: StatusBadge.vue -->
<template>
  <span :class="badgeClasses">
    <span :class="dotClasses" />
    {{ label }}
  </span>
</template>

<script setup lang="ts">
const variants = {
  active: 'bg-green-100 text-green-800 border-green-200',
  inactive: 'bg-gray-100 text-gray-800 border-gray-200',
  suspended: 'bg-red-100 text-red-800 border-red-200'
}
</script>
```

Diese UI-Struktur bietet:

- **📱 Responsive Design** für alle Geräte
- **🌓 Dark Mode Support**
- **♿ Accessibility** mit Headless UI
- **🎨 Konsistentes Design** mit Tailwind
- **🔍 Erweiterte Suche** und Filter
- **📊 Dashboard mit Statistiken**
- **👥 Detaillierte Benutzerverwaltung**
- **🏢 Organisationsübersicht**
- **⚙️ System-Monitoring**
- **📋 Audit und Aktivitäts-Logs**