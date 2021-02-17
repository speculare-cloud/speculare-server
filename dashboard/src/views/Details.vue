<template>
	<div class="details">
		<h1 class="text-6xl font-normal leading-normal mt-0 mb-2 text-gray-800">Details</h1>
		<p>{{ this.$route.params.uuid }}</p>

		<CpuFreq :uuid="this.$route.params.uuid"/>
	</div>
</template>

<script>
import CpuFreq from '../components/CpuFreq';

export default {
	name: 'Details',
	components: {
    	CpuFreq
  	},

	beforeRouteUpdate (to, from, next) {
		next();
  	}
}
</script>