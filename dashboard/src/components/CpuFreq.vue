<template>
	<div class="cpufreq">
		<LineChart :chartdata="datacollection" :options="chartOptions"/>
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
			chartOptions: {
				legend: {
            		display: false
          		},
				elements: {
      				line: {
        				tension: 0
      				}
    			},
				scales: {
            		xAxes: [{
                		type: 'time',
                		time: {
                    		unit: 'second'
                		},
						gridLines: {
    						display: false,
  						},
            		}],
					yAxes: [{
      					gridLines: {
        					drawBorder: false,
      					},
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

		this.connection.onmessage = function(event) {
			let json = JSON.parse(event.data);
			let newValues = json["columnvalues"];

			vm.chartLabels.push(newValues[3]);

			if (vm.chartDataObj == null) {
				vm.chartDataObj = {
            		label: 'CpuFreq',
            		backgroundColor: '#f87979',
	            	data: [newValues[1]]
            	};
			} else {
				vm.chartDataObj.data.push(newValues[1]);
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