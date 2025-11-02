<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <!-- Mobile menu overlay -->
    <div
      v-if="mobileMenuOpen"
      class="relative z-50 lg:hidden"
      role="dialog"
      aria-modal="true"
    >
      <!-- Background backdrop -->
      <div
        class="fixed inset-0 bg-gray-900/80"
        @click="mobileMenuOpen = false"
      ></div>

      <!-- Mobile sidebar -->
      <div class="fixed inset-0 flex">
        <div class="relative mr-16 flex w-full max-w-xs flex-1">
          <!-- Close button -->
          <div class="absolute left-full top-0 flex w-16 justify-center pt-5">
            <button
              type="button"
              class="-m-2.5 p-2.5"
              @click="mobileMenuOpen = false"
            >
              <span class="sr-only">Seitenleiste schließen</span>
              <XMarkIcon class="h-6 w-6 text-white" aria-hidden="true" />
            </button>
          </div>

          <!-- Mobile sidebar content -->
          <Sidebar :mobile="true" @close="mobileMenuOpen = false" />
        </div>
      </div>
    </div>

    <!-- Static sidebar for desktop -->
    <Sidebar class="hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-64 lg:flex-col" />

    <!-- Main content area -->
    <div class="lg:pl-64">
      <!-- Top header -->
      <Header @toggle-mobile-menu="mobileMenuOpen = true" />

      <!-- Breadcrumb navigation -->
      <Breadcrumb />

      <!-- Main content -->
      <main class="py-6 pb-24">
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <!-- Page header slot -->
          <div v-if="$slots.header" class="mb-6">
            <slot name="header" />
          </div>

          <!-- Main content slot -->
          <slot />
        </div>
      </main>

      <!-- Footer -->
      <Footer />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { XMarkIcon } from '@heroicons/vue/24/outline'
import Sidebar from './Sidebar.vue'
import Header from './Header.vue'
import Breadcrumb from './Breadcrumb.vue'
import Footer from './Footer.vue'

const mobileMenuOpen = ref(false)
</script>