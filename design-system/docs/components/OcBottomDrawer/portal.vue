<template>
  <portal-target name="app.design-system.nu-bottom-drawer.portal" />

  <nu-button id="toggle-bottom-drawer-portal">Open bottom drawer</nu-button>
  <nu-bottom-drawer
    drawer-id="example-bottom-drawer"
    toggle="#toggle-bottom-drawer-portal"
    title="Example Bottom Drawer"
    use-portal
    portal-target="app.design-system.nu-bottom-drawer.portal"
  >
    Some content in the bottom drawer.
  </nu-bottom-drawer>
</template>
<script setup lang="ts">
import { OcBottomDrawer } from '../../../src/components'
</script>
