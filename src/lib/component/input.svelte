<script lang="ts">
    export let inputType = "text";
    export let inputName = "default";


</script>

<input type="{inputType}" name="{inputName}" id="{inputName}" class="rounded p-2 h-9 mt-1 border-purple-500 w-4/5 border-2 focus:border-2 focus:border-purple-200">

<style>
    input:focus{
	outline: none
}
</style>