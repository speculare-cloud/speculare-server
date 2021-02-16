<template>
	<div class="details">
		<h1 class="text-6xl font-normal leading-normal mt-0 mb-2 text-gray-800">Details</h1>
		<p>{{ this.$route.params.uuid }}</p>
	</div>
</template>

<script>
export default {
	name: 'Details',
	connection: null,

	beforeRouteUpdate (to, from, next) {
    	// react to route changes...
    	// don't forget to call next()
		next();
  	}
}
</script>