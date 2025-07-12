<template>
  <div class="app-container">
    <header>
      <h1>League All-In</h1>
    </header>
    <main>
      <Recorder />
    </main>
  </div>
</template>

<script setup>
import Recorder from './components/Recorder.vue'
</script>

<style>
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
    Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  background-color: #f9f9f9;
}

.app-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

header {
  background-color: #1a237e;
  color: white;
  padding: 1rem 2rem;
  text-align: center;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

h1 {
  margin: 0;
  font-size: 1.8rem;
}

main {
  flex: 1;
  padding: 2rem;
}
</style>
