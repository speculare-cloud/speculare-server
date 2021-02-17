<template>
	<div id="app">
		<div id="nav" class="p-8">
			<router-link to="/">Home</router-link>
		</div>
		<router-view/>
	</div>
</template>

<script>
export default {
	name: 'App',
}
</script>

<style>
#nav a {
	font-weight: bold;
	color: #2c3e50;
}

#nav a.router-link-exact-active {
	color: #42b983;
}
</style>
