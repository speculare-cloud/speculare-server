<template>
	<div id="app">
		<div id="nav" class="p-8">
			<router-link to="/">Home</router-link>
		</div>
		<div class="container mx-auto">
			<router-view/>
		</div>
	</div>
</template>

<script>
export default {
	name: 'App',
}
</script>

<style>
@import './assets/uPlot.min.css';
#nav a {
	font-weight: bold;
	color: #2c3e50;
}

#nav a.router-link-exact-active {
	color: rgb(55, 48, 163);
}

.u-legend {
	text-align: left;
	padding-left: 50px;
}

.u-inline tr {
	margin-right: 8px;
}

.u-label {
	font-size: 10px;
}
</style>
