<template>
	<div class="details">
		<h1 class="text-6xl font-normal leading-normal mt-0 mb-2 text-gray-800">Details</h1>
		<p>{{ this.$route.params.uuid }}</p>

		<div role="section" class="mt-8">
			<h3 class="text-2xl">CPU</h3>
			<CpuFreq :uuid="this.$route.params.uuid"/>
		</div>
		<div role="section" class="mt-4">
			<h3 class="text-2xl">Disks</h3>
			<DisksThroughput :uuid="this.$route.params.uuid"/>
		</div>
	</div>
</template>

<script>
import CpuFreq from '../components/CpuFreq';
import DisksThroughput from '../components/DisksThroughput';

export default {
	name: 'Details',
	components: {
    	CpuFreq,
		DisksThroughput
  	},

	beforeRouteUpdate (to, from, next) {
		next();
  	}
}
</script>