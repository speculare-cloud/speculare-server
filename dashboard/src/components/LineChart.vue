<template>
	<canvas id="lineChart" @mouseover="hover = true" @mouseleave="hover = false"></canvas>
</template>

<script>
import Chart from 'chart.js';

export default {
	name: 'LineChart',
	props: {
		options: {
			type: Object,
			default: null
    	},
		chartdata: {
			type: Object,
			default: null
    	},
		minvalue: {
			type: Date,
			default: null
		}
  	},

	data () {
		return {
			chart: null,
			ctx: null,
			hover: false
		}
	},
	
	watch: {
		chartdata: function (newData, oldData) {
			if (oldData) {
				this.chart.update();
			} else {
				if (this.chart) {
					this.chart.destroy();
				}
				if (this.ctx) {
					this.chart = new Chart(this.ctx, {
						type: 'line',
						data: this.chartdata,
						options: this.options
					})
				}
			}
		},
		minvalue: function (newData, oldData) {
			this.chart.options.scales.xAxes[0].ticks.min = newData;
			this.chart.update();
		}
	},

	mounted () {
		if (this.chart == null) {
			this.ctx = document.getElementById('lineChart').getContext('2d');
			this.chart = new Chart(this.ctx, {
				type: 'line',
				data: this.chartdata,
				options: this.options
			});
		}
	}
}
</script>