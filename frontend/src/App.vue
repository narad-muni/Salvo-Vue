<script setup lang="ts">
    import Home from './pages/Home.vue';
    import Blog from './pages/Blog.vue';
    import Link from './components/Link.vue';
    import Router from './components/Router.vue';

    const routes = {
        "/": Home,
        "/blog": Blog,
    }

    

</script>

<template>
    <Link href="/">Home</Link><br/>
    <Link href="/blog">About</Link><br/>
    <Link href="/non-existent-path">Broken Link</Link>
    
    <Router :routes="routes" />
</template>