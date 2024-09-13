<script>
    import {createEventDispatcher} from "svelte"

    export let loading = false
    export let transition = "transition ease-in-out delay-150 hover:-translate-y-1 hover:bg-purple-300 duration-300"
    $: disabled = !loading

    const dispatch = createEventDispatcher();

    function clickDispatch() {
        dispatch('message', {
            text: 'Hello!'
        });
    }
</script>

<div class="flex justify-center mt-2">
    <button class="bg-purple-600 text-white rounded leading-9 w-1/3 h-9 {transition} flex flex-row justify-center items-center ml-0"
             on:click={clickDispatch} >
        {#if loading === false}
            <slot></slot>
        {:else}
            <svg class="animate-spin h-5 w-5 ml-2 text-white basis-1/5" xmlns="http://www.w3.org/2000/svg" fill="none"
                 viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor"
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <span class="basis-4/5">Loading...</span>
        {/if}

    </button>
</div>