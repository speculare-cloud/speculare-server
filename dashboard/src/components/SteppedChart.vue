<template>
	<div ref="uniqueName" class="w-100" @mouseover="hover=true" @mouseleave="hover=false"></div>
</template>

<script>
import uPlot from 'uplot';

const { stepped } = uPlot.paths;

export default {
	name: 'SteppedChart',
	props: {
		chartdata: {
			type: Array,
			default: null
    	},
  	},

	data () {
		return {
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
					this.chart.setScale('y', { min: 0, max: (max + (max / 10) + 10) });
				}
				this.chart.setData(newData);
			}
		}
	},

	mounted () {
		this.$nextTick(function() {
			window.addEventListener('resize', this.setChartSize);
		});
	},

	methods: {
		createChart: function(data) {
			this.max = Math.max.apply(null, data);
			let opts = {
				...this.getSize(),
				cursor: {
					points: {
						size: (u, seriesIdx) => u.series[seriesIdx].points.size * 1.1,
						width: (_u, _seriesIdx, size) => size / 4,
						fill: "#EF843C",
					}
				},
				legend: {
					live: true,
				},
				select: {
					show: false,
				},
				series: [
					{},
					{
						label: "Y",
						drawStyle: 0,
						lineInterpolation: 3,
						stroke: "#EF843C",
						fill: "#EF843C1A",
						paths: this.paths,
					},
				],
				scales: {
					x: {
						time: true,
					},
					y: {
						auto: false,
						range: [0, this.max + (this.max / 10)]
					},
				}
			};
			this.chart = new uPlot(opts, data, this.$refs.uniqueName);
		},
		getSize: function() {
			return {
				width: this.$refs.uniqueName.clientWidth,
				height: 200,
			}
		},
		setChartSize: function (_event) {
			if (this.chart) {
				this.chart.setSize(this.getSize());
			}
		},
		paths: function(u, seriesIdx, idx0, idx1, extendGap, buildClip) {
			let s = u.series[seriesIdx];
			let style = s.drawStyle;
			let interp = s.lineInterpolation;

			let renderer = (
				style == 0 ? (
					interp == 3 ? stepped({align: -1}) :
					null
				) : () => null
			);

			return renderer(u, seriesIdx, idx0, idx1, extendGap, buildClip);
		}
	},

	beforeDestroy: function() {
		window.removeEventListener('resize', this.setChartSize);
	}
}
</script>
