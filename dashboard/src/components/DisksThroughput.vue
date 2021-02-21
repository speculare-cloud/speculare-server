<template>
	<div class="disksthroughput">
		<SteppedChart :chartdata="datacollection" />
	</div>
</template>

<script>
import SteppedChart from '@/components/SteppedChart'

export default {
	name: 'disksthroughput',
	props: ['uuid'],
	connection: null,
	components: {
		SteppedChart
	},

	data () {
		return {
			datacollection: null,
			chartLabels: [],
			chartDataObj: [],
			dataHistory: [],
			// Workaround until I handle multiple disks
			which: null
		}
	},

	created: function() {		
		let vm = this;
		
		if (vm.connection == null) {
			console.log("[CPU] Starting connection to WebSocket Server");
			vm.connection = new WebSocket("wss://cdc.speculare.cloud:9641/ws?change_table=iostats&specific_filter=host_uuid.eq." + vm.uuid);
		}

		this.connection.onmessage = function(event) {
			let json = JSON.parse(event.data);
			let newValues = json["columnvalues"];
			
			if (this.which == null) {
				this.which = newValues[1];
			} else if (newValues[1] != this.which) {
				return;
			}

			let date_obj = new Date(newValues[5]).valueOf() / 1000;
			vm.chartLabels.push(date_obj);

			if (vm.chartDataObj.length == 0) {
				vm.chartDataObj.push(0);
			} else {
				let previous = vm.dataHistory[vm.dataHistory.length - 1];
				let diff = newValues[3] - previous;
				vm.chartDataObj.push(diff / 1000000);
			}
			vm.dataHistory.push(newValues[3]);

			// 5 mins history
			if (vm.chartDataObj.length > (60 * 5)) {
				vm.chartLabels.shift();
				vm.chartDataObj.shift();
			}

			vm.datacollection = [
				vm.chartLabels,
				vm.chartDataObj,
			];
		}
	},

	beforeDestroy: function() {
		console.log("[CPU] Closing the WebSocket connection");
		this.connection.close();
		this.connection = null;
	}
}
</script>