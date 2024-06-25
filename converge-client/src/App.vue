<template>
  <div class="container">
    <frame></frame>
    <router-view></router-view>
  </div>
</template>
<script setup>
import { RouterView } from 'vue-router';
import frame from '@/components/frame.vue';

</script>
<style scoped></style>
