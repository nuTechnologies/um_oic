---
title: OcBreadcrumb component
next: false
prev: false
---

# OcBreadcrumb component

## Description

The `OcBreadcrumb` component displays given items in a breadcrumb style.

## Examples

### Default

The default use case gets a list of items to display.

::: livecode

```html
<nu-breadcrumb
  :items="[
		{ text: 'Root', to: { path: 'root' } },
		{ text: 'Folder', to: { path: 'folder' } },
		{ text: 'Subfolder', to: { path: 'subfolder' } }
	]"
/>
<nu-breadcrumb
  :items="[
		{ text: 'Root', to: { path: 'root' } },
		{ text: 'Folder', to: { path: 'folder' } },
		{ text: 'Subfolder', to: { path: 'subfolder' } }
	]"
  truncation-offset="3"
/>
```

:::

### Context menu

The last item of the breadcrumbs can have a context menu. This is useful when it represents the current folder and the user can perform actions on it.

::: livecode

```html
<nu-breadcrumb
  :items="[
		{ text: 'Root', to: { path: 'root' } },
		{ text: 'Folder', to: { path: 'folder' } },
		{ text: 'Subfolder', to: { path: 'subfolder' } }
	]"
  truncation-offset="3"
  :show-context-actions="true"
>
  <template v-slot:contextMenu>
    <span>This is an example item</span>
  </template>
</nu-breadcrumb>
```

:::

::: component-api
