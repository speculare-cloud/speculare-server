<template>
	<div class="home">
		<h1 class="text-6xl font-normal leading-normal mt-0 mb-2 text-gray-800">Home</h1>
		<h3 v-if="this.$store.state.hosts_loading">Loading</h3>
		<table class="table-fixed" v-if="!this.$store.state.hosts_loading">
      		<thead>
        		<tr>
					<th class="w-1/4" v-for="key in hosts_keys" v-bind:key="key">{{ key }}</th>
        		</tr>
      		</thead>
      		<tbody v-if="this.$store.state.hosts_values.length">
        		<tr v-for="item in this.$store.state.hosts_values" v-bind:key="item.uuid">
          			<td>{{ item.hostname }}</td>
					<td>{{ item.os }}</td>
					<td>{{ item.uptime }}</td>
					<td>
						<router-link :to="'/h/' + item.uuid" class="bg-gray-300 hover:bg-gray-400 text-gray-800 py-2 px-4 rounded inline-flex items-center">
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
			hosts_keys: ["hostname", "os", "uptime", ""]
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
				uptime: newValues[2],
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

	beforeDestroy: function() {
		console.log("[HOSTS] Closing the WebSocket connection");
		this.connection.close();
		this.connection = null;
	}
}
</script>