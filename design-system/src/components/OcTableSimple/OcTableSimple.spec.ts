import { shallowMount } from '@xwork-eu/web-test-helpers'

import Table from './OcTableSimple.vue'

describe('OcTableSimple', () => {
  it('adds hover', () => {
    const wrapper = shallowMount(Table, {
      props: {
        hover: true
      }
    })

    expect(wrapper.attributes('class')).toContain('nu-table-simple-hover')
  })
})
