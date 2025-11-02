---
title: OcAvatar component
next: false
prev: false
---

# OcAvatar component

## Description

The `OcAvatar` component is a thumbnail that is usually representing a user or a group. It can either be an image or the initials of a given username. There are a few variants of the component that can be used in specific contexts. See the examples below.

If you want to use multiple avatar items in one container, have a look at [OcAvatars](./OcAvatars).

## Accessibility

The component can be provided with an `accessible-label` in case the avatar is used alone. In case the avatar is used next to a username or display name, it should be left empty. If not specified, an avatar will get `aria-hidden="true"`.

## Examples

### Default

::: livecode

```vue
<nu-avatar src="https://picsum.photos/50/50?image=550" accessible-label="Some avatar" />
<nu-avatar user-name="Alan Turing" accessible-label="Alan Turing" />
```

:::

### Variants

::: livecode

```vue
<nu-avatar-federated name="Federated User" accessible-label="Federated User" />
<nu-avatar-group name="Some group" accessible-label="Some group" />
<nu-avatar-guest name="Guest user" accessible-label="Guest user" />
<nu-avatar-link name="Link" accessible-label="Link" />
```

:::

::: component-api
