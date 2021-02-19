<template>
	<div class="cpufreq">
		<RealTimeChart :chartdata="datacollection" :options="chartOptions"/>
	</div>
</template>

<script>
import LineChart from '@/components/RealTimeChart'

export default {
	name: 'cpufreq',
	props: ['uuid'],
	connection: null,
	components: {
		LineChart
	},

	data () {
		return {
			datacollection: null,
			chartLabels: [],
			chartDataObj: [],
			chartOptions: {
				...this.getSize(),
				cursor: {
					dataIdx: (self, seriesIdx, hoveredIdx) => {
						let seriesData = self.data[seriesIdx];

						if (seriesData[hoveredIdx] == null) {
							let nonNullLft = hoveredIdx,
								nonNullRgt = hoveredIdx,
								i;

							i = hoveredIdx;
							while (nonNullLft == hoveredIdx && i-- > 0)
								if (seriesData[i] != null)
									nonNullLft = i;

							i = hoveredIdx;
							while (nonNullRgt == hoveredIdx && i++ < seriesData.length)
								if (seriesData[i] != null)
									nonNullRgt = i;

							return nonNullRgt - hoveredIdx > hoveredIdx - nonNullLft ? nonNullLft : nonNullRgt;
						}

						return hoveredIdx;
					},
					drag: {
						setScale: false,
					}
				},
				legend: {
					live: false,
				},
				select: {
					show: false,
				},
				series: [
					{},
					{
						label: "CPU Frequency",
						stroke: "red",
						fill: "rgba(255,0,0,0.05)",
					}
				],
				scales: {
					x: {
						time: true,
					},
					y: {
						auto: true,
					},
				}
			}
		}
	},

	created: function() {		
		let vm = this;
		
		if (vm.connection == null) {
			console.log("[CPU] Starting connection to WebSocket Server");
			vm.connection = new WebSocket("wss://cdc.speculare.cloud:9641/ws?change_table=cpu_info&specific_filter=host_uuid.eq." + vm.uuid);
		}

		this.connection.onmessage = function(event) {
			let json = JSON.parse(event.data);
			let newValues = json["columnvalues"];

			let date_obj = new Date(newValues[3]).valueOf() / 1000;
			vm.chartLabels.push(date_obj);
			vm.chartDataObj.push(newValues[1]);

			// 5 mins history
			if (vm.chartDataObj.length > (60 * 5)) {
				vm.chartLabels.shift();
				vm.chartDataObj.shift();
			}

			// Apply the Y range when we first got the data,
			// as the graph is drawn using this data for now
			if (vm.datacollection == null) {
				// If the newData[1] contains more than 4000 items, use a for loop
				// https://medium.com/coding-at-dawn/the-fastest-way-to-find-minimum-and-maximum-values-in-an-array-in-javascript-2511115f8621
				let max = Math.max.apply(null, vm.chartDataObj);
				vm.chartOptions.scales.y = {
					auto: false,
					range: [0, max + (max / 10)]
				}
			}

			vm.datacollection = [
				vm.chartLabels,
				vm.chartDataObj,
			];
		}
	},

	methods: {
		getSize: function() {
			return {
				width: window.innerWidth,
				height: 300,
			}
		}
	},

	beforeDestroy: function() {
		console.log("[CPU] Closing the WebSocket connection");
		this.connection.close();
		this.connection = null;
	}
}
</script>