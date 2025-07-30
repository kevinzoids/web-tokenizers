<script setup>
import { ref } from 'vue';
import {encodeAsync} from './utils/encoder.js'

const text = ref("test");
const tokens = ref();

async function submit() {
  let start = performance.now();
  let result = await encodeAsync(text.value);
  tokens.value = result.tokens;
  console.log(result.inputIds);
  console.log(result.attentionMask);
  console.log(performance.now() - start);
}
</script>

<template>
  <div>
    <input v-model="text"></input>
    <button v-on:click="submit" >submit</button>
    <div>
      <pre>
        {{ tokens }}
      </pre>
    </div>
  </div>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
