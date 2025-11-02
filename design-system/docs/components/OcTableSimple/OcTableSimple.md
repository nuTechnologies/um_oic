---
title: OcTableSimple component
next: false
prev: false
---

# OcTableSimple component

## Description

The `OcTableSimple` component represents a simple table. As opposed to the [OcTable](./OcTable) component, it can't be generated dynamically and needs to be built manually.

## Examples

### Default

The component provides a default slot that can be filled with standard HTML table elements (or their equivalents from this design system).

::: livecode

```html
<nu-table-simple>
  <nu-table-head>
    <nu-table-tr>
      <nu-table-th>ID</nu-table-th>
      <nu-table-th>Filename</nu-table-th>
      <nu-table-th>Size</nu-table-th>
    </nu-table-tr>
  </nu-table-head>
  <nu-table-body>
    <nu-table-tr>
      <nu-table-td>83558362-3fc6-4b96-a2e5-dba7435c4fae</nu-table-td>
      <nu-table-td>textfile.txt</nu-table-td>
      <nu-table-td>50</nu-table-td>
    </nu-table-tr>
    <nu-table-tr>
      <nu-table-td>fbd793d3-c36c-4f92-bff6-dfeebaec8248</nu-table-td>
      <nu-table-td>Folder</nu-table-td>
      <nu-table-td>9482</nu-table-td>
    </nu-table-tr>
  </nu-table-body>
</nu-table-simple>
```

:::

::: component-api
