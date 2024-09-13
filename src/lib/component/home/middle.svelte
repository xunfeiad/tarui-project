<script lang="ts">
    export let functions: Array<string> = ["我的汪圈", "每日签到", "设置"];

</script>
<div class="flex flex-col">
    {#each functions as name}
        <div class="basis-1/6 bg-white rounded mt-4 h-10 leading-10">
            {name}
        </div>
    {/each}
</div>