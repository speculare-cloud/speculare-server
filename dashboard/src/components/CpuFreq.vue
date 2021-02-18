<template>
	<div class="cpufreq">
		<LineChart :chartdata="datacollection" :options="chartOptions" :minvalue="chartMinval"/>
	</div>
</template>

<script>
import LineChart from '@/components/LineChart'

export default {
	name: 'cpufreq',
	props: ['uuid'],
	connection: null,
	components: {
		LineChart,
	},

	data () {
		return {
			datacollection: null,
			chartLabels: [],
			chartDataObj: null,
			chartMinval: null,
			chartOptions: {
				legend: {
            		display: false
          		},
				elements: {
      				line: {
        				tension: 0
      				},
					point:{
                        radius: 0,
                    }
    			},
				hover: {
					mode: 'nearest',
					intersect: true
				},
				tooltips: {
					mode: 'index',
					intersect: false,
				},
				scales: {
            		xAxes: [{
                		type: 'time',
                		time: {
                    		unit: 'second',
							unitStepSize: 30
                		},
						gridLines: {
    						display: false,
  						},
						ticks: {
        					min : null,
						}
            		}],
					yAxes: [{
      					gridLines: {
							display: false,
      					},
            			ticks: {
                			beginAtZero: true
            			}
    				}]
        		},
				responsive: true,
      			maintainAspectRatio: false
			}
		}
	},

	created: function() {		
		let vm = this;
		
		if (vm.connection == null) {
			console.log("[CPU] Starting connection to WebSocket Server");
			vm.connection = new WebSocket("wss://cdc.speculare.cloud:9641/ws?change_table=cpu_info&specific_filter=host_uuid.eq." + vm.uuid);
		}

		let count = 0;

		this.connection.onmessage = function(event) {
			let json = JSON.parse(event.data);
			let newValues = json["columnvalues"];

			let date_with_no_ms = newValues[3].replace(/\.\d+/, "");
			vm.chartLabels.push(date_with_no_ms);

			// Update the min date
			let tmp = new Date(date_with_no_ms);
			tmp.setMinutes(tmp.getMinutes() - 5);
			vm.chartMinval = tmp;

			if (vm.chartDataObj == null) {
				vm.chartDataObj = {
            		label: 'CpuFreq',
            		backgroundColor: '#f87979',
	            	data: [newValues[1]]
            	};
			} else {
				vm.chartDataObj.data.push(newValues[1]);
			}

			if (vm.chartDataObj.data.length > (60 * 5) + 15) {
				vm.chartDataObj.data.shift();
				vm.chartLabels.shift();
			}

			vm.datacollection = {
				labels: vm.chartLabels,
				datasets: [vm.chartDataObj]
			};
		}
	},

	beforeDestroy: function() {
		console.log("[CPU] Closing the WebSocket connection");
		this.connection.close();
		this.connection = null;
	}
}
</script>