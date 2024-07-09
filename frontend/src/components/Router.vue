<script setup lang="ts">
import { computed } from 'vue';
import { currentPath } from './Link.vue';

    const props = defineProps<{
        routes: any
    }>()

    const currentView = computed(() => {
        return props.routes[currentPath.value || '/']
    })
</script>

<template>
    <component :is="currentView" />
</template>