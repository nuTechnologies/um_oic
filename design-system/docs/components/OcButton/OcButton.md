---
title: OcButton component
next: false
prev: false
---

# OcButton component

## Description

The `OcButton` component is supposed to be used for interface actions. It's suitable for all-purpose use.

It defaults to an outlined button with a white background. A filled appearance should be used only once per view for the main call-to-action. All buttons are built with a css grid which ensures that there will be a pre-defined gutter between all child items.

## Accessibility

### Distinction when to use `<button>` and when to use an `<a>`

Regardless of the visual representation the following differentiation should be made if in doubt what to choose:

- An anchor/link changes the location, be it internally on the website, or to another resource/document/route.
- A button does change the state of the website, e.g.: adds, deletes, opens, ...

### Accessible name

The `accessible-name` of a button is derived from several sources. Right now, only two of them are relevant:

- The value of the `aria-label` attribute
- The text between the opening and closing tag: `<button>This text here</button>`

When an `aria-label` attribute exists, its value will override the button text. So in this case, the accessible name would be "foo": `<button aria-label="foo">Bar</button>`, although visual users will see "Bar". This difference between accessible name and visual name is a problem for a certain type of assistive technology (explainer for the term), this is why the WCAG success criterion 2.5.3, "Label in name" exists. This difference should be avoided, if it can't, W3C recommends that the accessible name should start with visible label.

### Icon-only button

Every icon-only button has to have an `aria-label` that describes the purpose of the button.

## Examples

### Appearance

::: livecode

```html
<nu-button appearance="filled">Filled</nu-button>
<nu-button appearance="outline">Outline</nu-button>
<nu-button appearance="raw" class="p-2 ml-1">Raw</nu-button>
<div class="p-2 mt-4" style="background: #000000;">
  <nu-button appearance="raw-inverse" class="p-2">Raw-inverse</nu-button>
</div>
```

:::

### Icons

::: livecode

```html
<nu-button appearance="filled"><nu-icon name="home" /><span>Home</span></nu-button>
<nu-button><nu-icon name="home" /><span>Home</span></nu-button>
<nu-button aria-label="Go to your home"><nu-icon name="home" /></nu-button>
<nu-button aria-label="Go to your home" appearance="raw" class="p-2 ml-1"
  ><nu-icon name="home"
/></nu-button>
```

:::

### Groups

::: livecode

```html
<div class="nu-button-group">
  <nu-button>Foo</nu-button>
  <nu-button>Bar</nu-button>
  <nu-button appearance="filled">Baz</nu-button>
</div>
```

:::

### Click handler

A click handler can be registered via the `@click` property.

::: livecode {path=/components/OcButton/handler.vue}
<<< @/components/OcButton/handler.vue
:::

::: component-api
