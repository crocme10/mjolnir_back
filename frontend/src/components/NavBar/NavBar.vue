<template>
  <div class="bg-blue-800 flex flex-row justify-between py-0 px-8">
    <button v-for="route in navigationRouteOptions" v-bind:key="route.name" @click="goTo(route.path)"
      class="inline-block py-1 md:py-4 text-gray-500 hover:text-gray-100 mr-6">
      {{ route.name }}
    </button>
  </div>
</template>

<script>
import AppRoutes from '@/router'
export default {
  name: 'NavBar',
  data () {
    return {
      navigationRouteOptions: [AppRoutes.Home, AppRoutes.Dashboard]
    }
  },
  methods: {
    goBack () {
      this.$router.back()
    },
    goHome () {
      this.$router.push(AppRoutes.Home)
    },
    goTo (routeName) {
      this.$router.push(routeName)
    }
  }
}
</script>
