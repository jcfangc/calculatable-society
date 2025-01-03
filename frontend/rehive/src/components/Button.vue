<template>
  <button class="btn">
    提交
  </button>
</template>

<script lang="ts" setup>

</script>

<style scoped lang="scss">
@use "@/assets/style/default/index.scss" as *;

.btn {
  @include button-style(0.5rem, 1rem, 1rem);
}
</style>
