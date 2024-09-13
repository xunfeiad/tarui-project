<script lang="ts">
  import { progress } from "../store";
  let show = true;
  $: show = $progress !== 0;
</script>

{#if show === true}
  <progress value={$progress}></progress>
{/if}

<style>
    progress{
    width: 100%;
    border-radius: .2rem;        /*进度条外圆角*/
    font-size: 0;            /*进度条高度空大*/
    overflow: hidden;
}
  progress::-webkit-progress-bar {
    background: #1c1917;
  }
  progress::-webkit-progress-value {
    background: #ddb765;
    border-radius: 0.2rem; /*进度条内圆角*/
  }
</style>
