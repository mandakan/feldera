<script lang="ts">
  import type { Snippet } from 'svelte'
  import type { Action } from 'svelte/action'

  let {
    value = $bindable(),
    children,
    onvalue,
    class: _class = ''
  }: {
    value: string
    children?: Snippet
    onvalue?: (value: string) => void
    class?: string
  } = $props()
  let showInput = $state(false)

  const autofocus: Action = (e) => {
    e.focus()
  }

  const handleSubmit = (e: { currentTarget: EventTarget & HTMLInputElement }) => {
    if (_value !== e.currentTarget.value) {
      value = _value = e.currentTarget.value
      onvalue?.(e.currentTarget.value)
    }
    showInput = false
  }
  // Local _value can be updated instantly and checked within sequential onblur and onkeydown handlers,
  // unlike $bindable value
  let _value = value
  $effect(() => {
    _value = value
  })
</script>

{#if showInput}
  <input
    use:autofocus
    {value}
    onblur={(e) => {
      handleSubmit(e)
    }}
    onkeydown={(e) => {
      if (e.code !== 'Enter') {
        return
      }
      handleSubmit(e)
    }}
    class={_class}
  />
{:else}
  <span class="group" role="button" tabindex={0} ondblclick={() => (showInput = true)}>
    {@render children?.()}
    <button
      onclick={() => (showInput = true)}
      class="fd fd-edit text-[20px] text-surface-400-600 group-hover:text-surface-950-50"
      aria-label="Edit pipeline name"
    >
    </button>
  </span>
{/if}
