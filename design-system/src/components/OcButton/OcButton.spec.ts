import { PartialComponentProps, defaultPlugins, shallowMount } from '@xwork-eu/web-test-helpers'
import OcButton from './OcButton.vue'

describe('OcButton', () => {
  it('should display slot html', () => {
    const wrapper = getWrapperWithTestSlot()
    const slot = wrapper.find('p')
    expect(slot).toBeTruthy()
    expect(slot.attributes('class')).toBe('text')
    expect(slot.text()).toBe('Test button')
  })

  describe('click event', () => {
    it('should emit click event when click is triggered', async () => {
      const wrapper = getWrapperWithProps({})
      await wrapper.trigger('click')
      expect(wrapper.emitted('click')).toBeTruthy()
    })
    it.each`
      type
      ${'a'}
      ${'router-link'}
    `('should not emit click event when type is $type', async ({ type }) => {
      const wrapper = getWrapperWithProps({ type: type })
      await wrapper.trigger('click')
      expect(wrapper.emitted('click')).toBeFalsy()
    })
  })
  describe('when nu button is disabled', () => {
    let wrapper: ReturnType<typeof getWrapperWithProps>
    beforeEach(() => {
      wrapper = getWrapperWithProps({ disabled: true })
    })
    it('should have disabled attribute set to disabled', () => {
      // disabled: true => '' disabled: false => undefined ¯\_(ツ)_/¯
      expect(wrapper.attributes('disabled')).toBe('')
    })
    it('should not emit click event', async () => {
      await wrapper.trigger('click')
      expect(wrapper.emitted('click')).toBeFalsy()
    })
  })
  describe('different types of button', () => {
    it.each`
      type             | expectLink | expectButton | expectRouterLink
      ${'a'}           | ${true}    | ${false}     | ${false}
      ${'button'}      | ${false}   | ${true}      | ${false}
      ${'router-link'} | ${false}   | ${false}     | ${true}
    `('can behave as a $type', ({ type, expectLink, expectButton, expectRouterLink }) => {
      const wrapper = getWrapperWithProps({ type: type })
      expect(wrapper.find('a').exists()).toBe(expectLink)
      expect(wrapper.find('button').exists()).toBe(expectButton)
      expect(wrapper.find('router-link-stub').exists()).toBe(expectRouterLink)
    })
  })
  describe('different sizes of button', () => {
    it.each`
      size        | expectedClass
      ${'small'}  | ${'text-sm'}
      ${'medium'} | ${'text-base'}
      ${'large'}  | ${'text-lg'}
    `(
      'when size prop is set as $size class $expectedClass should be assigned',
      ({ size, expectedClass }) => {
        const wrapper = getWrapperWithProps({
          size: size
        })
        expect(wrapper.attributes('class')).toContain(expectedClass)
      }
    )
  })
  describe('default prop values', () => {
    it.each`
      name                           | expected
      ${'size'}                      | ${'text-base'}
      ${'color role'}                | ${'nu-button-secondary'}
      ${'gap size'}                  | ${'gap-2'}
      ${'justify content'}           | ${'justify-center'}
      ${'appearance'}                | ${'nu-button-outline'}
      ${'color role and appearance'} | ${'nu-button-secondary-outline'}
    `('should have attribute "$name" as "$expected"', ({ expected }) => {
      const wrapper = getWrapperWithProps({})
      expect(wrapper.attributes('class')).toContain(expected)
    })
  })
  describe('nu button appearance', () => {
    describe('when appearance is "filled"', () => {
      it('should not have wrong appearance classes', () => {
        const wrapper = getWrapperWithProps({
          appearance: 'filled'
        })
        expect(wrapper.attributes('class')).toContain('nu-button-filled')
        expect(wrapper.attributes('class')).not.toContain('nu-button-raw')
        expect(wrapper.attributes('class')).not.toContain('nu-button-raw-inverse')
        expect(wrapper.attributes('class')).not.toContain('nu-button-outline')
      })
    })
    describe('when nu button is initialized with color role and appearance', () => {
      it.each`
        colorRole      | appearance   | expectedClass
        ${'secondary'} | ${'raw'}     | ${'nu-button-secondary nu-button-raw nu-button-secondary-raw'}
        ${'secondary'} | ${'outline'} | ${'nu-button-secondary nu-button-outline nu-button-secondary-outline'}
        ${'primary'}   | ${'raw'}     | ${'nu-button-primary nu-button-raw nu-button-primary-raw'}
        ${'primary'}   | ${'outline'} | ${'nu-button-primary-outline'}
      `('should have extra appearance class', ({ colorRole, appearance, expectedClass }) => {
        const wrapper = getWrapperWithProps({
          appearance: appearance,
          colorRole: colorRole
        })
        expect(wrapper.attributes('class')).toContain(expectedClass)
      })
    })
  })
})

function getWrapperWithProps(props: PartialComponentProps<typeof OcButton>) {
  return shallowMount(OcButton, { props, global: { plugins: [...defaultPlugins()] } })
}
function getWrapperWithTestSlot() {
  const testSlots = { default: '<p class="text">Test button</p>' }
  return shallowMount(OcButton, { slots: testSlots, global: { plugins: [...defaultPlugins()] } })
}
