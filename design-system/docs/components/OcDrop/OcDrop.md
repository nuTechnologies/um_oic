---
title: OcDrop component
next: false
prev: false
---

# OcDrop component

## Description

The `OcDrop` component displays given content or action items inside a dropdown menu.

## Examples

### Default

The most common use case of the component is in combination with a button. It's important that the button `id` matches the `toggle` attribute of the dropdown.

::: livecode

```html
<nu-button id="drop-btn">Open drop</nu-button>
<nu-drop toggle="#drop-btn" mode="click" padding-size="medium"> Some content. </nu-drop>
```

:::

### Action items

The following example shows how to use the component to display action items.

::: livecode

```html
<nu-button id="drop-2-btn">Open drop</nu-button>
<nu-drop drop-id="drop-drop" toggle="#drop-2-btn" mode="click" padding-size="small">
  <nu-list :raw="true">
    <li>
      <nu-button class="w-full" justify-content="left" appearance="raw"> Create Folder </nu-button>
    </li>
    <li>
      <nu-button class="w-full" justify-content="left" appearance="raw"> Create Space </nu-button>
    </li>
    <li>
      <nu-button class="w-full" justify-content="left" appearance="raw"> Create File </nu-button>
    </li>
  </nu-list>
</nu-drop>
```

:::

::: component-api
