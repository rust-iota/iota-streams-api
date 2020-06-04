<template>
  <v-form ref="form" v-model="valid" lazy-validation>
    <v-text-field v-model="address" :counter="81" :rules="addressRules" label="Address" required></v-text-field>

    <v-text-field v-model="tag" :rules="tagRules" label="Tag" required></v-text-field>

    <v-text-field v-model="node" :rules="nodeRules" label="IOTA Node" required></v-text-field>

    <v-btn :disabled="!valid" color="success" @click="validate">Search</v-btn>

    <v-btn color="error" @click="reset">Reset Form</v-btn>
  </v-form>
</template>

<script>
export default {
  data: () => ({
    valid: true,
    address: '',
    addressRules: [
      (v) => !!v || 'Address is required',
      (v) =>
        (v && v.length <= 81) ||
        'Address must be less or equal than 81 characters'
    ],
    tag: '',
    tagRules: [(v) => !!v || 'tag is required'],
    node: '',
    nodeRules: [(v) => !!v || 'node is required']
  }),

  methods: {
    validate() {
      if (this.$refs.form.validate()) {
        let data = { address: this.address, tag: this.tag, node: this.node }
        this.snackbar = true
        console.log('success', data)
      }
    },
    reset() {
      this.$refs.form.reset()
    },
    resetValidation() {
      this.$refs.form.resetValidation()
    }
  }
}
</script>
<style>
</style>