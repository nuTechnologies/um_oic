<template>
  <nu-button id="toggle-bottom-drawer-parent">Open bottom drawer</nu-button>
  <nu-bottom-drawer
    ref="parent"
    drawer-id="example-bottom-drawer-parent"
    toggle="#toggle-bottom-drawer-parent"
    title="Example Bottom Drawer Parent"
    @show="renderChild = true"
    @hide="renderChild = false"
  >
    <nu-list class="flex flex-col p-2">
      <nu-button justify-content="left" appearance="raw" no-hover aria-expanded="false">
        <nu-icon size="small" name="rectangle" fill-type="line" />
        Simple button
      </nu-button>
      <nu-button
        id="toggle-bottom-drawer-child"
        class="mt-2"
        justify-content="left"
        appearance="raw"
        no-hover
        aria-expanded="false"
      >
        <nu-icon size="small" name="information" fill-type="line" />
        Open child parent drawer...
      </nu-button>
    </nu-list>
  </nu-bottom-drawer>
  <nu-bottom-drawer
    v-if="renderChild"
    drawer-id="example-bottom-drawer-child"
    toggle="#toggle-bottom-drawer-child"
    title="Example Bottom Drawer Child"
    :is-nested-element="true"
    :nested-parent-ref="parent"
  >
    <nu-list class="flex flex-col p-2">
      <nu-button justify-content="left" appearance="raw" no-hover aria-expanded="false">
        <nu-icon size="small" name="rectangle" fill-type="line" />
        Simple button
      </nu-button>
    </nu-list>
  </nu-bottom-drawer>
</template>
<script setup lang="ts">
import { OcBottomDrawer } from '../../../src/components'
import { ref, useTemplateRef } from 'vue'

const parent = useTemplateRef('parent')
const renderChild = ref(false)
</script>
