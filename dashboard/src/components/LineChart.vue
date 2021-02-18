<template>
	<div class="w-100">
		Cpu Frequencies
		<div id="lineChart" @mouseover="hover=true" @mouseleave="hover=false"></div>
	</div>
</template>

<script>
import uPlot from 'uplot';

export default {
	name: 'LineChart',
	props: {
		options: Object,
		chartdata: {
			type: Array,
			default: null
    	},
  	},

	data () {
		return {
			ctx: null,
			chart: null,
			hover: false,
			max: 0
		}
	},
	
	watch: {
		chartdata: function (newData, oldData) {
			if (oldData == null || !this.chart) {
				this.createChart(newData);
			} else if (!this.hover && this.chart) {
				// If the newData[1] contains more than 4000 items, use a for loop
				// https://medium.com/coding-at-dawn/the-fastest-way-to-find-minimum-and-maximum-values-in-an-array-in-javascript-2511115f8621
				let max = Math.max.apply(null, newData[1]);
				if (this.max != max) {
					this.chart.setScale('y', { min: 0, max: (max + (max / 10)) });
				}
				this.chart.setData(newData);
			}
		}
	},

	mounted () {
		if (this.ctx == null) {
			this.ctx = document.getElementById('lineChart');
		}

		this.$nextTick(function() {
			window.addEventListener('resize', this.setChartSize);
		});
	},

	methods: {
		createChart: function(data) {
			this.chart = new uPlot(this.options, data, this.ctx);
		},
		getSize: function() {
			return {
				width: window.innerWidth,
				height: 300,
			}
		},
		setChartSize: function (event) {
			if (this.chart) {
				this.chart.setSize(this.getSize());
			}
		}
	},

	beforeDestroy: function() {
		window.removeEventListener('resize', this.setChartSize);
	}
}
</script>

<style scoped>
@import './../assets/uPlot.min.css';
</style>