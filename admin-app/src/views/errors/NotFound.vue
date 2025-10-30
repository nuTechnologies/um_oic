<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
    <div class="sm:mx-auto sm:w-full sm:max-w-md">
      <div class="text-center">
        <h1 class="text-6xl font-bold text-gray-900 dark:text-white">404</h1>
        <h2 class="mt-4 text-2xl font-bold text-gray-900 dark:text-white">
          Seite nicht gefunden
        </h2>
        <p class="mt-2 text-gray-600 dark:text-gray-400">
          Die angeforderte Seite konnte nicht gefunden werden.
        </p>
      </div>

      <div class="mt-8 flex justify-center space-x-4">
        <router-link
          to="/dashboard"
          class="btn btn-primary"
        >
          Zum Dashboard
        </router-link>
        <button
          @click="$router.go(-1)"
          class="btn btn-secondary"
        >
          Zurück
        </button>
      </div>
    </div>
  </div>
</template>