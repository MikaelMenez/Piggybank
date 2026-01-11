<script setup>
import { RouterView } from 'vue-router'
</script>

<template>
  <RouterView />
</template>

<style>
/* Reset Violento para garantir que nada fique centralizado à força */
html, body, #app {
  width: 100%;
  min-height: 100vh;
  margin: 0;
  padding: 0;
  display: block !important; /* Remove o display: flex que centraliza */
  background-color: #11034b86; /* Cor de fundo padrão */
}

/* Garante que o conteúdo dentro do App também possa crescer */
#app {
  max-width: 100% !important;
  text-align: left !important;
}
</style>