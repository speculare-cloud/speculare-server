<template>
	<div id="steppedChart" class="w-100" @mouseover="hover=true" @mouseleave="hover=false"></div>
</template>

<script>
import uPlot from 'uplot';

const { linear, spline, stepped, bars } = uPlot.paths;

const lineInterpolations = {
	linear:     0,
	smooth:     1,
	stepAfter:  2,
	stepBefore: 3,
};

const drawStyles = {
	line:      0,
	bars:      1,
	points:    2,
	barsLeft:  3,
	barsRight: 4,
};

// generate bar builder with 60% bar (40% gap) & 100px max bar width
const _bars60_100   = bars({size: [0.6, 100]});
const _bars100Left  = bars({size: [1], align:  1});
const _bars100Right = bars({size: [1], align: -1});
const _stepBefore   = stepped({align: -1});
const _stepAfter    = stepped({align:  1});
const _spline       = spline();
const _linear       = linear();

function paths(u, seriesIdx, idx0, idx1, extendGap, buildClip) {
	let s = u.series[seriesIdx];
	let style = s.drawStyle;
	let interp = s.lineInterpolation;

	let renderer = (
		style == drawStyles.line ? (
			interp == lineInterpolations.linear     ? _linear :
			interp == lineInterpolations.smooth     ? _spline :
			interp == lineInterpolations.stepAfter  ? _stepAfter :
			interp == lineInterpolations.stepBefore ? _stepBefore :
			null
		) :
		style == drawStyles.bars ? (
			_bars60_100
		) :
		style == drawStyles.barsLeft ? (
			_bars100Left
		) :
		style == drawStyles.barsRight ? (
			_bars100Right
		) :
		style == drawStyles.points ? (
			() => null
		) : () => null
	);

	return renderer(u, seriesIdx, idx0, idx1, extendGap, buildClip);
}

const palette = [
	'#7EB26D', // 0: pale green
	'#EAB839', // 1: mustard
	'#6ED0E0', // 2: light blue
	'#EF843C', // 3: orange
	'#E24D42', // 4: red
	'#1F78C1', // 5: ocean
	'#BA43A9', // 6: purple
	'#705DA0', // 7: violet
	'#508642', // 8: dark green
	'#CCA300', // 9: dark sand
];

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
			this.ctx = document.getElementById('steppedChart');
		}

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
					live: false,
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
						paths,
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
			this.chart = new uPlot(opts, data, this.ctx);
		},
		getSize: function() {
			return {
				width: window.innerWidth,
				height: 200,
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

<style>
.u-legend {
	text-align: left;
	padding-left: 50px;
}

.u-inline tr {
	margin-right: 8px;
}

.u-label {
	font-size: 10px;
}
</style>