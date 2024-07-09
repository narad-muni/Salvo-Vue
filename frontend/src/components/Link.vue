<script setup lang="ts">
    import { ref } from 'vue';
    
    const props = defineProps<{ href: string }>()
</script>

<script lang="ts">
    export const currentPath = ref(window.location.pathname);

    export function navigate(href: string) {
        window.history.pushState('', '', href);
        currentPath.value = window.location.pathname;
    }
</script>

<template>
    <a @click.prevent="navigate(props.href)" href="{{ props.href }}">
        <slot />
    </a>
</template>