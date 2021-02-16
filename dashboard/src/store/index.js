import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
    state: {
        hosts_loading: true,
        hosts_values: [],
    },
    mutations: {
        updateHosts(state, payload) {
            Vue.set(state.hosts_values, payload.index, payload.newdata);
        },
        pushHosts(state, payload) {
            state.hosts_values.push(payload.newdata);
        }
    },
    actions: {},
    modules: {}
})