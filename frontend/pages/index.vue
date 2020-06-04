<template>
  <v-layout column justify-center align-center>
    <v-flex xs12 sm8 md6>
      <v-card>
        <v-card-title class="headline">Welcome to the IOTA Streams Explorer</v-card-title>
        <v-card-text>
          <p>IOTA Streams, a framework for cryptographic protocols called Applications. Replaces Masked Authenticated Messaging (MAM). Alpha version.</p>
          <p>
            For more information on IOTA Streams, check out the
            <a
              href="https://github.com/iotaledger/streams"
              target="_blank"
            >documentation</a>.
          </p>
          <p>
            If you have questions, please join the official
            <a
              href="https://discord.iota.org/"
              target="_blank"
              title="chat"
            >discord</a>.
          </p>
          <p>
            Find a bug? Report it on the github
            <a
              href="https://github.com/rust-iota/iota-streams-api/issues"
              target="_blank"
              title="contribute"
            >issue board</a>.
          </p>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn color="primary" nuxt to="/inspire">Continue</v-btn>
        </v-card-actions>
      </v-card>
      <v-card>
        <v-card-title class="headline">Explorer</v-card-title>
        <Input />
      </v-card>
    </v-flex>
  </v-layout>
</template>

<script>
import Input from '~/components/Input.vue'

export default {
  components: {
    Input
  }
}
</script>
