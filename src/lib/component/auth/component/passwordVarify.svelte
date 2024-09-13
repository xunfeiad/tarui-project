<script lang="ts">
  import { RangeSlider } from '@skeletonlabs/skeleton';

  let value = 15;
</script>

<div class="password-strength ml-2">
  <!-- <div>Password strength:</div> -->
</div>
<RangeSlider name="range-slider" bind:value={value} max={25} step={1} ticked>Label</RangeSlider>
