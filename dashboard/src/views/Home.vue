<template>
	<div class="home">
		<h1 class="text-6xl font-normal leading-normal mt-0 mb-2 text-gray-800">Home</h1>
		
		<h3 v-if="this.$store.state.hosts_loading">Loading</h3>
		
		<table class="overflow-x-auto w-full bg-white" v-if="!this.$store.state.hosts_loading">
	        <thead class="bg-blue-100 border-b border-gray-300">
	            <tr>
					<th class="p-4 text-left text-sm font-medium text-gray-500 w-1/4" v-for="key in hosts_keys" v-bind:key="key">{{ key }}</th>
	            </tr>
	        </thead>
	        <tbody class="text-gray-600 text-sm divide-y divide-gray-300">
				<tr class="bg-white font-medium text-sm divide-y divide-gray-200" v-for="item in this.$store.state.hosts_values" v-bind:key="item.uuid">
          			<td class="p-4 whitespace-nowrap">{{ item.hostname }}</td>
					<td class="p-4 whitespace-nowrap">{{ item.os }}</td>
					<td class="p-4 whitespace-nowrap">{{ item.uptime }}</td>
					<td class="p-4 whitespace-nowrap">
						<router-link :to="'/h/' + item.uuid" class="bg-indigo-100 text-indigo-800 text-xs font-semibold px-4 py-2 rounded-md border-0">
							Details
						</router-link>
					</td>
        		</tr>
	        </tbody>
	    </table>
	</div>
</template>

<script>
export default {
	name: 'Home',
	connection: null,

	data() {
		return {
			hosts_keys: ["Hostname", "OS", "Uptime", ""]
		};
	},

	created: function() {		
		let vm = this;
		let store = vm.$store;
		
		if (vm.connection == null) {
			console.log("[HOSTS] Starting connection to WebSocket Server");
			vm.connection = new WebSocket("wss://cdc.speculare.cloud:9641/ws?change_table=hosts");
		}

		vm.connection.onmessage = function(event) {
			let json = JSON.parse(event.data);
			let newValues = json["columnvalues"];
			// Construct the newObj from the values (it's the hosts table)
			let newObj = {
				os: newValues[0],
				hostname: newValues[1],
				uptime: vm.secondsToDhms(newValues[2]),
				uuid: newValues[3],
				created_at: newValues[4],
			};
			// Find at which position the UUID is currently present
			let objIndex = store.state.hosts_values.findIndex((obj => obj.uuid == newObj.uuid));
			// If not found, we push it
			if (objIndex == -1) {
				store.commit({type: 'pushHosts', newdata: newObj});
			// Else we update the value
			} else {
				store.commit({type: 'updateHosts', index: objIndex, newdata: newObj});
			}
			if (store.state.hosts_loading) {
				store.state.hosts_loading = false;
			}
		}
	},

	methods: {
		// TODO - Rework
		secondsToDhms: function(s) {
			const d = Math.floor(s / (3600 * 24));
			s  -= d * 3600 * 24;
			const h = Math.floor(s / 3600);
			s  -= h * 3600;
			const m = Math.floor(s / 60);
			s  -= m * 60;
			const tmp = [];
			
			(d) && tmp.push(d + 'd');
			(d || h) && tmp.push(h + 'h');
			(d || h || m) && tmp.push(m + 'm');
			tmp.push(s + 's');
			return tmp.join(' ');
		}
	},

	beforeDestroy: function() {
		console.log("[HOSTS] Closing the WebSocket connection");
		this.connection.close();
		this.connection = null;
	}
}
</script>